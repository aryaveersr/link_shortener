mod api;
mod to;

use crate::AppState;
use axum::{Router, routing::get};
use tower_http::services::{ServeDir, ServeFile};

pub fn routes() -> Router<AppState> {
    // Serving static files
    let serve_public_dir = ServeDir::new("public");
    let serve_404 = ServeFile::new("public/404.html");

    // Create router
    Router::new()
        .route("/to/{slug}", get(to::handler))
        .nest("/api", api::routes())
        .fallback_service(serve_public_dir.not_found_service(serve_404))
}
