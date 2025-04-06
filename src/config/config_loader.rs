// use crate::config::config_model::DotEnvyConfig;
use anyhow::{Ok, Result};

// อยู่ภายใต้ config เลยใช้ super
use super::{
    config_model::{AdventurerSecret, Database, DotEnvyConfig, GuildCommanderSecret, Server},
    stage::Stage,
};

pub fn load() -> Result<DotEnvyConfig> {
    dotenvy::dotenv().ok();

    let server = Server {
        port: std::env::var("SERVER_PORT")?.parse()?,
        body_limit: std::env::var("SERVER_BODY_LIMIT")?.parse()?,
        timeout: std::env::var("SERVER_TIMEOUT")?.parse()?,
    };

    let database = Database {
        url: std::env::var("DATABASE_URL")?,
    };

    Ok(DotEnvyConfig { server, database })
}

pub fn get_stage() -> Stage {
    dotenvy::dotenv().ok();

    let stage_str = std::env::var("STAGE").unwrap_or_default();
    Stage::try_from(&stage_str).unwrap_or_default()
}

pub fn get_adventurer_secret() -> Result<AdventurerSecret> {
    dotenvy::dotenv().ok();

    let secret = std::env::var("JWT_ADVENTURER_SECRET")?;
    let refresh_secret = std::env::var("JWT_ADVENTURER_REFRESH_SECRET")?;

    Ok(AdventurerSecret {
        secret,
        refresh_secret,
    })
}

pub fn get_guild_commander_secret() -> Result<GuildCommanderSecret> {
    dotenvy::dotenv().ok();

    let secret = std::env::var("JWT_GUILD_COMMANDER_SECRET")?;
    let refresh_secret = std::env::var("JWT_GUILD_COMMANDER_REFRESH_SECRET")?;

    Ok(GuildCommanderSecret {
        secret,
        refresh_secret,
    })
}
