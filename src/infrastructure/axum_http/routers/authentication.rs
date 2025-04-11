use std::sync::Arc;

use axum::{
    extract::State,
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use cookie::time::Duration;

use crate::{
    application::usecases::authentication::AuthenticationUseCase,
    config::{config_loader::get_stage, stage::Stage},
    domain::repositories::{
        adventurers::AdventurerRepository, guild_commanders::GuildCommanderRepository,
    },
    infrastructure::{
        jwt_authentication::authentication_model::LoginModel,
        postgres::{
            postgres_connection::PgPoolSquad,
            repositories::{
                adventurers::AdventurerPostgres, guild_commanders::GuildCommanderPostgres,
            },
        },
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let adventurer_repository = Arc::new(AdventurerPostgres::new(Arc::clone(&db_pool)));
    let guild_commander_repository = Arc::new(GuildCommanderPostgres::new(Arc::clone(&db_pool)));

    let authentication_usecase =
        AuthenticationUseCase::new(adventurer_repository, guild_commander_repository);

    Router::new()
        .route("/adventurers/login", post(adventurer_login))
        .route("/guild-commanders/login", post(guild_commander_login))
        .route("/adventurers/refresh-token", post(adventurer_refresh_token))
        .route(
            "/guild-commanders/refresh-token",
            post(guild_commander_refresh_token),
        )
        .with_state(Arc::new(authentication_usecase))
}

pub async fn adventurer_login<T1, T2>(
    State(authentication_usecase): State<Arc<AuthenticationUseCase<T1, T2>>>,
    Json(login_model): Json<LoginModel>,
) -> impl IntoResponse
where
    T1: AdventurerRepository + Send + Sync,
    T2: GuildCommanderRepository + Send + Sync,
{
    match authentication_usecase.adventurer_login(login_model).await {
        Ok(passport) => {
            let mut access_token_cookie = Cookie::build(("act", passport.access_token.clone()))
                .path("/")
                .same_site(cookie::SameSite::Lax)
                .http_only(true)
                .max_age(Duration::days(14));

            let mut refresh_token_cookie = Cookie::build(("rft", passport.refresh_token.clone()))
                .path("/")
                .same_site(cookie::SameSite::Lax)
                .http_only(true)
                .max_age(Duration::days(14));

            if get_stage() == Stage::Production {
                access_token_cookie = access_token_cookie.secure(true);
                refresh_token_cookie = refresh_token_cookie.secure(true);
            }

            let mut headers = HeaderMap::new();
            headers.append(
                header::SET_COOKIE,
                header::HeaderValue::from_str(&access_token_cookie.to_string()).unwrap(),
            );
            headers.append(
                header::SET_COOKIE,
                header::HeaderValue::from_str(&refresh_token_cookie.to_string()).unwrap(),
            );

            (StatusCode::OK, headers, "Login successfully").into_response()
        }
        Err(e) => (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
    }
}

pub async fn adventurer_refresh_token<T1, T2>(
    State(authentication_usecase): State<Arc<AuthenticationUseCase<T1, T2>>>,
    jar: CookieJar,
) -> impl IntoResponse
where
    T1: AdventurerRepository + Send + Sync,
    T2: GuildCommanderRepository + Send + Sync,
{
    if let Some(rft) = jar.get("rft") {
        match authentication_usecase
            .adventurer_refresh_token(rft.value().to_string())
            .await
        {
            Ok(passport) => {
                let mut access_token_cookie = Cookie::build(("act", passport.access_token.clone()))
                    .path("/")
                    .same_site(cookie::SameSite::Lax)
                    .http_only(true)
                    .max_age(Duration::days(14));

                let mut refresh_token_cookie =
                    Cookie::build(("rft", passport.refresh_token.clone()))
                        .path("/")
                        .same_site(cookie::SameSite::Lax)
                        .http_only(true)
                        .max_age(Duration::days(14));

                if get_stage() == Stage::Production {
                    access_token_cookie = access_token_cookie.secure(true);
                    refresh_token_cookie = refresh_token_cookie.secure(true);
                }

                let mut headers = HeaderMap::new();
                headers.append(
                    header::SET_COOKIE,
                    header::HeaderValue::from_str(&access_token_cookie.to_string()).unwrap(),
                );
                headers.append(
                    header::SET_COOKIE,
                    header::HeaderValue::from_str(&refresh_token_cookie.to_string()).unwrap(),
                );

                (StatusCode::OK, headers, "Login successfully").into_response()
            }
            Err(e) => (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
        }
    } else {
        (StatusCode::BAD_REQUEST, "Refresh token not found").into_response()
    }
}

pub async fn guild_commander_login<T1, T2>(
    State(authentication_usecase): State<Arc<AuthenticationUseCase<T1, T2>>>,
    Json(login_model): Json<LoginModel>,
) -> impl IntoResponse
where
    T1: AdventurerRepository + Send + Sync,
    T2: GuildCommanderRepository + Send + Sync,
{
    match authentication_usecase
        .guild_commander_login(login_model)
        .await
    {
        Ok(passport) => {
            let mut access_token_cookie = Cookie::build(("act", passport.access_token.clone()))
                .path("/")
                .same_site(cookie::SameSite::Lax)
                .http_only(true)
                .max_age(Duration::days(14));

            let mut refresh_token_cookie = Cookie::build(("rft", passport.refresh_token.clone()))
                .path("/")
                .same_site(cookie::SameSite::Lax)
                .http_only(true)
                .max_age(Duration::days(14));

            if get_stage() == Stage::Production {
                access_token_cookie = access_token_cookie.secure(true);
                refresh_token_cookie = refresh_token_cookie.secure(true);
            }

            let mut headers = HeaderMap::new();
            headers.append(
                header::SET_COOKIE,
                header::HeaderValue::from_str(&access_token_cookie.to_string()).unwrap(),
            );
            headers.append(
                header::SET_COOKIE,
                header::HeaderValue::from_str(&refresh_token_cookie.to_string()).unwrap(),
            );

            (StatusCode::OK, headers, "Login successfully").into_response()
        }
        Err(e) => (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
    }
}

pub async fn guild_commander_refresh_token<T1, T2>(
    State(authentication_usecase): State<Arc<AuthenticationUseCase<T1, T2>>>,
    jar: CookieJar,
) -> impl IntoResponse
where
    T1: AdventurerRepository + Send + Sync,
    T2: GuildCommanderRepository + Send + Sync,
{
    if let Some(rft) = jar.get("rft") {
        match authentication_usecase
            .guild_commander_refresh_token(rft.value().to_string())
            .await
        {
            Ok(passport) => {
                let mut access_token_cookie = Cookie::build(("act", passport.access_token.clone()))
                    .path("/")
                    .same_site(cookie::SameSite::Lax)
                    .http_only(true)
                    .max_age(Duration::days(14));

                let mut refresh_token_cookie =
                    Cookie::build(("rft", passport.refresh_token.clone()))
                        .path("/")
                        .same_site(cookie::SameSite::Lax)
                        .http_only(true)
                        .max_age(Duration::days(14));

                if get_stage() == Stage::Production {
                    access_token_cookie = access_token_cookie.secure(true);
                    refresh_token_cookie = refresh_token_cookie.secure(true);
                }

                let mut headers = HeaderMap::new();
                headers.append(
                    header::SET_COOKIE,
                    header::HeaderValue::from_str(&access_token_cookie.to_string()).unwrap(),
                );
                headers.append(
                    header::SET_COOKIE,
                    header::HeaderValue::from_str(&refresh_token_cookie.to_string()).unwrap(),
                );

                (StatusCode::OK, headers, "Login successfully").into_response()
            }
            Err(e) => (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
        }
    } else {
        (StatusCode::BAD_REQUEST, "Refresh token not found").into_response()
    }
}
