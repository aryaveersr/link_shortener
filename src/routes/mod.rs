mod api;
mod to;

use crate::AppState;
use axum::{Router, routing::get};
use tower_http::services::ServeDir;

pub fn routes() -> Router<AppState> {
    // Serving static files
    let serve_dir = ServeDir::new("public");

    // Create router
    Router::new()
        .route("/to/{slug}", get(to::handler))
        .nest("/api", api::routes())
        .fallback_service(serve_dir)
}
