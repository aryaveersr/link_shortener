mod api;
mod domain;
mod telemetry;

use axum::{Router, middleware};
use sqlx::{Pool, Sqlite};
use tower_http::services::ServeDir;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Sqlite>,
}

pub async fn init(pool: Pool<Sqlite>) -> anyhow::Result<Router> {
    // Init logging
    let _ = tracing_subscriber::fmt::try_init();

    // Serving static files
    let serve_dir = ServeDir::new("public");

    // Build router
    let router = Router::new()
        .nest("/api", api::routes())
        .fallback_service(serve_dir)
        .with_state(AppState { pool })
        .layer(middleware::from_fn(telemetry::middleware));

    Ok(router)
}
