use crate::{
    AppState,
    domain::{Href, LinkEntry, Slug},
};
use anyhow::{Context, anyhow};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
};
use sqlx::{Pool, Sqlite};

#[derive(thiserror::Error, Debug)]
pub enum ResponseError {
    #[error("{0}")]
    ValidationError(String),

    #[error("Requested slug does not exist")]
    NotFound,

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl IntoResponse for ResponseError {
    fn into_response(self) -> Response {
        match self {
            Self::ValidationError(_) => StatusCode::BAD_REQUEST,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
        .into_response()
    }
}

pub async fn handler(
    State(AppState { pool }): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Redirect, ResponseError> {
    let parsed_slug = Slug::parse(slug).map_err(ResponseError::ValidationError)?;
    let link_entry = fetch_slug(&pool, parsed_slug).await?;

    Ok(Redirect::temporary(link_entry.href.as_ref()))
}

async fn fetch_slug(pool: &Pool<Sqlite>, slug: Slug) -> Result<LinkEntry, ResponseError> {
    let slug_ref = slug.as_ref();

    let record = sqlx::query!("SELECT * FROM links WHERE slug = $1;", slug_ref)
        .fetch_optional(pool)
        .await
        .context("Failed to execute query")?;

    match record {
        Some(rec) => {
            let href = Href::parse(&rec.href)
                .map_err(|err| ResponseError::UnexpectedError(anyhow!(err)))?;

            Ok(LinkEntry { href, slug })
        }

        None => Err(ResponseError::NotFound),
    }
}
