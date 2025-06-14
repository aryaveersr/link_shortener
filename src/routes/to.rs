use crate::{AppState, domain::Slug};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
};
use sqlx::{Pool, Sqlite};
use tracing::error;

#[derive(thiserror::Error, Debug)]
pub enum ResponseError {
    #[error("Requested slug does not exist")]
    NotFound,

    #[error(transparent)]
    UnexpectedError(#[from] sqlx::Error),
}

impl IntoResponse for ResponseError {
    fn into_response(self) -> Response {
        match self {
            Self::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            Self::NotFound => (
                StatusCode::NOT_FOUND,
                Html(include_str!("../../public/404.html")),
            )
                .into_response(),
        }
    }
}

#[tracing::instrument(skip(pool))]
pub async fn handler(
    State(AppState { pool }): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Redirect, ResponseError> {
    let slug = Slug::parse(slug).or(Err(ResponseError::NotFound))?;

    match fetch_href_for_slug(&pool, &slug).await? {
        Some(href) => Ok(Redirect::temporary(&href)),
        None => Err(ResponseError::NotFound),
    }
}

#[tracing::instrument(skip(pool))]
async fn fetch_href_for_slug(
    pool: &Pool<Sqlite>,
    slug: &Slug,
) -> Result<Option<String>, sqlx::Error> {
    let slug_ref = slug.as_ref();

    let record = sqlx::query!("SELECT * FROM links WHERE slug = $1;", slug_ref)
        .fetch_optional(pool)
        .await?;

    Ok(record.map(|r| r.href))
}
