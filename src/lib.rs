mod api;

use axum::{Router, routing::get};

async fn root() -> &'static str {
    "Hello, World!"
}

pub fn routes() -> Router {
    Router::new()
        .nest("/api", api::routes())
        .route("/", get(root))
}
