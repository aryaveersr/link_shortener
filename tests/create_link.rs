mod utils;

use anyhow::Context;
use reqwest::{Client, StatusCode};
use serde_json::json;
use sqlx::{Pool, Sqlite};

#[sqlx::test]
async fn create_link_returns_200_for_valid_data_and_stores_it(
    pool: Pool<Sqlite>,
) -> anyhow::Result<()> {
    // # Arrange
    const SLUG: &'static str = "shortened-link";
    const HREF: &'static str = "https://google.com/";

    let url = utils::spawn_server(pool.clone()).await?;

    // # Act
    let response = Client::new()
        .post(url.join("/api/links/create")?)
        .json(&json!({
            "slug": SLUG,
            "href": HREF
        }))
        .send()
        .await
        .context("Failed to execute request")?;

    let record = sqlx::query!(r#"SELECT * FROM links WHERE slug = $1;"#, SLUG)
        .fetch_one(&pool)
        .await?;

    // # Assert
    assert_eq!(response.status(), StatusCode::OK, "Status code isn't 200");
    assert_eq!(record.href, HREF, "Record stored doesn't have same href");

    Ok(())
}

#[sqlx::test]
async fn create_link_returns_error_for_slug_already_used(pool: Pool<Sqlite>) -> anyhow::Result<()> {
    // # Arrange
    const SLUG: &'static str = "shortened-link";

    let url = utils::spawn_server(pool.clone()).await?;
    let client = Client::new();

    // # Act
    client
        .post(url.join("/api/links/create")?)
        .json(&json!({
            "slug": SLUG,
            "href": "https://google.com"
        }))
        .send()
        .await
        .context("Failed to execute request")?;

    let response = client
        .post(url.join("/api/links/create")?)
        .json(&json!({
            "slug": SLUG,
            "href": "https://github.com"
        }))
        .send()
        .await
        .context("Failed to execute request")?;

    // # Assert
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    Ok(())
}

#[sqlx::test]
async fn create_link_returns_error_for_invalid_data(pool: Pool<Sqlite>) -> anyhow::Result<()> {
    // # Arrange
    let url = utils::spawn_server(pool).await?;
    let client = Client::new();

    let test_cases = [
        json!({}),
        json!({"slug": ""}),
        json!({"href": ""}),
        json!({"slug": "", "href": "https://google.com"}),
        json!({"slug": "shortened-link-13", "href": ""}),
        json!({"slug": "", "href": ""}),
        json!({"slug": "shortened/link/12", "href": "https://google.com"}),
        json!({"slug": "shortened-link", "href": r#"javascript:alert("Hi")"#}),
    ];

    for json in test_cases {
        // # Act
        let response = client
            .post(url.join("/api/links/create")?)
            .json(&json)
            .send()
            .await
            .context("Failed to execute request")?;

        // # Assert
        assert!(
            response.status().is_client_error(),
            "Status code is {}",
            response.status()
        );
    }

    Ok(())
}
