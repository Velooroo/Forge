// re-import/export auth
pub mod auth;
pub mod middleware;
pub mod repo;

use crate::AppState;
use axum::Router;
use std::sync::Arc;

// Сделаем удобную функцию для объединения всех роутов API
pub fn api_router() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/auth", auth::auth_router())
        .nest("/repos", repo::repo_router())
}
