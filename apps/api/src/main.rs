use axum::{Router, routing::{get, post}};
use std::sync::Arc;
use tokio::io::AsyncBufReadExt;

mod git;
mod handlers;

pub struct AppConfig {
    pub repo_storage: String,
    pub jwt_secret: String,
}

pub struct AppState {
    pub db: sqlx::PgPool,
    pub config: AppConfig,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:super_secret@localhost:5432/postgres".to_string());

    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "forge-dev-secret-change-in-production".to_string());

    let repo_storage = std::env::var("REPO_STORAGE")
        .unwrap_or_else(|_| "./repositories".to_string());

    let pool = sqlx::PgPool::connect(&db_url).await?;
    sqlx::migrate!().run(&pool).await?;

    let state = Arc::new(AppState {
        db: pool,
        config: AppConfig { repo_storage, jwt_secret },
    });

    // Console reader
    let console_state = state.clone();
    tokio::spawn(async move {
        let stdin = tokio::io::stdin();
        let reader = tokio::io::BufReader::new(stdin);
        let mut lines = reader.lines();
        println!("⚡ Forge console ready. Type 'help' for commands.");

        while let Ok(Some(line)) = lines.next_line().await {
            let cmd = line.trim().to_lowercase();
            match cmd.as_str() {
                "help" | "?" => {
                    println!("── Forge Console ──────────────");
                    println!("  help    – show this help");
                    println!("  stop    – stop the server");
                    println!("  status  – server status");
                    println!("  users   – list all users");
                    println!("  repos   – list all repos");
                    println!("  sync    – scan disk for repos");
                    println!("───────────────────────────────");
                }
                "stop" | "exit" | "quit" => {
                    println!("🛑 Shutting down...");
                    std::process::exit(0);
                }
                "status" => {
                    println!("🔥 Forge is running");
                    println!("  Host: localhost:8080");
                    println!("  Storage: {}", console_state.config.repo_storage);
                }
                "users" => {
                    let users = sqlx::query!("SELECT username, email, created_at FROM users ORDER BY created_at DESC")
                        .fetch_all(&console_state.db).await;
                    match users {
                        Ok(rows) => {
                            println!("── Users ({}) ──────────────", rows.len());
                            for u in rows {
                                println!("  {} | {} | {}", u.username, u.email, u.created_at.map(|t| t.to_string()).unwrap_or_default());
                            }
                        }
                        Err(e) => println!("Error: {}", e),
                    }
                }
                "repos" => {
                    let repos = sqlx::query!(
                        "SELECT r.name, u.username, r.is_private FROM repositories r JOIN users u ON r.owner_id = u.id ORDER BY r.created_at DESC"
                    ).fetch_all(&console_state.db).await;
                    match repos {
                        Ok(rows) => {
                            println!("── Repos ({}) ──────────────", rows.len());
                            for r in rows {
                                let vis = if r.is_private { "private" } else { "public" };
                                println!("  {}/{} [{}]", r.username, r.name, vis);
                            }
                        }
                        Err(e) => println!("Error: {}", e),
                    }
                }
                "sync" => {
                    println!("📂 Scanning repositories/...");
                    let path = std::path::Path::new(&console_state.config.repo_storage);
                    if let Ok(entries) = std::fs::read_dir(path) {
                        for entry in entries.flatten() {
                            let username = entry.file_name().to_string_lossy().to_string();
                            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                                if let Ok(repo_dir) = std::fs::read_dir(entry.path()) {
                                    for repo in repo_dir.flatten() {
                                        let repo_name = repo.file_name().to_string_lossy()
                                            .trim_end_matches(".git").to_string();
                                        let user_id = sqlx::query_scalar!("SELECT id FROM users WHERE username = $1", &username)
                                            .fetch_optional(&console_state.db).await;
                                        if let Ok(Some(uid)) = user_id {
                                            sqlx::query!(
                                                "INSERT INTO repositories (name, owner_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
                                                repo_name, uid
                                            ).execute(&console_state.db).await.ok();
                                            println!("  ✓ {}/{}", username, repo_name);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    println!("✅ Sync done");
                }
                "" => {}
                _ => println!("Unknown command: {}. Type 'help'", cmd),
            }
        }
    });

    let app = Router::new()
        .route("/api/health", get(|| async { "Forge is up" }))
        .nest("/api", handlers::api_router(state.clone()))
        .nest("/git", Router::new()
            .route("/:user/:repo/info/refs", get(git::handle_info_refs))
            .route("/:user/:repo/git-upload-pack", post(git::handle_upload_pack))
            .route("/:user/:repo/git-receive-pack", post(git::handle_receive_pack))
            .route("/:user/:repo/archive", get(git::get_archive)))
        .with_state(state)
        .layer(tower_http::cors::CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("localhost:8080").await?;
    println!("🔥 Forge glowing at 8080");
    axum::serve(listener, app).await?;
    Ok(())
}