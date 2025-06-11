use crate::AppState;
use axum::{Router, http::StatusCode, response::IntoResponse, routing::post};

async fn create_link() -> impl IntoResponse {
    StatusCode::OK
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/create", post(create_link))
}
