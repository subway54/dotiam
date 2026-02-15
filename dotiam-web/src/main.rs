use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Form, Router,
};
use dotiam_app::Repository;
use dotiam_core::{GameState, WorldTemplate, TileType, Position, MAP_SIZE, Tile};
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

#[derive(Deserialize)]
struct EditTileInput {
    tile_type: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    run_id: String,
    state: GameState,
    edit_mode: bool,
}

#[derive(Template)]
#[template(path = "partial_game.html")]
struct GamePartialTemplate {
    run_id: String,
    state: GameState,
    edit_mode: bool,
}

#[derive(Deserialize)]
struct GameQuery {
    #[serde(default)]
    edit: bool,
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

struct EditorTile {
    char: char,
    x: i32,
    y: i32,
    is_active: bool,
    exists: bool,
}

#[derive(Template)]
#[template(path = "editor.html")]
struct EditorTemplate {
    run_id: String,
    grid: Vec<Vec<EditorTile>>,
    pos: Position,
    tile: Option<dotiam_core::Tile>,
}

#[derive(Template)]
#[template(path = "partial_editor_details.html")]
struct EditorDetailTemplate {
    run_id: String,
    pos: Position,
    tile: Option<dotiam_core::Tile>,
}

#[derive(Deserialize)]
struct UpdateDescriptionInput {
    description: String,
    tile_type: String,
}

#[derive(Deserialize)]
struct EditorQuery {
    x: Option<i32>,
    y: Option<i32>,
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
        .route("/game/{id}/edit_tile", post(edit_tile_handler))
        .route("/game/{id}/export", get(export_handler))
        .route("/game/{id}/editor", get(editor_handler))
        .route("/game/{id}/editor/tile/{x}/{y}", get(editor_tile_handler))
        .route("/game/{id}/editor/tile/{x}/{y}/description", post(update_description_handler))
        .route("/game/{id}/editor/tile/{x}/{y}/quick_update", post(quick_update_handler))
        .route("/game/{id}/editor/undo", post(editor_undo_handler))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn root_handler(
    Query(query): Query<GameQuery>,
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
        edit_mode: query.edit,
    };
    Ok(Html(template.render().map_err(|e| AppError(e.to_string()))?))
}

async fn game_handler(
    Path(id): Path<String>,
    Query(query): Query<GameQuery>,
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, AppError> {
    let game_state = state.repo.load_run(&id).await?;

    let template = IndexTemplate {
        run_id: id,
        state: game_state,
        edit_mode: query.edit,
    };
    Ok(Html(template.render().map_err(|e| AppError(e.to_string()))?))
}

async fn command_handler(
    Path(id): Path<String>,
    Query(query): Query<GameQuery>,
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
        edit_mode: query.edit,
    };
    Ok(Html(template.render().map_err(|e| AppError(e.to_string()))?))
}

async fn edit_tile_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
    Form(input): Form<EditTileInput>,
) -> Result<Html<String>, AppError> {
    let mut game_state = state.repo.load_run(&id).await?;

    let tile_type = match input.tile_type.as_str() {
        "Forest" => TileType::Forest,
        "Ruins" => TileType::Ruins,
        "Cave" => TileType::Cave,
        "Plains" => TileType::Plains,
        _ => return Err(AppError("Invalid tile type".to_string())),
    };

    if let Some(tile) = game_state.world.tiles.get_mut(&game_state.player.pos) {
        tile.tile_type = tile_type;
    } else {
        game_state.world.tiles.insert(
            game_state.player.pos.clone(),
            dotiam_core::Tile {
                tile_type,
                discovered: true,
                description: None,
            },
        );
    }

    state.repo.save_run(&id, &game_state).await?;

    let template = GamePartialTemplate {
        run_id: id,
        state: game_state,
        edit_mode: true,
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
    Query(query): Query<SuggestionQuery>,
) -> Result<Html<String>, AppError> {
    let input = query.command.trim().to_lowercase();
    let all_commands = vec!["north", "south", "east", "west", "explore", "inventory"];
    
    let suggestions: Vec<String> = if input.is_empty() {
        vec![]
    } else {
        all_commands.into_iter()
            .filter(|c| c.starts_with(&input))
            .map(|c| c.to_string())
            .collect()
    };

    let template = SuggestionsTemplate { suggestions };
    Ok(Html(template.render().map_err(|e| AppError(e.to_string()))?))
}

async fn editor_handler(
    Path(id): Path<String>,
    Query(query): Query<EditorQuery>,
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, AppError> {
    let game_state = state.repo.load_run(&id).await?;
    let mut x = query.x.unwrap_or(game_state.player.pos.x);
    let mut y = query.y.unwrap_or(game_state.player.pos.y);

    // Clamp values to MAP_SIZE
    x = x.clamp(0, MAP_SIZE - 1);
    y = y.clamp(0, MAP_SIZE - 1);

    let cursor_pos = Position { x, y };

    // Viewport size
    let vp_size = 10; // 10 cells in each direction -> 21x21 total
    let min_x = (x - vp_size).max(0);
    let max_x = (x + vp_size).min(MAP_SIZE - 1);
    let min_y = (y - vp_size).max(0);
    let max_y = (y + vp_size).min(MAP_SIZE - 1);

    let mut grid = Vec::new();
    for curr_y in (min_y..=max_y).rev() {
        let mut row = Vec::new();
        for curr_x in min_x..=max_x {
            let pos = Position { x: curr_x, y: curr_y };
            let tile = game_state.world.tiles.get(&pos);
            let char = match tile {
                Some(t) => match t.tile_type {
                    dotiam_core::TileType::HorizontalPath => '-',
                    dotiam_core::TileType::VerticalPath => '|',
                    dotiam_core::TileType::Crossroad => '+',
                    dotiam_core::TileType::Plains | dotiam_core::TileType::Forest | dotiam_core::TileType::Ruins | dotiam_core::TileType::Cave => '+',
                },
                None => ' ',
            };
            row.push(EditorTile {
                char,
                x: curr_x,
                y: curr_y,
                is_active: curr_x == cursor_pos.x && curr_y == cursor_pos.y,
                exists: tile.is_some(),
            });
        }
        grid.push(row);
    }

    let tile = game_state.world.tiles.get(&cursor_pos).cloned();

    let template = EditorTemplate {
        run_id: id,
        grid,
        pos: cursor_pos,
        tile,
    };
    Ok(Html(template.render().map_err(|e| AppError(e.to_string()))?))
}

async fn editor_tile_handler(
    Path((id, x, y)): Path<(String, i32, i32)>,
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, AppError> {
    let game_state = state.repo.load_run(&id).await?;
    let pos = Position { x, y };
    let tile = game_state.world.tiles.get(&pos).cloned();

    let template = EditorDetailTemplate {
        run_id: id,
        pos,
        tile,
    };
    Ok(Html(template.render().map_err(|e| AppError(e.to_string()))?))
}

async fn update_description_handler(
    Path((id, x, y)): Path<(String, i32, i32)>,
    State(state): State<Arc<AppState>>,
    Form(input): Form<UpdateDescriptionInput>,
) -> Result<Html<String>, AppError> {
    let mut game_state = state.repo.load_run(&id).await?;
    let pos = Position { x, y };

    let tile_type = match input.tile_type.as_str() {
        "Forest" => TileType::Forest,
        "Ruins" => TileType::Ruins,
        "Cave" => TileType::Cave,
        "Plains" => TileType::Plains,
        "HorizontalPath" => TileType::HorizontalPath,
        "VerticalPath" => TileType::VerticalPath,
        "Crossroad" => TileType::Crossroad,
        _ => return Err(AppError("Invalid tile type".to_string())),
    };
    
    let description = if input.description.trim().is_empty() {
        None
    } else {
        Some(input.description)
    };

    if let Some(tile) = game_state.world.tiles.get_mut(&pos) {
        tile.tile_type = tile_type;
        tile.description = description;
    } else {
        game_state.world.tiles.insert(pos.clone(), dotiam_core::Tile {
            tile_type,
            discovered: true,
            description,
        });
    }

    state.repo.save_run(&id, &game_state).await?;

    let tile = game_state.world.tiles.get(&pos).cloned();
    let template = EditorDetailTemplate {
        run_id: id,
        pos,
        tile,
    };
    Ok(Html(template.render().map_err(|e| AppError(e.to_string()))?))
}

async fn quick_update_handler(
    Path((id, x, y)): Path<(String, i32, i32)>,
    State(state): State<Arc<AppState>>,
    Form(input): Form<EditTileInput>,
) -> Result<StatusCode, AppError> {
    let mut game_state = state.repo.load_run(&id).await?;
    let pos = Position { x, y };

    // Uložit současný svět do historie před změnou
    game_state.history.push(game_state.world.clone());
    // Omezit historii na 50 kroků
    if game_state.history.len() > 50 {
        game_state.history.remove(0);
    }

    if input.tile_type == "Empty" {
        game_state.world.tiles.remove(&pos);
    } else {
        let tile_type = match input.tile_type.as_str() {
            "Forest" => TileType::Forest,
            "Ruins" => TileType::Ruins,
            "Cave" => TileType::Cave,
            "Plains" => TileType::Plains,
            "HorizontalPath" => TileType::HorizontalPath,
            "VerticalPath" => TileType::VerticalPath,
            "Crossroad" => TileType::Crossroad,
            _ => return Err(AppError("Invalid tile type".to_string())),
        };

        if let Some(tile) = game_state.world.tiles.get_mut(&pos) {
            tile.tile_type = tile_type;
        } else {
            game_state.world.tiles.insert(pos, Tile {
                tile_type,
                discovered: true,
                description: None,
            });
        }
    }

    state.repo.save_run(&id, &game_state).await?;
    Ok(StatusCode::OK)
}

async fn editor_undo_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<StatusCode, AppError> {
    let mut game_state = state.repo.load_run(&id).await?;
    
    if let Some(previous_world) = game_state.history.pop() {
        game_state.world = previous_world;
        state.repo.save_run(&id, &game_state).await?;
        Ok(StatusCode::OK)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}