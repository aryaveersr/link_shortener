use crate::{
    AppState,
    domain::{Href, LinkEntry, Slug},
};
use anyhow::Context;
use axum::{
    Form,
    body::Body,
    extract::State,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use serde::Deserialize;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FormData {
    slug: String,
    href: String,
}

impl TryFrom<FormData> for LinkEntry {
    type Error = String;

    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let href = Href::parse(value.href)?;
        let slug = Slug::parse(value.slug)?;

        Ok(Self { href, slug })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum CreateError {
    #[error("{0}")]
    ValidationError(String),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl IntoResponse for CreateError {
    fn into_response(self) -> Response<Body> {
        match self {
            CreateError::ValidationError(_) => StatusCode::BAD_REQUEST,
            CreateError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
        .into_response()
    }
}

pub async fn handler(
    State(AppState { pool }): State<AppState>,
    Form(form_data): Form<FormData>,
) -> Result<StatusCode, CreateError> {
    // Parse incoming form data.
    let link_entry: LinkEntry = form_data.try_into().map_err(CreateError::ValidationError)?;

    // Insert the link entry into database.
    insert_link_entry(&pool, link_entry)
        .await
        .context("Failed to insert the link entry")?;

    Ok(StatusCode::OK)
}

async fn insert_link_entry(pool: &Pool<Sqlite>, link_entry: LinkEntry) -> Result<(), sqlx::Error> {
    let id = Uuid::new_v4();

    sqlx::query("INSERT INTO links (id, slug, href) VALUES ($1, $2, $3);")
        .bind(id.to_string())
        .bind(link_entry.slug.as_ref())
        .bind(link_entry.href.as_ref())
        .execute(pool)
        .await?;

    Ok(())
}
