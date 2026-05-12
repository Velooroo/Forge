use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;

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
        config: AppConfig {
            repo_storage,
            jwt_secret,
        },
    });

    let app = Router::new()
        .route("/api/health", get(|| async { "Forge is up" }))
        .nest("/api", handlers::api_router(state.clone()))
        .nest(
            "/git",
            Router::new()
                .route("/:user/:repo/info/refs", get(git::handle_info_refs))
                .route("/:user/:repo/git-upload-pack", post(git::handle_upload_pack))
                .route("/:user/:repo/git-receive-pack", post(git::handle_receive_pack))
                .route("/:user/:repo/archive", get(git::get_archive)),
        )
        .with_state(state)
        .layer(tower_http::cors::CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("localhost:8080").await?;
    println!("🔥 Forge glowing at 8080");
    axum::serve(listener, app).await?;

    Ok(())
}