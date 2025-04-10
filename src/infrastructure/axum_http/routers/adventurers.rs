use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};

use crate::{
    application::usecases::adventurers::AdventurersUseCase,
    domain::{
        repositories::adventurers::AdventurerRepository,
        value_objects::adventurer_model::RegisterAdventurerModel,
    },
    infrastructure::postgres::{
        postgres_connection::PgPoolSquad, repositories::adventurers::AdventurerPostgres,
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let adventurer_repository = AdventurerPostgres::new(db_pool);
    let adventurer_usecase = AdventurersUseCase::new(Arc::new(adventurer_repository));

    Router::new()
        .route("/", post(register))
        .with_state(Arc::new(adventurer_usecase))
}

pub async fn register<T>(
    State(adventurer_usecase): State<Arc<AdventurersUseCase<T>>>, // เอาของจากด้านนอกมาใช้งานใน handler
    Json(register_adventurer_model): Json<RegisterAdventurerModel>, // รับ body จาก client
) -> impl IntoResponse
where
    T: AdventurerRepository + Send + Sync,
{
    match adventurer_usecase.register(register_adventurer_model).await {
        Ok(adventurer_id) => (
            StatusCode::CREATED,
            format!("Register success:{}", adventurer_id),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
