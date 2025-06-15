use crate::{AppState, domain::Slug};
use anyhow::Context;
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};

#[derive(Deserialize, Debug)]
pub struct RequestBody {
    slug: String,
}

#[derive(Serialize)]
struct ResponseBody {
    exists: bool,
}

#[derive(thiserror::Error, Debug)]
pub enum ResponseError {
    #[error("{0}")]
    ValidationError(String),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl IntoResponse for ResponseError {
    fn into_response(self) -> Response {
        match self {
            Self::ValidationError(_) => Json(ResponseBody { exists: false }).into_response(),
            Self::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

#[tracing::instrument(skip(pool))]
pub async fn handler(
    State(AppState { pool }): State<AppState>,
    Json(request_body): Json<RequestBody>,
) -> Result<impl IntoResponse, ResponseError> {
    // Parse incoming request body.
    let slug = Slug::parse(request_body.slug).map_err(ResponseError::ValidationError)?;

    // Check if the slug exists in the database.
    let exists = check_if_slug_already_exists(&pool, &slug)
        .await
        .context("Failed to check for slug in database")?;

    Ok(Json(ResponseBody { exists }))
}

#[tracing::instrument(skip(pool))]
pub async fn check_if_slug_already_exists(
    pool: &Pool<Sqlite>,
    slug: &Slug,
) -> Result<bool, sqlx::Error> {
    let slug_ref = slug.as_ref();

    let record = sqlx::query!("SELECT * FROM links WHERE slug = $1;", slug_ref)
        .fetch_optional(pool)
        .await?;

    Ok(record.is_some())
}
