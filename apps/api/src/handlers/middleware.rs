use crate::AppState;
use axum::{
    extract::{Request, State},
    http::{StatusCode, header},
    middleware::Next,
    response::{IntoResponse, Response},
};
use base64::{Engine as _, engine::general_purpose};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct CurrentUser {
    pub id: sqlx::types::Uuid,
    pub username: String,
}

#[derive(Deserialize)]
struct JwtClaims {
    sub: String,
    username: String,
    exp: usize,
}

async fn verify_basic_auth(state: &AppState, auth_header: Option<&str>) -> Result<CurrentUser, ()> {
    let header = auth_header.ok_or(())?;
    if !header.starts_with("Basic ") {
        return Err(());
    }

    let credentials = header.trim_start_matches("Basic ");
    let decoded = general_purpose::STANDARD.decode(credentials).map_err(|_| ())?;
    let creds = String::from_utf8(decoded).map_err(|_| ())?;

    let parts: Vec<&str> = creds.splitn(2, ':').collect();
    if parts.len() != 2 {
        return Err(());
    }

    let (username, password) = (parts[0], parts[1]);

    let user = sqlx::query!(
        "SELECT id, username, password_hash FROM users WHERE username = $1",
        username
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| ())?
    .ok_or(())?;

    use argon2::{Argon2, PasswordHash, PasswordVerifier};
    let parsed_hash = PasswordHash::new(&user.password_hash).map_err(|_| ())?;

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| ())?;

    Ok(CurrentUser {
        id: user.id,
        username: user.username,
    })
}

fn verify_jwt(state: &AppState, auth_header: Option<&str>) -> Result<CurrentUser, ()> {
    let header = auth_header.ok_or(())?;
    if !header.starts_with("Bearer ") {
        return Err(());
    }

    let token = header.trim_start_matches("Bearer ");

    let data = decode::<JwtClaims>(
        token,
        &DecodingKey::from_secret(state.config.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| ())?;

    let id = sqlx::types::Uuid::parse_str(&data.claims.sub).map_err(|_| ())?;

    Ok(CurrentUser {
        id,
        username: data.claims.username,
    })
}

pub async fn optional_auth(
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Response {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok());

    if let Ok(user) = verify_basic_auth(&state, auth_header).await {
        req.extensions_mut().insert(user);
    }

    next.run(req).await
}

pub async fn require_auth(
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<Response, Response> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok());

    let user = match verify_basic_auth(&state, auth_header).await {
        Ok(u) => u,
        Err(_) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                [(header::WWW_AUTHENTICATE, "Basic realm=\"Git\"")],
                "Unauthorized",
            )
                .into_response());
        }
    };

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}

pub async fn require_jwt(
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<Response, Response> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok());

    let user = match verify_jwt(&state, auth_header) {
        Ok(u) => u,
        Err(_) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                "Unauthorized — valid JWT required",
            )
                .into_response());
        }
    };

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}