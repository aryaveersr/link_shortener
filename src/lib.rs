mod api;
mod domain;
mod telemetry;

use axum::{Router, middleware, routing::get};
use sqlx::{Pool, Sqlite};

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Sqlite>,
}

pub async fn init(pool: Pool<Sqlite>) -> anyhow::Result<Router> {
    // Init logging
    let _ = tracing_subscriber::fmt::try_init();

    // Build router
    let router = Router::new()
        .nest("/api", api::routes())
        .route("/", get(root))
        .with_state(AppState { pool })
        .layer(middleware::from_fn(telemetry::middleware));

    Ok(router)
}

async fn root() -> &'static str {
    "Hello, World!"
}
