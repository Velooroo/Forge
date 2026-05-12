use crate::AppState;
use argon2::PasswordHash;
use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use argon2::{
    Argon2, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};

#[derive(Deserialize)]
pub struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub login: String,
    password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    token: String,
    user: UserResponse,
}

#[derive(Serialize)]
pub struct UserResponse {
    id: String,
    username: String,
    email: String,
}

#[derive(Serialize)]
pub struct TokenClaims {
    pub sub: String,
    pub username: String,
    pub exp: usize,
    pub iat: usize,
}

pub fn auth_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}

fn create_token(state: &AppState, user_id: &str, username: &str) -> Result<String, StatusCode> {
    let now = chrono::Utc::now();
    let claims = TokenClaims {
        sub: user_id.to_string(),
        username: username.to_string(),
        iat: now.timestamp() as usize,
        exp: (now + chrono::Duration::days(30)).timestamp() as usize,
    };

    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.config.jwt_secret.as_bytes()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = match argon2
        .hash_password(payload.password.as_bytes(), &salt)
    {
        Ok(h) => h.to_string(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Hashing failed").into_response(),
    };

    let user = sqlx::query!(
        "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING id, username, email",
        payload.username,
        payload.email,
        password_hash
    )
    .fetch_one(&state.db)
    .await;

    let user = match user {
        Ok(u) => u,
        Err(e) => return (StatusCode::BAD_REQUEST, format!("Error: {}", e)).into_response(),
    };

    let token = match create_token(&state, &user.id.to_string(), &user.username) {
        Ok(t) => t,
        Err(s) => return (s, "Token creation failed").into_response(),
    };

    (
        StatusCode::CREATED,
        Json(AuthResponse {
            token,
            user: UserResponse {
                id: user.id.to_string(),
                username: user.username,
                email: user.email,
            },
        }),
    )
        .into_response()
}

async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let user = sqlx::query!(
        "SELECT id, username, email, password_hash FROM users WHERE username = $1 OR email = $1",
        payload.login
    )
    .fetch_optional(&state.db)
    .await;

    let user = match user {
        Ok(Some(u)) => u,
        Ok(None) => {
            return (StatusCode::UNAUTHORIZED, "Неверные учётные данные").into_response()
        }
        Err(e) => {
            eprintln!("Database error: {e}");
            return (StatusCode::INTERNAL_SERVER_ERROR, "Server error").into_response();
        }
    };

    let parsed = match PasswordHash::new(&user.password_hash) {
        Ok(ph) => ph,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Server error").into_response();
        }
    };

    if Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed)
        .is_err()
    {
        return (StatusCode::UNAUTHORIZED, "Неверные учётные данные").into_response();
    }

    let token = match create_token(&state, &user.id.to_string(), &user.username) {
        Ok(t) => t,
        Err(s) => return (s, "Token creation failed").into_response(),
    };

    (
        StatusCode::OK,
        Json(AuthResponse {
            token,
            user: UserResponse {
                id: user.id.to_string(),
                username: user.username,
                email: user.email,
            },
        }),
    )
        .into_response()
}