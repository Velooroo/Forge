use crate::AppState;
use crate::handlers::middleware::CurrentUser;
use axum::{
    body::Bytes,
    extract::{Path, Query, State},
    http::{StatusCode, header},
    response::IntoResponse,
};
use base64::{Engine as _, engine::general_purpose};
use std::{collections::HashMap, path::PathBuf, process::Stdio, sync::Arc};
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

fn extract_basic_creds(headers: &axum::http::HeaderMap) -> Option<(String, String)> {
    let auth_header = headers.get(header::AUTHORIZATION)?.to_str().ok()?;
    if !auth_header.starts_with("Basic ") {
        return None;
    }
    let credentials = auth_header.trim_start_matches("Basic ");
    let decoded = general_purpose::STANDARD.decode(credentials).ok()?;
    let creds = String::from_utf8(decoded).ok()?;
    let parts: Vec<&str> = creds.splitn(2, ':').collect();
    if parts.len() != 2 {
        return None;
    }
    Some((parts[0].to_string(), parts[1].to_string()))
}

async fn verify_basic_user(
    state: &Arc<AppState>,
    username: &str,
    password: &str,
) -> Option<CurrentUser> {
    let user = sqlx::query!(
        "SELECT id, username, password_hash FROM users WHERE username = $1",
        username
    )
    .fetch_optional(&state.db)
    .await
    .ok()?;

    let user = user?;

    use argon2::{Argon2, PasswordHash, PasswordVerifier};
    let parsed_hash = PasswordHash::new(&user.password_hash).ok()?;
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .ok()?;

    Some(CurrentUser {
        id: user.id,
        username: user.username,
    })
}

async fn get_authenticated_user(
    state: &Arc<AppState>,
    headers: &axum::http::HeaderMap,
) -> Option<CurrentUser> {
    let (username, password) = extract_basic_creds(headers)?;
    verify_basic_user(state, &username, &password).await
}

