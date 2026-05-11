use crate::AppState;
use crate::handlers::middleware::CurrentUser;

use axum::{
    Extension, Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::Arc;
use tokio::process::Command;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct CreateRepoRequest {
    name: String,
    description: Option<String>,
    visibility: Option<String>,
}

#[derive(Serialize, FromRow)]
pub struct RepoResponse {
    id: String,
    name: String,
    description: Option<String>,
    visibility: String,
    clone_url: String,
    created_at: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateVisibilityRequest {
    visibility: String,
}

#[derive(Serialize)]
pub struct TreeEntry {
    pub mode: String,
    pub r#type: String,
    pub sha: String,
    pub name: String,
}

pub fn repo_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/create", post(create_repo))
        .route("/list", get(list_repos))
        .route("/:id", get(get_repo))
        .route("/:id/visibility", put(update_visibility))
        .route("/:id/tree", get(browse_tree))
}

async fn create_repo(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<CurrentUser>,
    Json(payload): Json<CreateRepoRequest>,
) -> impl IntoResponse {
    let visibility = payload
        .visibility
        .as_deref()
        .filter(|v| ["public", "internal", "private"].contains(v))
        .unwrap_or("private");

    let repo_path = std::path::PathBuf::from(&state.config.repo_storage)
        .join(&user.username)
        .join(format!("{}.git", payload.name));

    let repo = sqlx::query_as::<_, RepoResponse>(
        r#"
        INSERT INTO repositories (name, description, owner_id, visibility)
        VALUES ($1, $2, $3, $4)
        RETURNING
            id::text as id,
            name,
            description,
            visibility,
            ''::text as clone_url,
            created_at::text as created_at
        "#,
    )
    .bind(&payload.name)
    .bind(&payload.description)
    .bind(&user.id)
    .bind(visibility)
    .fetch_one(&state.db)
    .await;

    let mut repo = match repo {
        Ok(r) => r,
        Err(e) => return (StatusCode::BAD_REQUEST, format!("Error: {}", e)).into_response(),
    };

    tokio::fs::create_dir_all(&repo_path).await.unwrap();
    Command::new("git")
        .args(["init", "--bare"])
        .current_dir(&repo_path)
        .output()
        .await
        .unwrap();

    repo.clone_url = format!("http://localhost:8080/git/{}/{}", user.username, payload.name);

    (StatusCode::CREATED, Json(repo)).into_response()
}

async fn list_repos(State(state): State<Arc<AppState>>) -> Result<Json<Vec<RepoResponse>>, StatusCode> {
    let repos = sqlx::query_as::<_, RepoResponse>(
        r#"
        SELECT
            id::text as id,
            name,
            description,
            visibility,
            ''::text as clone_url,
            created_at::text as created_at
        FROM repositories
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        eprintln!("DB error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let response: Vec<RepoResponse> = repos
        .into_iter()
        .map(|r| RepoResponse {
            clone_url: format!("http://localhost:8080/git/{}/{}", "kazilsky", r.name),
            ..r
        })
        .collect();

    Ok(Json(response))
}

async fn get_repo(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let repo = sqlx::query_as::<_, RepoResponse>(
        r#"
        SELECT
            r.id::text as id,
            r.name,
            r.description,
            r.visibility,
            u.username || '/' || r.name as clone_url,
            r.created_at::text as created_at
        FROM repositories r
        JOIN users u ON u.id = r.owner_id
        WHERE r.id::text = $1
        "#,
    )
    .bind(&id)
    .fetch_optional(&state.db)
    .await;

    match repo {
        Ok(Some(r)) => Json(r).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Not found").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {}", e)).into_response(),
    }
}

async fn browse_tree(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let path = params.get("path").map(|s| s.as_str()).unwrap_or("");
    let reference = params.get("ref").map(|s| s.as_str()).unwrap_or("HEAD");

    let repo = sqlx::query!(
        "SELECT r.name, u.username FROM repositories r JOIN users u ON u.id = r.owner_id WHERE r.id::text = $1",
        id
    )
    .fetch_optional(&state.db)
    .await;

    let (name, username) = match repo {
        Ok(Some(r)) => (r.name, r.username),
        _ => return (StatusCode::NOT_FOUND, "Repo not found").into_response(),
    };

    let repo_path = std::path::PathBuf::from(&state.config.repo_storage)
        .join(&username)
        .join(format!("{}.git", name));

    if !repo_path.exists() {
        return (StatusCode::NOT_FOUND, "Repo not found on disk").into_response();
    }

    let mut cmd = Command::new("git");
    cmd.args(["ls-tree", "-l", reference])
        .current_dir(&repo_path);

    if !path.is_empty() {
        cmd.arg(path);
    }

    let output = cmd.output().await;

    match output {
        Ok(out) if out.status.success() => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let entries: Vec<TreeEntry> = stdout
                .lines()
                .filter(|l| !l.is_empty())
                .filter_map(|line| {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 4 {
                        Some(TreeEntry {
                            mode: parts[0].to_string(),
                            r#type: parts[1].to_string(),
                            sha: parts[2].to_string(),
                            name: parts[3..].join(" "),
                        })
                    } else {
                        None
                    }
                })
                .collect();

            (StatusCode::OK, Json(entries)).into_response()
        }
        Ok(out) => {
            let err = String::from_utf8_lossy(&out.stderr);
            (StatusCode::NOT_FOUND, format!("Git error: {}", err)).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", e)).into_response(),
    }
}

async fn update_visibility(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<CurrentUser>,
    Json(payload): Json<UpdateVisibilityRequest>,
) -> impl IntoResponse {
    if !["public", "internal", "private"].contains(&payload.visibility.as_str()) {
        return (
            StatusCode::BAD_REQUEST,
            "Visibility must be public, internal, or private",
        )
            .into_response();
    }

    let result = sqlx::query!(
        "UPDATE repositories SET visibility = $1 WHERE id::text = $2 AND owner_id = $3",
        payload.visibility,
        id,
        user.id
    )
    .execute(&state.db)
    .await;

    match result {
        Ok(r) if r.rows_affected() > 0 => (StatusCode::OK, "Visibility updated").into_response(),
        Ok(_) => (StatusCode::NOT_FOUND, "Repo not found or not yours").into_response(),
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", e)).into_response()
        }
    }
}
