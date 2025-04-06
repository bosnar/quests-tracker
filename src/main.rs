use quests_tracker::{config::config_loader, infrastructure::postgres::postgres_connection};
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let dotenv_env = match config_loader::load() {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to load config: {}", e);
            std::process::exit(1);
        }
    };

    info!("Server is running on port {}", dotenv_env.server.port);

    let postgres_pool = match postgres_connection::establish_connection(&dotenv_env.database.url) {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to establish connection: {}", e);
            std::process::exit(1);
        }
    };
}
