mod api;

use axum::{Router, routing::get};
use sqlx::{Pool, Sqlite};
#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Sqlite>,
}

pub async fn init(pool: Pool<Sqlite>) -> anyhow::Result<Router> {
    // Init logging
    tracing_subscriber::fmt::init();

    // Build router
    let router = Router::new()
        .nest("/api", api::routes())
        .route("/", get(root))
        .with_state(AppState { pool });

    Ok(router)
}

async fn root() -> &'static str {
    "Hello, World!"
}
