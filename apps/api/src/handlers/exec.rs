use crate::AppState;
use crate::handlers::middleware::CurrentUser;
use axum::{Extension, Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::process::Command;

#[derive(Deserialize)]
pub struct ExecRequest {
    command: String,
}

#[derive(Serialize)]
pub struct ExecResponse {
    stdout: String,
    stderr: String,
    exit_code: i32,
}

pub fn exec_router() -> Router<Arc<AppState>> {
    Router::new().route("/", post(exec))
}

async fn exec(
    State(_state): State<Arc<AppState>>,
    Extension(_user): Extension<CurrentUser>,
    Json(payload): Json<ExecRequest>,
) -> impl IntoResponse {
    let output = Command::new("sh")
        .arg("-c")
        .arg(&payload.command)
        .output()
        .await;

    match output {
        Ok(out) => {
            Json(ExecResponse {
                stdout: String::from_utf8_lossy(&out.stdout).to_string(),
                stderr: String::from_utf8_lossy(&out.stderr).to_string(),
                exit_code: out.status.code().unwrap_or(-1),
            })
            .into_response()
        }
        Err(e) => {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to execute: {}", e),
            )
                .into_response()
        }
    }
}