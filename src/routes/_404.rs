use askama::Template;
use axum::{
    extract::Request,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use tracing::{debug, error};

#[derive(Template)]
#[template(path = "404.html")]
struct HtmlTemplate {
    path: String,
}

#[tracing::instrument]
pub async fn handler(request: Request) -> Response {
    let path = request.uri().path();
    let template = HtmlTemplate { path: path.into() };

    debug!(path, "Rendering 404.html");

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(err) => {
            error!(?err, "Template rendering failed");

            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
