// use crate::config::config_model::DotEnvyConfig;
use anyhow::Result;

// อยู่ภายใต้ config เลยใช้ super
use super::config_model::DotEnvyConfig;

pub fn load() -> Result<DotEnvyConfig> {
    dotenvy::dotenv.ok();

    let server = Server {
        port: std::env::var("SERVER_PORT")?.parse()?,
        body_limit: std::env::var("SERVER_BODY_LIMIT")?.parse()?,
        timeout: std::env::var("SERVER_TIMEOUT")?.parse()?,
    };
}
