mod utils;

use anyhow::Context;
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use serde_json::json;
use sqlx::{Pool, Sqlite};

#[sqlx::test]
async fn get_returns_true_for_slug_that_exists(pool: Pool<Sqlite>) -> anyhow::Result<()> {
    // # Arrange
    const SLUG: &str = "shortened-link";

    let client = Client::new();
    let url = utils::spawn_server(pool).await?;

    // # Act
    // Create a new link
    client
        .post(url.join("/api/links")?)
        .json(&json!({"slug": SLUG, "href": "https://google.com/"}))
        .send()
        .await
        .context("Failed to execute request")?;

    let response = client
        .get(url.join("/api/links")?)
        .json(&json!({"slug": SLUG}))
        .send()
        .await
        .context("Failed to execute request")?;

    // # Assert
    assert_eq!(response.status(), StatusCode::OK);

    #[derive(Deserialize)]
    struct ResponseBody {
        exists: bool,
    }

    let body: ResponseBody = response.json().await.unwrap();

    assert!(body.exists);

    Ok(())
}

#[sqlx::test]
async fn get_returns_false_for_slug_that_does_not_exist(pool: Pool<Sqlite>) -> anyhow::Result<()> {
    // # Arrange
    let url = utils::spawn_server(pool).await?;

    // # Act
    let response = Client::new()
        .get(url.join("/api/links")?)
        .json(&json!({"slug": "shortened-link"}))
        .send()
        .await
        .context("Failed to execute request")?;

    // # Assert
    assert_eq!(response.status(), StatusCode::OK);

    #[derive(Deserialize)]
    struct ResponseBody {
        exists: bool,
    }

    let body: ResponseBody = response.json().await.unwrap();

    assert!(!body.exists);

    Ok(())
}
