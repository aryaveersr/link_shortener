mod health_check;
mod links;

use crate::AppState;
use axum::{Router, routing::get};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/health_check", get(health_check::handler))
        .route("/links", links::method_routes())
}
