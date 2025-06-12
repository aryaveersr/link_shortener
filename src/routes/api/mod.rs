mod health_check;
mod links;

use crate::AppState;
use axum::{Router, routing::get};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/health_check", get(health_check::handler))
        .nest("/links", links::routes())
}
