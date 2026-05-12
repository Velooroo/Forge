use crate::AppState;
use crate::handlers::middleware::CurrentUser;

use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
pub struct UserProfile {
    id: String,
    username: String,
    email: String,
    created_at: String,
    repo_count: i64,
}

pub fn users_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/me", get(me))
        .route("/:username", get(get_user))
}

async fn me(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<CurrentUser>,
) -> impl IntoResponse {
    let profile = sqlx::query!(
        r#"
        SELECT u.id, u.username, u.email, u.created_at,
               (SELECT COUNT(*) FROM repositories WHERE owner_id = u.id) as repo_count
        FROM users u WHERE u.id = $1
        "#,
        user.id
    )
    .fetch_optional(&state.db)
    .await;

    let profile = match profile {
        Ok(Some(p)) => p,
        Ok(None) => return (StatusCode::NOT_FOUND, "User not found").into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "DB error").into_response(),
    };

    Json(UserProfile {
        id: profile.id.to_string(),
        username: profile.username,
        email: profile.email,
        created_at: profile.created_at.map(|t| t.to_string()).unwrap_or_default(),
        repo_count: profile.repo_count.unwrap_or(0),
    })
    .into_response()
}

async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(username): Path<String>,
) -> impl IntoResponse {
    let profile = sqlx::query!(
        r#"
        SELECT u.id, u.username, u.email, u.created_at,
               (SELECT COUNT(*) FROM repositories WHERE owner_id = u.id) as repo_count
        FROM users u WHERE u.username = $1
        "#,
        username
    )
    .fetch_optional(&state.db)
    .await;

    let profile = match profile {
        Ok(Some(p)) => p,
        Ok(None) => return (StatusCode::NOT_FOUND, "User not found").into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "DB error").into_response(),
    };

    Json(UserProfile {
        id: profile.id.to_string(),
        username: profile.username,
        email: profile.email,
        created_at: profile.created_at.map(|t| t.to_string()).unwrap_or_default(),
        repo_count: profile.repo_count.unwrap_or(0),
    })
    .into_response()
}