use crate::{
    config::config_model::DotEnvyConfig, infrastructure::postgres::postgres_connection::PgPoolSquad,
};
use anyhow::{Ok, Result};
use axum::{
    http::{HeaderName, Method},
    routing::get,
    Router,
};
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    limit::RequestBodyLimitLayer,
    timeout::TimeoutLayer,
};
use tracing::info;

use super::default_routers;

pub async fn start(config: Arc<DotEnvyConfig>, postgres_pool: Arc<PgPoolSquad>) -> Result<()> {
    let app = Router::new()
        .fallback(default_routers::not_found)
        .route("/health-check", get(default_routers::health_check))
        .with_state(postgres_pool)
        .layer(TimeoutLayer::new(Duration::from_secs(
            config.server.timeout,
        )))
        .layer(RequestBodyLimitLayer::new(
            (config.server.body_limit * 1024 * 1024).try_into()?,
        ))
        .layer(
            CorsLayer::new()
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::PATCH,
                    Method::DELETE,
                ])
                .allow_origin(Any)
                .allow_headers([
                    HeaderName::from_static("content-type"),
                    HeaderName::from_static("authorization"),
                    HeaderName::from_static("accept"),
                    HeaderName::from_static("origin"),
                ]),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));

    let listener = TcpListener::bind(addr).await?;

    info!("Listening on: {}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    let terminate = std::future::pending();

    tokio::pin!(ctrl_c, terminate);

    tokio::select! {
        _ = ctrl_c => info!("received Ctrl+C signal, shutting down"),
        _ = terminate => info!("received terminate signal, shutting down"),
    }

    info!("graceful shutdown");
}
