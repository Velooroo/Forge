use axum::{
    Router, middleware,
    routing::{get, post},
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

mod git;
mod handlers;

pub struct AppConfig {
    pub repo_storage: String,
}

pub struct AppState {
    pub db: sqlx::PgPool,
    pub config: AppConfig,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Инициализация (в будущем из .env)
    let db_url = "postgres://postgres:super_secret@localhost:5432/postgres";
    let pool = sqlx::PgPool::connect(db_url).await?;

    // 2. Мигрирование всех скриптов
    sqlx::migrate!().run(&pool).await?;

    // 3. Инициализация состояния приложения
    let state = Arc::new(AppState {
        db: pool,
        config: AppConfig {
            repo_storage: "./repositories".to_string(),
        },
    });

    // 4. Роутинг
    let app = Router::new()
        .route("/api/health", get(|| async { "Forge is up" }))
        // Публичное API (auth)
        .nest("/api/auth", handlers::auth::auth_router())
        // Защищённое API (repos)
        .nest(
            "/api/repos",
            handlers::repo::repo_router().layer(middleware::from_fn_with_state(
                state.clone(),
                handlers::middleware::require_auth,
            )),
        )
        // Git роуты (с auth)
        .nest(
            "/git",
            Router::new()
                .route("/:user/:repo/info/refs", get(git::handle_info_refs))
                .route(
                    "/:user/:repo/git-upload-pack",
                    post(git::handle_upload_pack),
                )
                .route(
                    "/:user/:repo/git-receive-pack",
                    post(git::handle_receive_pack),
                )
                .route("/:user/:repo/archive", get(git::get_archive))
                .layer(middleware::from_fn_with_state(
                    state.clone(),
                    handlers::middleware::require_auth,
                )),
        )
        .with_state(state)
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("localhost:8080").await?;
    println!("🔥 Forge glowing at 8080");
    axum::serve(listener, app).await?;

    Ok(())
}