/// Auto-create repo directory + DB record on first push
async fn ensure_repo_exists(
    state: &Arc<AppState>,
    user: &str,
    repo: &str,
    current_user: &CurrentUser,
) -> Result<PathBuf, StatusCode> {
    let repo_path = PathBuf::from(&state.config.repo_storage)
        .join(user)
        .join(format!("{}.git", repo));

    if repo_path.exists() {
        return Ok(repo_path);
    }

    // Only owner can auto-create
    if current_user.username != user {
        return Err(StatusCode::FORBIDDEN);
    }

    sqlx::query!(
        "INSERT INTO repositories (name, owner_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
        repo,
        current_user.id
    )
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    tokio::fs::create_dir_all(&repo_path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let output = Command::new("git")
        .args(["init", "--bare"])
        .current_dir(&repo_path)
        .output()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !output.status.success() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(repo_path)
}

/// Get repo path and check read access
async fn check_read_access(
    state: &Arc<AppState>,
    user: &str,
    repo: &str,
    current_user: Option<&CurrentUser>,
) -> Result<PathBuf, StatusCode> {
    let repo_path = PathBuf::from(&state.config.repo_storage)
        .join(user)
        .join(format!("{}.git", repo));

    let repo_info = sqlx::query!(
        "SELECT r.id, r.is_private, r.owner_id FROM repositories r JOIN users u ON r.owner_id = u.id WHERE u.username = $1 AND r.name = $2",
        user,
        repo
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let repo_info = match repo_info {
        Some(r) => r,
        None => {
            if repo_path.exists() {
                return Ok(repo_path);
            }
            return Err(StatusCode::NOT_FOUND);
        }
    };

    if repo_info.is_private {
        let current_user = current_user.ok_or(StatusCode::UNAUTHORIZED)?;

        if current_user.id != repo_info.owner_id {
            let is_collab = sqlx::query_scalar!(
                "SELECT COUNT(*) FROM collaborators WHERE repo_id = $1 AND user_id = $2",
                repo_info.id,
                current_user.id
            )
            .fetch_one(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .unwrap_or(0)
                > 0;

            if !is_collab {
                return Err(StatusCode::FORBIDDEN);
            }
        }
    }

    Ok(repo_path)
}

/// Check write access (owner + admin/write collaborators)
async fn check_write_access(
    state: &Arc<AppState>,
    user: &str,
    repo: &str,
    current_user: &CurrentUser,
) -> Result<PathBuf, StatusCode> {
    let repo_path = PathBuf::from(&state.config.repo_storage)
        .join(user)
        .join(format!("{}.git", repo));

    let repo_info = sqlx::query!(
        "SELECT r.id, r.owner_id FROM repositories r JOIN users u ON r.owner_id = u.id WHERE u.username = $1 AND r.name = $2",
        user,
        repo
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (repo_id, owner_id) = match repo_info {
        Some(r) => (r.id, r.owner_id),
        None => {
            return ensure_repo_exists(state, user, repo, current_user).await;
        }
    };

    if current_user.id == owner_id {
        return Ok(repo_path);
    }

    let perm = sqlx::query_scalar!(
        "SELECT permission FROM collaborators WHERE repo_id = $1 AND user_id = $2",
        repo_id,
        current_user.id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match perm.as_deref() {
        Some("write") | Some("admin") => Ok(repo_path),
        _ => Err(StatusCode::FORBIDDEN),
    }
}

pub async fn handle_info_refs(
    Path((user, repo)): Path<(String, String)>,
    Query(params): Query<HashMap<String, String>>,
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
) -> impl IntoResponse {
    let service = match params.get("service") {
        Some(s) => s.clone(),
        None => return (StatusCode::BAD_REQUEST, "Missing service param").into_response(),
    };

    let require_write = service == "git-receive-pack";
    let repo_path = if require_write {
        let current_user = match get_authenticated_user(&state, &headers).await {
            Some(u) => u,
            None => return (StatusCode::UNAUTHORIZED, "Authentication required").into_response(),
        };
        match check_write_access(&state, &user, &repo, &current_user).await {
            Ok(p) => p,
            Err(s) => return (s, "Access denied").into_response(),
        }
    } else {
        let current_user = get_authenticated_user(&state, &headers).await;
        match check_read_access(&state, &user, &repo, current_user.as_ref()).await {
            Ok(p) => p,
            Err(s) => return (s, "Access denied").into_response(),
        }
    };

    let git_command = if service == "git-upload-pack" {
        "upload-pack"
    } else {
        "receive-pack"
    };

    let output = Command::new("git")
        .arg(git_command)
        .arg("--advertise-refs")
        .arg(&repo_path)
        .output()
        .await;

    match output {
        Ok(out) if out.status.success() => {
            let mut response = Vec::new();
            let service_header = format!("# service={}\n", service);
            let header_length = service_header.len() + 4;
            response.extend_from_slice(format!("{:04x}", header_length).as_bytes());
            response.extend_from_slice(service_header.as_bytes());
            response.extend_from_slice(b"0000");
            response.extend_from_slice(&out.stdout);
            (
                [(
                    header::CONTENT_TYPE,
                    format!("application/x-{}-advertisement", service),
                )],
                response,
            )
                .into_response()
        }
        Ok(out) => {
            eprintln!("git error: {}", String::from_utf8_lossy(&out.stderr));
            (StatusCode::INTERNAL_SERVER_ERROR, "Git command failed").into_response()
        }
        Err(e) => {
            eprintln!("spawn error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to run git").into_response()
        }
    }
}

pub async fn handle_upload_pack(
    Path((user, repo)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    let current_user = get_authenticated_user(&state, &headers).await;
    let repo_path = match check_read_access(&state, &user, &repo, current_user.as_ref()).await {
        Ok(p) => p,
        Err(status) => return (status, "Access denied").into_response(),
    };

    let mut child = match Command::new("git")
        .arg("upload-pack")
        .arg("--stateless-rpc")
        .arg(&repo_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to spawn git").into_response();
        }
    };

    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(&body).await;
    }

    match child.wait_with_output().await {
        Ok(out) if out.status.success() => (
            [(header::CONTENT_TYPE, "application/x-git-upload-pack-result")],
            out.stdout,
        )
            .into_response(),
        _ => (StatusCode::INTERNAL_SERVER_ERROR, "git-upload-pack failed").into_response(),
    }
}

pub async fn handle_receive_pack(
    Path((user, repo)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    let current_user = match get_authenticated_user(&state, &headers).await {
        Some(u) => u,
        None => return (StatusCode::UNAUTHORIZED, "Authentication required").into_response(),
    };

    let repo_path = match check_write_access(&state, &user, &repo, &current_user).await {
        Ok(p) => p,
        Err(status) => return (status, "Access denied").into_response(),
    };

    let mut child = match Command::new("git")
        .arg("receive-pack")
        .arg("--stateless-rpc")
        .arg(&repo_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to spawn git").into_response();
        }
    };

    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(&body).await;
    }

    match child.wait_with_output().await {
        Ok(out) if out.status.success() => {
            println!("Push received: {}/{}", user, repo);
            (
                [(
                    header::CONTENT_TYPE,
                    "application/x-git-receive-pack-result",
                )],
                out.stdout,
            )
                .into_response()
        }
        _ => (StatusCode::INTERNAL_SERVER_ERROR, "git-receive-pack failed").into_response(),
    }
}

pub async fn get_archive(
    Path((user, repo)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
) -> impl IntoResponse {
    let current_user = get_authenticated_user(&state, &headers).await;
    let repo_path = match check_read_access(&state, &user, &repo, current_user.as_ref()).await {
        Ok(p) => p,
        Err(status) => return (status, "Access denied").into_response(),
    };

    println!("📦 Archiving {}/{}...", user, repo);

    let output = Command::new("git")
        .arg("archive")
        .arg("--format=tar.gz")
        .arg("HEAD")
        .current_dir(&repo_path)
        .output()
        .await;

    match output {
        Ok(out) if out.status.success() => {
            (
                [
                    (header::CONTENT_TYPE, "application/gzip"),
                    (
                        header::CONTENT_DISPOSITION,
                        &format!("attachment; filename=\"{}-{}.tar.gz\"", repo, "HEAD"),
                    ),
                ],
                out.stdout,
            )
                .into_response()
        }
        Ok(out) => {
            let err = String::from_utf8_lossy(&out.stderr);
            println!("Archive error: {}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Git error: {}", err),
            )
                .into_response()
        }
        Err(e) => {
            eprintln!("spawn error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to run git").into_response()
        }
    }
}