use crate::AppState;
use crate::handlers::middleware::CurrentUser;

use axum::{
    Extension, Json, Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};
use tokio::process::Command;

#[derive(Deserialize)]
pub struct CreateRepoRequest {
    name: String,
    description: Option<String>,
}

#[derive(Serialize)]
pub struct RepoResponse {
    id: String,
    name: String,
    description: Option<String>,
    clone_url: String,
}

pub fn repo_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/create", post(create_repo))
        .route("/list", get(list_repos))
}

async fn create_repo(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<CurrentUser>,
    Json(payload): Json<CreateRepoRequest>,
) -> impl IntoResponse {
    // Создаём запись в БД
    let repo = sqlx::query!(
        r#"
        INSERT INTO repositories (name, description, owner_id)
        VALUES ($1, $2, $3)
        RETURNING id::text as id, name, description
        "#,
        payload.name,
        payload.description,
        user.id
    )
    .fetch_one(&state.db)
    .await;

    let repo = match repo {
        Ok(r) => r,
        Err(e) => return (StatusCode::BAD_REQUEST, format!("Error: {}", e)).into_response(),
    };

    // Создаём папку и git init
    let repo_path = PathBuf::from(&state.config.repo_storage)
        .join("kazilsky")
        .join(format!("{}.git", payload.name));

    tokio::fs::create_dir_all(&repo_path).await.unwrap();

    Command::new("git")
        .args(["init", "--bare"])
        .current_dir(&repo_path)
        .output()
        .await
        .unwrap();

    let response = RepoResponse {
        id: repo.id.unwrap(), // id уже String из-за ::text
        name: repo.name,
        description: repo.description,
        clone_url: format!("http://localhost:8080/kazilsky/{}", payload.name),
    };

    (StatusCode::CREATED, Json(response)).into_response()
}

async fn list_repos(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    println!("test");
    let repos = sqlx::query!("SELECT id::text as id, name, description FROM repositories")
        .fetch_all(&state.db)
        .await
        .unwrap();

    let response: Vec<RepoResponse> = repos
        .into_iter()
        .map(|r| RepoResponse {
            id: r.id.unwrap(),
            name: r.name.to_string(),
            description: r.description,
            clone_url: format!("http://localhost:8080/git/kazilsky/{}", r.name),
        })
        .collect();

    Json(response)
}
