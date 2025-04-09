use std::sync::Arc;

use axum::{response::IntoResponse, routing::post, Router};

use crate::infrastructure::postgres::postgres_connection::PgPoolSquad;

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    Router::new().route("/login", post(login))
}

pub async fn login() -> impl IntoResponse {
    unimplemented!()
}
