use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Form, Router,
};
use dotiam_app::Repository;
use dotiam_core::{GameState, WorldTemplate};
use serde::Deserialize;
use sqlx::sqlite::SqlitePool;
use std::sync::Arc;
use askama::Template;
use std::fs;
use std::path::Path as StdPath;

struct AppState {
    repo: Repository,
}

#[derive(Deserialize)]
struct CommandInput {
    command: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    run_id: String,
    state: GameState,
}

#[derive(Template)]
#[template(path = "partial_game.html")]
struct GamePartialTemplate {
    run_id: String,
    state: GameState,
}

#[derive(Deserialize)]
struct SuggestionQuery {
    command: String,
}

#[derive(Template)]
#[template(path = "suggestions.html")]
struct SuggestionsTemplate {
    suggestions: Vec<String>,
}

struct AppError(String);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0).into_response()
    }
}

impl<E> From<E> for AppError
where
    E: std::error::Error,
{
    fn from(err: E) -> Self {
        Self(err.to_string())
    }
}

#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    
    // Run migrations
    sqlx::migrate!("../dotiam-app/migrations")
        .run(&pool)
        .await
        .unwrap();

    let repo = Repository::new(pool);
    
    let app_state = Arc::new(AppState { repo });

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/game/{id}", get(game_handler))
        .route("/game/{id}/command", post(command_handler))
        .route("/game/{id}/suggest", get(suggest_handler))
        .route("/game/{id}/export", get(export_handler))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn root_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, AppError> {
    let run_id = if StdPath::new("world.yaml").exists() {
        let content = fs::read_to_string("world.yaml").map_err(|e| AppError(e.to_string()))?;
        let template = WorldTemplate::from_yaml(&content).map_err(|e| AppError(e.to_string()))?;
        state
            .repo
            .create_run_from_template("Adventurer".to_string(), template)
            .await?
    } else {
        state.repo.create_run("Adventurer".to_string()).await?
    };

    let game_state = state.repo.load_run(&run_id).await?;
    let template = IndexTemplate {
        run_id,
        state: game_state,
    };
    Ok(Html(template.render().map_err(|e| AppError(e.to_string()))?))
}

async fn game_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, AppError> {
    let game_state = state.repo.load_run(&id).await?;

    let template = IndexTemplate {
        run_id: id,
        state: game_state,
    };
    Ok(Html(template.render().map_err(|e| AppError(e.to_string()))?))
}

async fn command_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
    Form(input): Form<CommandInput>,
) -> Result<Html<String>, AppError> {
    let mut game_state = state.repo.load_run(&id).await?;

    let action = game_state.parse_command(&input.command);
    game_state.apply_action(action);
    state.repo.save_run(&id, &game_state).await?;

    let template = GamePartialTemplate {
        run_id: id,
        state: game_state,
    };
    Ok(Html(template.render().map_err(|e| AppError(e.to_string()))?))
}

async fn export_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Response, AppError> {
    let game_state = state.repo.load_run(&id).await?;
    let template = WorldTemplate::from_world(&game_state.world);
    let yaml = template.to_yaml();

    Ok(Response::builder()
        .header("Content-Type", "text/yaml")
        .header("Content-Disposition", "attachment; filename=\"world.yaml\"")
        .body(axum::body::Body::from(yaml))
        .unwrap())
}

async fn suggest_handler(
    Path(id): Path<String>,
    Query(query): Query<SuggestionQuery>,
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, AppError> {
    let input = query.command.trim().to_lowercase();
    let game_state = state.repo.load_run(&id).await?;
    
    let mut suggestions = vec![
        "help".to_string(),
        "look".to_string(),
        "inventory".to_string(),
        "explore".to_string(),
        "pickup".to_string(),
        "drop".to_string(),
        "use".to_string(),
        "combine".to_string()
    ];
    
    if let Some(node) = game_state.world.nodes.get(&game_state.player.current_node) {
        for edge in &node.edges {
            if game_state.can_traverse(edge) {
                suggestions.push(format!("go {}", edge.label));
                suggestions.push(format!("go {}", edge.target_id));
            }
        }
        for item_id in &node.items {
            suggestions.push(format!("pickup {}", item_id));
            suggestions.push(format!("explore {}", item_id));
        }
    }

    for item_id in &game_state.player.inventory {
        suggestions.push(format!("drop {}", item_id));
        suggestions.push(format!("use {}", item_id));
        suggestions.push(format!("explore {}", item_id));
        suggestions.push(format!("combine {}", item_id));
    }

    let filtered: Vec<String> = if input.is_empty() {
        vec![]
    } else {
        suggestions.into_iter()
            .filter(|c| c.to_lowercase().starts_with(&input))
            .collect()
    };

    let template = SuggestionsTemplate { suggestions: filtered };
    Ok(Html(template.render().map_err(|e| AppError(e.to_string()))?))
}