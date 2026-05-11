use crate::AppState;
use argon2::PasswordHash;
use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use serde::Deserialize;
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
    email: String,
    password: String,
}

pub fn auth_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}

async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    // 1. Хэшируем пароль
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    // 2. Записываем в БД через SQLx
    let res = sqlx::query!(
        "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3)",
        payload.username,
        payload.email,
        password_hash
    )
    .execute(&state.db)
    .await;

    match res {
        Ok(_) => (StatusCode::CREATED, "User created").into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, format!("Error: {}", e)).into_response(),
    }
}

async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    // Можно также использовать fetch_optional вместо fetch_one
    let user = sqlx::query!(
        "SELECT password_hash FROM users WHERE email = $1",
        payload.email
    )
    .fetch_optional(&state.db)
    .await;

    match user {
        Ok(Some(user)) => {
            let argon2 = Argon2::default();

            let parsed = match PasswordHash::new(&user.password_hash) {
                Ok(ph) => ph,
                Err(_) => {
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Server error").into_response();
                }
            };

            match argon2.verify_password(payload.password.as_bytes(), &parsed) {
                Ok(()) => (StatusCode::OK, "Logged in").into_response(),
                Err(_) => (StatusCode::UNAUTHORIZED, "Неверные учётные данные").into_response(),
            }
        }
        Ok(None) => {
            // Пользователь не найден
            (StatusCode::UNAUTHORIZED, "Неверные учётные данные").into_response()
        }
        Err(e) => {
            eprintln!("Database error: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Server error").into_response()
        }
    }
}
