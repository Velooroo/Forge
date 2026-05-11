use crate::AppState;
use axum::{
    body::Bytes,
    extract::{Path, Query, State},
    http::{StatusCode, header},
    response::IntoResponse,
};
use sqlx::types::uuid;
use std::{collections::HashMap, path::PathBuf, process::Stdio, sync::Arc};
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

/// GET /:user/:repo/info/refs?service=git-upload-pack
pub async fn handle_info_refs(
    Path((user, repo)): Path<(String, String)>,
    Query(params): Query<HashMap<String, String>>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    // Авто-создание репо
    let repo_path = match ensure_repo_exists(&state, &user, &repo).await {
        Ok(p) => p,
        Err(status) => return (status, "Failed to create repo").into_response(),
    };

    let service = match params.get("service") {
        Some(s) => s.clone(),
        None => return (StatusCode::BAD_REQUEST, "Missing service param").into_response(),
    };

    if !repo_path.exists() {
        return (StatusCode::NOT_FOUND, "Repository not found").into_response();
    }

    // git-upload-pack или git-receive-pack
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
            // Формируем PKT-LINE ответ
            let mut response = Vec::new();

            // Сервисный заголовок
            let service_header = format!("# service={}\n", service);
            let header_length = service_header.len() + 4;
            response.extend_from_slice(format!("{:04x}", header_length).as_bytes());
            response.extend_from_slice(service_header.as_bytes());

            // Разделитель
            response.extend_from_slice(b"0000");

            // Вывод git команды
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

/// POST /:user/:repo/git-upload-pack (clone/fetch)
pub async fn handle_upload_pack(
    Path((user, repo)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    body: Bytes,
) -> impl IntoResponse {
    // Авто-создание репо
    let repo_path = match ensure_repo_exists(&state, &user, &repo).await {
        Ok(p) => p,
        Err(status) => return (status, "Failed to create repo").into_response(),
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

/// POST /:user/:repo/git-receive-pack (push)
pub async fn handle_receive_pack(
    Path((user, repo)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    body: Bytes,
) -> impl IntoResponse {
    // Авто-создание репо
    let repo_path = match ensure_repo_exists(&state, &user, &repo).await {
        Ok(p) => p,
        Err(status) => return (status, "Failed to create repo").into_response(),
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
            // 🚀 Тут будет триггер для Spark!
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

/// Создаёт репо если не существует
async fn ensure_repo_exists(
    state: &Arc<AppState>,
    user: &str,
    repo: &str,
) -> Result<PathBuf, StatusCode> {
    let repo_path = PathBuf::from(&state.config.repo_storage)
        .join(user)
        .join(format!("{}.git", repo));

    if !repo_path.exists() {
        println!("Repo doesn't exist, creating...");

        // Получаем owner_id
        let owner_id: Option<uuid::Uuid> =
            sqlx::query_scalar!("SELECT id FROM users WHERE username = $1", user)
                .fetch_optional(&state.db)
                .await
                .map_err(|e| {
                    eprintln!("DB error fetching user: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;

        let owner_id = match owner_id {
            Some(id) => id,
            None => {
                eprintln!("User '{}' not found", user);
                return Err(StatusCode::NOT_FOUND);
            }
        };

        // Создаём в БД
        sqlx::query!(
            "INSERT INTO repositories (name, owner_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
            repo,
            owner_id
        )
        .execute(&state.db)
        .await
        .map_err(|e| {
            eprintln!("DB error inserting repo: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        // Создаём папку
        tokio::fs::create_dir_all(&repo_path).await.map_err(|e| {
            eprintln!("Failed to create dir: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        // git init --bare
        let output = Command::new("git")
            .args(["init", "--bare"])
            .current_dir(&repo_path)
            .output()
            .await
            .map_err(|e| {
                eprintln!("Git init failed: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        if !output.status.success() {
            eprintln!(
                "Git init stderr: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    Ok(repo_path)
}

/// GET /:user/:repo/archive
/// Отдаёт tar.gz снимок HEAD (без истории .git)
pub async fn get_archive(
    Path((user, repo)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    // headers: axum::http::HeaderMap, // Если нужна авторизация - раскомменть
) -> impl IntoResponse {
    let repo_path = PathBuf::from(&state.config.repo_storage)
        .join(&user)
        .join(format!("{}.git", repo));

    if !repo_path.exists() {
        return (StatusCode::NOT_FOUND, "Repository not found").into_response();
    }

    println!("📦 Archiving {}/{}...", user, repo);

    // git archive --format=tar.gz HEAD
    let output = Command::new("git")
        .arg("archive")
        .arg("--format=tar.gz")
        .arg("HEAD") // Берём последний коммит
        .current_dir(&repo_path)
        .output()
        .await;

    match output {
        Ok(out) if out.status.success() => {
            (
                [
                    (header::CONTENT_TYPE, "application/gzip"),
                    // Подсказка браузеру/клиенту как назвать файл
                    (
                        header::CONTENT_DISPOSITION,
                        &format!("attachment; filename=\"{}-{}.tar.gz\"", repo, "HEAD"), // Можно версию добавить
                    ),
                ],
                out.stdout,
            )
                .into_response()
        }
        Ok(out) => {
            // ВОТ ЭТО ВАЖНО УВИДЕТЬ
            let err = String::from_utf8_lossy(&out.stderr);
            println!("Archive error: {}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Git error: {}", err),
            )
                .into_response()
        }
        Ok(out) => {
            eprintln!(
                "git archive error: {}",
                String::from_utf8_lossy(&out.stderr)
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Archive generation failed",
            )
                .into_response()
        }
        Err(e) => {
            eprintln!("spawn error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to run git").into_response()
        }
    }
}
