mod create;

use crate::AppState;
use axum::{Router, routing::post};

pub fn routes() -> Router<AppState> {
    Router::new().route("/create", post(create::handler))
}
