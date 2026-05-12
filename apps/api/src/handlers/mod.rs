pub mod auth;
pub mod collaborators;
pub mod middleware;
pub mod repo;
pub mod users;

use crate::AppState;
use axum::{Router, middleware::from_fn_with_state};
use std::sync::Arc;

pub fn api_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    let protected = Router::new()
        .nest("/repos", repo::repo_router())
        .nest("/users", users::users_router())
        .nest("/collaborators", collaborators::collaborators_router())
        .layer(from_fn_with_state(
            state.clone(),
            middleware::require_jwt,
        ));

    Router::new()
        .nest("/auth", auth::auth_router())
        .merge(protected)
}