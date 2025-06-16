use crate::{
    AppState,
    domain::{Code, Slug},
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
}

impl TryFrom<RequestBody> for (Slug, Code) {
    type Error = String;

    fn try_from(value: RequestBody) -> Result<Self, Self::Error> {
        let slug = Slug::parse(value.slug)?;
        let code = Code::parse(value.code)?;

        Ok((slug, code))
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
    debug!("Deleting a link entry");

    // Parse incoming request body.
    let (slug, code) = request_body
        .try_into()
        .map_err(ResponseError::ValidationError)?;

    // Delete the link entry in the database.
    let entry_existed = delete_link_entry(&pool, &slug, &code)
        .await
        .context("Failed to delete the link entry")?;

    if entry_existed {
        Ok(StatusCode::OK)
    } else {
        Err(ResponseError::ValidationError(
            "Slug or code incorrect".into(),
        ))
    }
}

#[tracing::instrument(skip(pool))]
async fn delete_link_entry(
    pool: &Pool<Sqlite>,
    slug: &Slug,
    code: &Code,
) -> Result<bool, sqlx::Error> {
    let slug = slug.as_ref();
    let code = code.as_u32();

    let rows_affected = sqlx::query!(
        "DELETE FROM links WHERE slug = $1 AND code = $2;",
        slug,
        code
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_affected != 0)
}
