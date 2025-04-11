use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::{
    config::config_loader::{get_adventurer_secret, get_guild_commander_secret},
    infrastructure::jwt_authentication,
};

pub async fn adventurers_authorization(
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if let Some(cookie_header) = req.headers().get(header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            let access_token = get_cookie_value(cookie_str, "act");
            if let Some(token) = access_token {
                if let Ok(secret) = get_adventurer_secret() {
                    if let Ok(claims) = jwt_authentication::verify_token(secret.secret, token) {
                        if let Ok(adventurer_id) = claims.sub.parse::<i32>() {
                            req.extensions_mut().insert(adventurer_id);
                            return Ok(next.run(req).await);
                        }
                    }
                }
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

pub async fn guild_commanders_authorization(
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if let Some(cookie_header) = req.headers().get(header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            let access_token = get_cookie_value(cookie_str, "act");
            if let Some(token) = access_token {
                if let Ok(secret) = get_guild_commander_secret() {
                    if let Ok(claims) = jwt_authentication::verify_token(secret.secret, token) {
                        if let Ok(guild_commander_id) = claims.sub.parse::<i32>() {
                            req.extensions_mut().insert(guild_commander_id);
                            return Ok(next.run(req).await);
                        }
                    }
                }
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

fn get_cookie_value(cookie_header: &str, key: &str) -> Option<String> {
    cookie_header.split("; ").find_map(|cookie| {
        let mut parts = cookie.splitn(2, "=");
        let cookie_key = parts.next()?.trim();
        let cookie_value = parts.next()?.trim();

        if cookie_key == key {
            Some(cookie_value.to_string())
        } else {
            None
        }
    })
}
