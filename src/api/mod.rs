mod health_check;

use axum::{Router, routing::get};

pub fn routes() -> Router {
    Router::new().route("/health_check", get(health_check::handler))
}
