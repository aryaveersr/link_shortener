mod utils;

use anyhow::Context;
use reqwest::{Client, StatusCode, redirect::Policy};
use serde_json::json;
use sqlx::{Pool, Sqlite};

#[sqlx::test]
async fn to_redirects_to_href_for_slug_that_exists(pool: Pool<Sqlite>) -> anyhow::Result<()> {
    // # Arrange
    const SLUG: &str = "shortened-link";
    const HREF: &str = "https://google.com/";

    // Set redirect policy to none since we want to assert that they happen.
    let client = Client::builder().redirect(Policy::none()).build()?;
    let url = utils::spawn_server(pool).await?;

    // # Act
    // Create a new link
    client
        .post(url.join("/api/links/create")?)
        .json(&json!({"slug": SLUG, "href": HREF}))
        .send()
        .await
        .context("Failed to execute request")?;

    // Send a GET request to the new link
    let response = client
        .get(url.join("/to/")?.join(SLUG)?)
        .send()
        .await
        .context("Failed to execute request")?;

    // # Assert
    assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);
    assert_eq!(response.headers().get("Location").unwrap(), HREF,);

    Ok(())
}

#[sqlx::test]
async fn to_returns_404_for_slug_that_does_not_exist(pool: Pool<Sqlite>) -> anyhow::Result<()> {
    // # Arrange
    let url = utils::spawn_server(pool).await?;

    // # Act
    let response = Client::new()
        .get(url.join("/to/hello-world")?)
        .send()
        .await
        .context("Failed to execute request")?;

    // # Assert
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    Ok(())
}
