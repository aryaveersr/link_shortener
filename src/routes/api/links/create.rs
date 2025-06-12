//! POST /api/links/create

use crate::{
    AppState,
    domain::{Href, LinkEntry, Slug},
};
use anyhow::Context;
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct RequestData {
    slug: String,
    href: String,
}

impl TryFrom<RequestData> for LinkEntry {
    type Error = String;

    fn try_from(value: RequestData) -> Result<Self, Self::Error> {
        let href = Href::parse(&value.href)?;
        let slug = Slug::parse(value.slug)?;

        Ok(Self { href, slug })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ResponseError {
    #[error("{0}")]
    ValidationError(String),

    #[error("Requested slug already exists")]
    AlreadyExists,

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl IntoResponse for ResponseError {
    fn into_response(self) -> Response {
        match self {
            Self::ValidationError(_) | Self::AlreadyExists => StatusCode::BAD_REQUEST,
            Self::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
        .into_response()
    }
}

pub async fn handler(
    State(AppState { pool }): State<AppState>,
    Json(request_data): Json<RequestData>,
) -> Result<StatusCode, ResponseError> {
    // Parse incoming request data.
    let link_entry: LinkEntry = request_data
        .try_into()
        .map_err(ResponseError::ValidationError)?;

    // Check if the slug already exists in the database.
    if check_if_slug_exists(&pool, &link_entry.slug)
        .await
        .context("Failed to check for slug in database")?
    {
        return Err(ResponseError::AlreadyExists);
    }

    // Insert the link entry into database.
    insert_link_entry(&pool, &link_entry)
        .await
        .context("Failed to insert the link entry")?;

    Ok(StatusCode::OK)
}

async fn check_if_slug_exists(pool: &Pool<Sqlite>, slug: &Slug) -> Result<bool, sqlx::Error> {
    let slug_ref = slug.as_ref();

    let record = sqlx::query!("SELECT * FROM links WHERE slug = $1;", slug_ref)
        .fetch_optional(pool)
        .await?;

    Ok(record.is_some())
}

async fn insert_link_entry(pool: &Pool<Sqlite>, link_entry: &LinkEntry) -> Result<(), sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let slug = link_entry.slug.as_ref();
    let href = link_entry.href.to_string();

    sqlx::query!(
        "INSERT INTO links (id, slug, href) VALUES ($1, $2, $3);",
        id,
        slug,
        href
    )
    .execute(pool)
    .await?;

    Ok(())
}
