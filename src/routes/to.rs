use axum::{extract::Path, http::StatusCode};

pub async fn handler(Path(_slug): Path<String>) -> StatusCode {
    StatusCode::OK
}
