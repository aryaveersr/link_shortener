mod utils;

use anyhow::Context;
use reqwest::{Client, StatusCode};
use sqlx::{Pool, Sqlite};

#[sqlx::test]
async fn health_check_works(pool: Pool<Sqlite>) -> anyhow::Result<()> {
    // Arrange
    let url = utils::spawn_server(pool).await?;

    // Act
    let response = Client::new()
        .get(url.path("/api/health_check")?)
        .send()
        .await
        .context("Failed to execute request")?;

    // Assert
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.content_length(), Some(0));

    Ok(())
}
