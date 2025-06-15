use crate::{
    AppState,
    domain::{Code, Href, LinkEntry, Slug},
};
use anyhow::Context;
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use tracing::debug;

#[derive(Deserialize, Debug)]
pub struct RequestBody {
    slug: String,
    code: u32,
    // The new href
    href: String,
}

impl TryFrom<RequestBody> for LinkEntry {
    type Error = String;

    fn try_from(value: RequestBody) -> Result<Self, Self::Error> {
        let href = Href::parse(&value.href)?;
        let slug = Slug::parse(value.slug)?;
        let code = Code::parse(value.code)?;

        Ok(Self { href, slug, code })
    }
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
        #[derive(Serialize, Debug, Clone)]
        struct ErrorBody {
            err: String,
        }

        match self {
            Self::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            Self::ValidationError(err) => {
                (StatusCode::BAD_REQUEST, Json(ErrorBody { err })).into_response()
            }
        }
    }
}

#[tracing::instrument(skip(pool))]
pub async fn handler(
    State(AppState { pool }): State<AppState>,
    Json(request_body): Json<RequestBody>,
) -> Result<StatusCode, ResponseError> {
    debug!("Updating a link entry");

    // Parse incoming request body.
    let link_entry: LinkEntry = request_body
        .try_into()
        .map_err(ResponseError::ValidationError)?;

    // Update the link entry in the database.
    let entry_existed = update_link_entry(&pool, &link_entry)
        .await
        .context("Failed to update the link entry")?;

    if entry_existed {
        Ok(StatusCode::OK)
    } else {
        Err(ResponseError::ValidationError(
            "Slug or code incorrect".into(),
        ))
    }
}

#[tracing::instrument(skip(pool))]
async fn update_link_entry(
    pool: &Pool<Sqlite>,
    link_entry: &LinkEntry,
) -> Result<bool, sqlx::Error> {
    let slug = link_entry.slug.as_ref();
    let href = link_entry.href.as_ref();
    let code = link_entry.code.as_u32();

    let rows_affected = sqlx::query!(
        "UPDATE links SET href = $1 WHERE slug = $2 AND code = $3;",
        href,
        slug,
        code
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_affected != 0)
}
