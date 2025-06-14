mod check;
mod create;

pub(self) use check::check_if_slug_already_exists;

use crate::AppState;
use axum::{Router, routing::post};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/create", post(create::handler))
        .route("/check", post(check::handler))
}
