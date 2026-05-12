use crate::AppState;
use crate::handlers::middleware::CurrentUser;

use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};
use tokio::process::Command;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateRepoRequest {
    name: String,
    description: Option<String>,
    is_private: Option<bool>,
}

#[derive(Serialize)]
pub struct RepoResponse {
    id: String,
    name: String,
    description: Option<String>,
    is_private: bool,
    clone_url: String,
    owner_username: String,
    created_at: String,
}

pub fn repo_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/create", post(create_repo))
        .route("/list", get(list_repos))
        .route("/mine", get(my_repos))
        .route("/:id", get(get_repo))
}

async fn create_repo(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<CurrentUser>,
    Json(payload): Json<CreateRepoRequest>,
) -> impl IntoResponse {
    let is_private = payload.is_private.unwrap_or(false);

    let repo = sqlx::query!(
        r#"
        INSERT INTO repositories (name, description, owner_id, is_private)
        VALUES ($1, $2, $3, $4)
        RETURNING id, name, description, is_private, created_at
        "#,
        payload.name,
        payload.description,
        user.id,
        is_private
    )
    .fetch_one(&state.db)
    .await;

    let repo = match repo {
        Ok(r) => r,
        Err(e) => return (StatusCode::BAD_REQUEST, format!("Error: {}", e)).into_response(),
    };

    let repo_path = PathBuf::from(&state.config.repo_storage)
        .join(&user.username)
        .join(format!("{}.git", payload.name));

    if let Err(e) = tokio::fs::create_dir_all(&repo_path).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("FS error: {}", e)).into_response();
    }

    if let Err(e) = Command::new("git")
        .args(["init", "--bare"])
        .current_dir(&repo_path)
        .output()
        .await
    {
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("Git error: {}", e)).into_response();
    }

    let host = "localhost:8080";
    let response = RepoResponse {
        id: repo.id.to_string(),
        name: repo.name,
        description: repo.description,
        is_private: repo.is_private,
        clone_url: format!("http://{}/git/{}/{}", host, user.username, payload.name),
        owner_username: user.username.clone(),
        created_at: repo.created_at.map(|t| t.to_string()).unwrap_or_default(),
    };

    (StatusCode::CREATED, Json(response)).into_response()
}

async fn list_repos(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let repos = sqlx::query!(
        r#"
        SELECT r.id, r.name, r.description, r.is_private, r.created_at, u.username as owner_username
        FROM repositories r
        JOIN users u ON r.owner_id = u.id
        WHERE r.is_private = false
        ORDER BY r.created_at DESC
        "#
    )
    .fetch_all(&state.db)
    .await;

    let repos = match repos {
        Ok(r) => r,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "DB error").into_response(),
    };

    let response: Vec<RepoResponse> = repos
        .into_iter()
.map(|r| RepoResponse {
            id: r.id.to_string(),
            name: r.name.clone(),
            description: r.description,
            is_private: r.is_private,
            clone_url: format!("http://localhost:8080/git/{}/{}", r.owner_username, r.name),
            owner_username: r.owner_username,
            created_at: r.created_at.map(|t| t.to_string()).unwrap_or_default(),
        })
        .collect();

    Json(response).into_response()
}

async fn my_repos(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<CurrentUser>,
) -> impl IntoResponse {
    let repos = sqlx::query!(
        r#"
        SELECT r.id, r.name, r.description, r.is_private, r.created_at, u.username as owner_username
        FROM repositories r
        JOIN users u ON r.owner_id = u.id
        WHERE r.owner_id = $1
        ORDER BY r.created_at DESC
        "#,
        user.id
    )
    .fetch_all(&state.db)
    .await;

    let repos = match repos {
        Ok(r) => r,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "DB error").into_response(),
    };

    let response: Vec<RepoResponse> = repos
        .into_iter()
        .map(|r| RepoResponse {
            id: r.id.to_string(),
            name: r.name.clone(),
            description: r.description,
            is_private: r.is_private,
            clone_url: format!("http://localhost:8080/git/{}/{}", r.owner_username, r.name),
            owner_username: r.owner_username,
            created_at: r.created_at.map(|t| t.to_string()).unwrap_or_default(),
        })
        .collect();

    Json(response).into_response()
}

async fn get_repo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let repo = sqlx::query!(
        r#"
        SELECT r.id, r.name, r.description, r.is_private, r.created_at, u.username as owner_username
        FROM repositories r
        JOIN users u ON r.owner_id = u.id
        WHERE r.id = $1
        "#,
        id
    )
    .fetch_optional(&state.db)
    .await;

    let repo = match repo {
        Ok(Some(r)) => r,
        Ok(None) => return (StatusCode::NOT_FOUND, "Repository not found").into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "DB error").into_response(),
    };

    let response = RepoResponse {
        id: repo.id.to_string(),
        name: repo.name.clone(),
        description: repo.description,
        is_private: repo.is_private,
        clone_url: format!("http://localhost:8080/git/{}/{}", repo.owner_username, repo.name),
        owner_username: repo.owner_username,
        created_at: repo.created_at.map(|t| t.to_string()).unwrap_or_default(),
    };

    Json(response).into_response()
}