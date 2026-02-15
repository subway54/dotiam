use dotiam_core::{GameState, WorldTemplate};
use sqlx::sqlite::SqlitePool;
use uuid::Uuid;

pub struct Repository {
    pool: SqlitePool,
}

impl Repository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_run(&self, player_name: String) -> Result<String, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let state = GameState::new(player_name.clone());
        self.insert_run(&id, &player_name, &state).await?;
        Ok(id)
    }

    pub async fn create_run_from_template(&self, player_name: String, template: WorldTemplate) -> Result<String, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let world = template.to_world();
        let state = GameState::new_with_world(player_name.clone(), world);
        self.insert_run(&id, &player_name, &state).await?;
        Ok(id)
    }

    async fn insert_run(&self, id: &str, player_name: &str, state: &GameState) -> Result<(), sqlx::Error> {
        let state_json = serde_json::to_string(state).unwrap();

        sqlx::query(
            "INSERT INTO game_runs (id, player_name, state_json) VALUES (?, ?, ?)"
        )
        .bind(id)
        .bind(player_name)
        .bind(&state_json)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn load_run(&self, id: &str) -> Result<GameState, sqlx::Error> {
        let row: (String,) = sqlx::query_as(
            "SELECT state_json FROM game_runs WHERE id = ?"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        let state: GameState = serde_json::from_str(&row.0).unwrap();
        Ok(state)
    }

    pub async fn save_run(&self, id: &str, state: &GameState) -> Result<(), sqlx::Error> {
        let state_json = serde_json::to_string(state).unwrap();
        let turn = state.turn as i64;

        sqlx::query(
            "UPDATE game_runs SET state_json = ?, turn = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?"
        )
        .bind(&state_json)
        .bind(turn)
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
