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
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct RequestBody {
    slug: String,
    href: String,
}

impl TryFrom<RequestBody> for LinkEntry {
    type Error = String;

    fn try_from(value: RequestBody) -> Result<Self, Self::Error> {
        let href = Href::parse(&value.href)?;
        let slug = Slug::parse(value.slug)?;
        let code = Code::generate();

        Ok(Self { href, slug, code })
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
        #[derive(Serialize, Debug, Clone)]
        struct ErrorBody {
            err: String,
        }

        match self {
            Self::AlreadyExists => StatusCode::CONFLICT.into_response(),
            Self::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            Self::ValidationError(err) => {
                (StatusCode::BAD_REQUEST, Json(ErrorBody { err })).into_response()
            }
        }
    }
}

#[derive(Serialize)]
pub struct ResponseBody {
    code: u32,
}

#[tracing::instrument(skip(pool))]
pub async fn handler(
    State(AppState { pool }): State<AppState>,
    Json(request_body): Json<RequestBody>,
) -> Result<Json<ResponseBody>, ResponseError> {
    debug!("Creating a new link entry");

    // Parse incoming request body.
    let link_entry: LinkEntry = request_body
        .try_into()
        .map_err(ResponseError::ValidationError)?;

    // Check if the slug already exists in the database.
    if super::_get::check_if_slug_already_exists(&pool, &link_entry.slug)
        .await
        .context("Failed to check for slug in database")?
    {
        return Err(ResponseError::AlreadyExists);
    }

    // Insert the link entry into database.
    insert_link_entry(&pool, &link_entry)
        .await
        .context("Failed to insert the link entry")?;

    Ok(Json(ResponseBody {
        code: link_entry.code.as_u32(),
    }))
}

#[tracing::instrument(skip(pool))]
async fn insert_link_entry(pool: &Pool<Sqlite>, link_entry: &LinkEntry) -> Result<(), sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let slug = link_entry.slug.as_ref();
    let href = link_entry.href.as_ref();
    let code = link_entry.code.as_u32();

    sqlx::query!(
        "INSERT INTO links (id, slug, href, code) VALUES ($1, $2, $3, $4);",
        id,
        slug,
        href,
        code
    )
    .execute(pool)
    .await?;

    Ok(())
}
