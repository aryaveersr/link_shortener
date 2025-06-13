//! GET /api/health_check

use axum::http::StatusCode;

#[tracing::instrument]
pub async fn handler() -> StatusCode {
    StatusCode::OK
}
