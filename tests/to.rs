mod utils;

use anyhow::Context;
use reqwest::{Client, StatusCode, redirect::Policy};
use sqlx::{Pool, Sqlite};
use std::collections::HashMap;

#[sqlx::test]
async fn to_redirects_to_href_for_valid_slug(pool: Pool<Sqlite>) -> anyhow::Result<()> {
    // Arrange
    const SLUG: &'static str = "shortened-link";
    const URL: &'static str = "https://google.com";
    let url = utils::spawn_server(pool).await?;
    let client = Client::builder().redirect(Policy::none()).build()?;

    // Act
    client
        .post(url.path("/api/links/create")?)
        .json(&HashMap::from([("slug", SLUG), ("href", URL)]))
        .send()
        .await
        .context("Failed to execute request")?;

    let response = client
        .get(url.path(&format!("/to/{}", SLUG))?)
        .send()
        .await
        .context("Failed to execute request")?;

    // Assert
    assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);
    assert!(response.headers().contains_key("Location"));

    let location_header_value = response
        .headers()
        .get("Location")
        .unwrap()
        .to_str()
        .unwrap();

    assert!(
        location_header_value.starts_with(URL),
        r#"Location header has value "{}""#,
        location_header_value
    );

    Ok(())
}

#[sqlx::test]
async fn to_returns_404_for_invalid_slug(pool: Pool<Sqlite>) -> anyhow::Result<()> {
    // Arrange
    let url = utils::spawn_server(pool).await?;

    // Act
    let response = Client::new()
        .get(url.path("/to/hello-world")?)
        .send()
        .await
        .context("Failed to execute request")?;

    // Assert
    assert_eq!(
        response.status(),
        StatusCode::NOT_FOUND,
        "Status code isn't 404"
    );

    Ok(())
}
