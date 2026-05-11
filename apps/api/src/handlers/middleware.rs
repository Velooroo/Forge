use crate::AppState;
use axum::{
    extract::{Request, State},
    http::{StatusCode, header},
    middleware::Next,
    response::{IntoResponse, Response},
};
use base64::{Engine as _, engine::general_purpose};
use std::sync::Arc;

// Структура текущего юзера
#[derive(Clone, Debug)]
pub struct CurrentUser {
    pub id: sqlx::types::Uuid,
    pub username: String,
}

// Middleware для Basic Auth
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
            println!("Auth failed");
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

// Проверка Basic Auth
async fn verify_basic_auth(state: &AppState, auth_header: Option<&str>) -> Result<CurrentUser, ()> {
    let header = auth_header.ok_or_else(|| {
        println!("No auth header");
    })?;

    if !header.starts_with("Basic ") {
        println!("Not Basic auth");
        return Err(());
    }

    let credentials = header.trim_start_matches("Basic ");
    let decoded = general_purpose::STANDARD.decode(credentials).map_err(|e| {
        println!("Base64 decode failed: {}", e);
    })?;

    let creds = String::from_utf8(decoded).map_err(|e| {
        println!("UTF8 decode failed: {}", e);
    })?;

    let parts: Vec<&str> = creds.splitn(2, ':').collect();
    if parts.len() != 2 {
        println!("Invalid format");
        return Err(());
    }

    let (username, password) = (parts[0], parts[1]);

    let user = sqlx::query!(
        "SELECT id, username, password_hash FROM users WHERE username = $1",
        username
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        println!("DB error: {}", e);
    })?
    .ok_or_else(|| {
        println!("User not found");
    })?;

    use argon2::{Argon2, PasswordHash, PasswordVerifier};
    let parsed_hash = PasswordHash::new(&user.password_hash).map_err(|e| {
        println!("Hash parse error: {}", e);
    })?;

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|e| {
            println!("Password verify failed: {}", e);
        })?;

    Ok(CurrentUser {
        id: user.id,
        username: user.username,
    })
}
