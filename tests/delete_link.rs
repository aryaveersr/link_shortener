mod utils;

use anyhow::Context;
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use serde_json::json;
use sqlx::{Pool, Sqlite};

#[sqlx::test]
async fn delete_link_entry_with_correct_code(pool: Pool<Sqlite>) -> anyhow::Result<()> {
    // # Arrange
    const SLUG: &str = "shortened-link";

    let client = Client::new();
    let url = utils::spawn_server(pool.clone()).await?;

    // # Act
    // Create a new link entry and get its code
    let post_res = client
        .post(url.join("/api/links")?)
        .json(&json!({"slug": SLUG, "href": "https://google.com/"}))
        .send()
        .await
        .context("Failed to execute request")?;

    #[derive(Deserialize)]
    struct ResponseBody {
        code: u32,
    }

    let post_res_status = post_res.status();
    let code = post_res.json::<ResponseBody>().await.unwrap().code;

    // Send a DELETE request to endpoint
    let response = client
        .delete(url.join("/api/links")?)
        .json(&json!({"slug": SLUG, "code": code}))
        .send()
        .await
        .context("Failed to execute request")?;

    // Get the record from database
    let record = sqlx::query!(r#"SELECT * FROM links WHERE slug = $1;"#, SLUG)
        .fetch_optional(&pool)
        .await?;

    // # Assert
    assert_eq!(post_res_status, StatusCode::OK);
    assert_eq!(response.status(), StatusCode::OK);
    assert!(record.is_none());

    Ok(())
}

#[sqlx::test]
async fn delete_does_not_delete_link_entry_with_incorrect_code(
    pool: Pool<Sqlite>,
) -> anyhow::Result<()> {
    // # Arrange
    const SLUG: &str = "shortened-link";

    let client = Client::new();
    let url = utils::spawn_server(pool.clone()).await?;

    // # Act
    // Create a new link entry and get its code
    let post_res = client
        .post(url.join("/api/links")?)
        .json(&json!({"slug": SLUG, "href": "https://google.com/"}))
        .send()
        .await
        .context("Failed to execute request")?;

    #[derive(Deserialize)]
    struct ResponseBody {
        code: u32,
    }

    let post_res_status = post_res.status();
    let code = post_res.json::<ResponseBody>().await.unwrap().code;
    // Create a valid but incorrect code
    let code_to_send = if code != 9999_9999 { code + 1 } else { 4 };

    // Send a DELETE request to endpoint
    let response = client
        .delete(url.join("/api/links")?)
        .json(&json!({"slug": SLUG, "code": code_to_send}))
        .send()
        .await
        .context("Failed to execute request")?;

    // Get the record from database
    let record = sqlx::query!(r#"SELECT * FROM links WHERE slug = $1;"#, SLUG)
        .fetch_optional(&pool)
        .await?;

    // # Assert
    assert_eq!(post_res_status, StatusCode::OK);
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert!(record.is_some());

    Ok(())
}
