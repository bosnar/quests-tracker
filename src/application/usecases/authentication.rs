use anyhow::{Ok, Result};
use chrono::{Duration, Utc};
use std::sync::Arc;

use crate::{
    config::config_loader::{get_adventurer_secret, get_guild_commander_secret},
    domain::repositories::{
        adventurers::AdventurerRepository, guild_commanders::GuildCommanderRepository,
    },
    infrastructure::{
        argon2_hashing,
        jwt_authentication::{
            self,
            authentication_model::LoginModel,
            jwt_model::{Claims, Passport, Roles},
        },
    },
};

pub struct AuthenticationUseCase<T1, T2>
where
    T1: AdventurerRepository + Send + Sync,
    T2: GuildCommanderRepository + Send + Sync,
{
    adventurer_repository: Arc<T1>,
    guild_commander_repository: Arc<T2>,
}

impl<T1, T2> AuthenticationUseCase<T1, T2>
where
    T1: AdventurerRepository + Send + Sync,
    T2: GuildCommanderRepository + Send + Sync,
{
    pub fn new(adventurer_repository: Arc<T1>, guild_commander_repository: Arc<T2>) -> Self {
        Self {
            adventurer_repository,
            guild_commander_repository,
        }
    }

    pub async fn adventurer_login(&self, login_model: LoginModel) -> Result<Passport> {
        let secret_env = get_adventurer_secret()?;

        let adventurer = self
            .adventurer_repository
            .find_by_username(login_model.username.clone())
            .await?;

        if !argon2_hashing::verify(login_model.password, adventurer.password)? {
            return Err(anyhow::anyhow!("Invalid password"));
        }

        let access_token_claims = Claims {
            sub: adventurer.id.to_string(),
            role: Roles::Adventurer,
            exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let refresh_token_claims = Claims {
            sub: adventurer.id.to_string(),
            role: Roles::Adventurer,
            exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let access_token =
            jwt_authentication::generate_token(secret_env.secret, &access_token_claims)?;

        let refresh_token =
            jwt_authentication::generate_token(secret_env.refresh_secret, &refresh_token_claims)?;

        Ok(Passport {
            access_token,
            refresh_token,
        })
    }

    pub async fn adventurer_refresh_token(&self, refresh_token: String) -> Result<Passport> {
        let secret_env = get_adventurer_secret()?;

        let claims =
            jwt_authentication::verify_token(secret_env.refresh_secret.clone(), refresh_token)?;

        let access_token_claims = Claims {
            sub: claims.sub.clone(),
            role: Roles::Adventurer,
            exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let refresh_token_claims = Claims {
            sub: claims.sub,
            role: Roles::Adventurer,
            exp: claims.exp,
            iat: Utc::now().timestamp() as usize,
        };

        let access_token =
            jwt_authentication::generate_token(secret_env.secret, &access_token_claims)?;

        let refresh_token =
            jwt_authentication::generate_token(secret_env.refresh_secret, &refresh_token_claims)?;

        Ok(Passport {
            access_token,
            refresh_token,
        })
    }

    pub async fn guild_commander_login(&self, login_model: LoginModel) -> Result<Passport> {
        let secret_env = get_guild_commander_secret()?;

        let guild_commander = self
            .guild_commander_repository
            .find_by_username(login_model.username.clone())
            .await?;

        if !argon2_hashing::verify(login_model.password, guild_commander.password)? {
            return Err(anyhow::anyhow!("Invalid password"));
        }

        let access_token_claims = Claims {
            sub: guild_commander.id.to_string(),
            role: Roles::GuildCommander,
            exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let refresh_token_claims = Claims {
            sub: guild_commander.id.to_string(),
            role: Roles::GuildCommander,
            exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let access_token =
            jwt_authentication::generate_token(secret_env.secret, &access_token_claims)?;

        let refresh_token =
            jwt_authentication::generate_token(secret_env.refresh_secret, &refresh_token_claims)?;

        Ok(Passport {
            access_token,
            refresh_token,
        })
    }

    pub async fn guild_commander_refresh_token(&self, refresh_token: String) -> Result<Passport> {
        let secret_env = get_guild_commander_secret()?;

        let claims =
            jwt_authentication::verify_token(secret_env.refresh_secret.clone(), refresh_token)?;

        let access_token_claims = Claims {
            sub: claims.sub.clone(),
            role: Roles::GuildCommander,
            exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let refresh_token_claims = Claims {
            sub: claims.sub,
            role: Roles::GuildCommander,
            exp: claims.exp,
            iat: Utc::now().timestamp() as usize,
        };

        let access_token =
            jwt_authentication::generate_token(secret_env.secret, &access_token_claims)?;

        let refresh_token =
            jwt_authentication::generate_token(secret_env.refresh_secret, &refresh_token_claims)?;

        Ok(Passport {
            access_token,
            refresh_token,
        })
    }
}
