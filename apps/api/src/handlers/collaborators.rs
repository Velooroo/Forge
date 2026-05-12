use crate::AppState;
use crate::handlers::middleware::CurrentUser;

use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct AddCollaboratorRequest {
    repo_id: Uuid,
    username: String,
    permission: Option<String>,
}

#[derive(Serialize)]
pub struct CollaboratorResponse {
    id: String,
    repo_id: String,
    user_id: String,
    username: String,
    permission: String,
}

pub fn collaborators_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/add", post(add_collaborator))
        .route("/remove/:repo_id/:user_id", delete(remove_collaborator))
        .route("/:repo_id", get(list_collaborators))
        .route("/:repo_id/:username/permission", put(update_permission))
}

async fn add_collaborator(
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Json(payload): Json<AddCollaboratorRequest>,
) -> impl IntoResponse {
    let permission = payload.permission.unwrap_or_else(|| "read".to_string());
    if !["read", "write", "admin"].contains(&permission.as_str()) {
        return (StatusCode::BAD_REQUEST, "Invalid permission").into_response();
    }

    let repo = sqlx::query!(
        "SELECT owner_id FROM repositories WHERE id = $1",
        payload.repo_id
    )
    .fetch_optional(&state.db)
    .await;

    let repo = match repo {
        Ok(Some(r)) => r,
        Ok(None) => return (StatusCode::NOT_FOUND, "Repository not found").into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "DB error").into_response(),
    };

    if repo.owner_id != current_user.id {
        return (StatusCode::FORBIDDEN, "Only the owner can add collaborators").into_response();
    }

    let target_user = sqlx::query!(
        "SELECT id FROM users WHERE username = $1",
        payload.username
    )
    .fetch_optional(&state.db)
    .await;

    let target_user = match target_user {
        Ok(Some(u)) => u,
        Ok(None) => return (StatusCode::NOT_FOUND, "User not found").into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "DB error").into_response(),
    };

    let result = sqlx::query!(
        "INSERT INTO collaborators (repo_id, user_id, permission) VALUES ($1, $2, $3) ON CONFLICT (repo_id, user_id) DO UPDATE SET permission = $3 RETURNING id, repo_id, user_id, permission",
        payload.repo_id,
        target_user.id,
        permission
    )
    .fetch_one(&state.db)
    .await;

    match result {
        Ok(c) => (
            StatusCode::CREATED,
            Json(CollaboratorResponse {
                id: c.id.to_string(),
                repo_id: c.repo_id.to_string(),
                user_id: c.user_id.to_string(),
                username: payload.username,
                permission: c.permission,
            }),
        )
            .into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, format!("Error: {}", e)).into_response(),
    }
}

async fn remove_collaborator(
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Path((repo_id, user_id)): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
    let repo = sqlx::query!(
        "SELECT owner_id FROM repositories WHERE id = $1",
        repo_id
    )
    .fetch_optional(&state.db)
    .await;

    let repo = match repo {
        Ok(Some(r)) => r,
        Ok(None) => return (StatusCode::NOT_FOUND, "Repository not found").into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "DB error").into_response(),
    };

    if repo.owner_id != current_user.id {
        return (StatusCode::FORBIDDEN, "Only the owner can remove collaborators").into_response();
    }

    sqlx::query!(
        "DELETE FROM collaborators WHERE repo_id = $1 AND user_id = $2",
        repo_id,
        user_id
    )
    .execute(&state.db)
    .await
    .ok();

    (StatusCode::OK, "Collaborator removed").into_response()
}

async fn list_collaborators(
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(repo_id): Path<Uuid>,
) -> impl IntoResponse {
    let repo = sqlx::query!(
        "SELECT owner_id FROM repositories WHERE id = $1",
        repo_id
    )
    .fetch_optional(&state.db)
    .await;

    let repo = match repo {
        Ok(Some(r)) => r,
        Ok(None) => return (StatusCode::NOT_FOUND, "Repository not found").into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "DB error").into_response(),
    };

    if repo.owner_id != current_user.id {
        let is_collab = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM collaborators WHERE repo_id = $1 AND user_id = $2",
            repo_id,
            current_user.id
        )
        .fetch_one(&state.db)
        .await;

        if let Ok(Some(0)) | Ok(None) = is_collab {
            return (StatusCode::FORBIDDEN, "Access denied").into_response();
        }
    }

    let collaborators = sqlx::query!(
        r#"
        SELECT c.id, c.repo_id, c.user_id, c.permission, u.username
        FROM collaborators c
        JOIN users u ON c.user_id = u.id
        WHERE c.repo_id = $1
        "#,
        repo_id
    )
    .fetch_all(&state.db)
    .await;

    let collaborators = match collaborators {
        Ok(c) => c,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "DB error").into_response(),
    };

    let response: Vec<CollaboratorResponse> = collaborators
        .into_iter()
        .map(|c| CollaboratorResponse {
            id: c.id.to_string(),
            repo_id: c.repo_id.to_string(),
            user_id: c.user_id.to_string(),
            username: c.username,
            permission: c.permission,
        })
        .collect();

    Json(response).into_response()
}

async fn update_permission(
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Path((repo_id, username)): Path<(Uuid, String)>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let permission = payload.get("permission").and_then(|v| v.as_str()).unwrap_or("read");
    if !["read", "write", "admin"].contains(&permission) {
        return (StatusCode::BAD_REQUEST, "Invalid permission").into_response();
    }

    let repo = sqlx::query!(
        "SELECT owner_id FROM repositories WHERE id = $1",
        repo_id
    )
    .fetch_optional(&state.db)
    .await;

    let repo = match repo {
        Ok(Some(r)) => r,
        Ok(None) => return (StatusCode::NOT_FOUND, "Repository not found").into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "DB error").into_response(),
    };

    if repo.owner_id != current_user.id {
        return (StatusCode::FORBIDDEN, "Only the owner can change permissions").into_response();
    }

    let target_user = sqlx::query!(
        "SELECT id FROM users WHERE username = $1",
        username
    )
    .fetch_optional(&state.db)
    .await;

    let target_user = match target_user {
        Ok(Some(u)) => u,
        Ok(None) => return (StatusCode::NOT_FOUND, "User not found").into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "DB error").into_response(),
    };

    sqlx::query!(
        "UPDATE collaborators SET permission = $1 WHERE repo_id = $2 AND user_id = $3",
        permission,
        repo_id,
        target_user.id
    )
    .execute(&state.db)
    .await
    .ok();

    (StatusCode::OK, "Permission updated").into_response()
}