use std::sync::Arc;

use axum::{Extension, Router};
use tower_http::trace::TraceLayer;

use crate::AppState;

/**
 * 路由管理
 */
pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .layer(TraceLayer::new_for_http())
        .layer(Extension(app_state))
}
