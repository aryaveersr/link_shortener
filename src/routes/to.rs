use crate::{
    AppState,
    domain::{Href, LinkEntry, Slug},
    routes::_404::HtmlTemplate,
};
use anyhow::{Context, anyhow};
use askama::Template;
use axum::{
    extract::{Path, Request, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
};
use sqlx::{Pool, Sqlite};
use tracing::{debug, error};

#[derive(thiserror::Error, Debug)]
pub enum ResponseError {
    #[error("Requested slug does not exist")]
    NotFound(String),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl IntoResponse for ResponseError {
    fn into_response(self) -> Response {
        match self {
            Self::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),

            Self::NotFound(path) => {
                debug!(path, "Rendering 404.html");

                let template = HtmlTemplate { path };

                match template.render() {
                    Ok(html) => (StatusCode::NOT_FOUND, Html(html)).into_response(),
                    Err(err) => {
                        error!(?err, "Template rendering failed");

                        StatusCode::INTERNAL_SERVER_ERROR.into_response()
                    }
                }
            }
        }
    }
}

#[tracing::instrument(skip(pool, request))]
pub async fn handler(
    State(AppState { pool }): State<AppState>,
    Path(slug): Path<String>,
    request: Request,
) -> Result<Redirect, ResponseError> {
    let path = request.uri().path();

    let parsed_slug = Slug::parse(slug).map_err(|_| ResponseError::NotFound(path.into()))?;

    match fetch_slug(&pool, parsed_slug).await? {
        Some(link_entry) => Ok(Redirect::temporary(link_entry.href.as_ref())),
        None => Err(ResponseError::NotFound(path.into())),
    }
}

#[tracing::instrument(skip(pool))]
async fn fetch_slug(pool: &Pool<Sqlite>, slug: Slug) -> anyhow::Result<Option<LinkEntry>> {
    let slug_ref = slug.as_ref();

    let record = sqlx::query!("SELECT * FROM links WHERE slug = $1;", slug_ref)
        .fetch_optional(pool)
        .await
        .context("Failed to execute query")?;

    match record {
        Some(rec) => {
            let href = Href::parse(&rec.href).map_err(|err| anyhow!(err))?;

            Ok(Some(LinkEntry { href, slug }))
        }
        None => Ok(None),
    }
}
