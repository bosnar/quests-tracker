use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};

use crate::{
    application::usecases::guild_commanders::GuildCommandersUseCase,
    domain::{
        repositories::guild_commanders::GuildCommanderRepository,
        value_objects::guild_commander_model::RegisterGuildCommanderModel,
    },
    infrastructure::postgres::{
        postgres_connection::PgPoolSquad, repositories::guild_commanders::GuildCommanderPostgres,
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let guild_commander_repository = GuildCommanderPostgres::new(Arc::clone(&db_pool));
    let guild_commander_usecase = GuildCommandersUseCase::new(Arc::new(guild_commander_repository));

    Router::new()
        .route("/", post(register))
        .with_state(Arc::new(guild_commander_usecase))
}

pub async fn register<T>(
    State(guild_commander_usecase): State<Arc<GuildCommandersUseCase<T>>>,
    Json(register_guild_commander_model): Json<RegisterGuildCommanderModel>,
) -> impl IntoResponse
where
    T: GuildCommanderRepository + Send + Sync,
{
    match guild_commander_usecase
        .register(register_guild_commander_model)
        .await
    {
        Ok(guild_commander_id) => (
            StatusCode::CREATED,
            format!("Register success:{}", guild_commander_id),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
