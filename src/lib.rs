mod api;

use axum::{Router, routing::get};
use sqlx::{Pool, Sqlite};

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Sqlite>,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .nest("/api", api::routes())
        .route("/", get(root))
}

async fn root() -> &'static str {
    "Hello, World!"
}
