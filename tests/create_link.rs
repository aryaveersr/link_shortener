mod utils;

use std::collections::HashMap;

use anyhow::Context;
use reqwest::{Client, StatusCode};
use sqlx::{Pool, Sqlite};

#[sqlx::test]
async fn create_link_returns_200_for_valid_data_and_stores_it(
    pool: Pool<Sqlite>,
) -> anyhow::Result<()> {
    // Arrange
    let url = utils::spawn_server(pool.clone()).await?;

    // Act
    let response = Client::new()
        .post(url.path("/api/links/create")?)
        .json(&HashMap::from([
            ("slug", "shortened-link-12"),
            ("href", "https://google.com"),
        ]))
        .send()
        .await
        .context("Failed to execute request")?;

    let record = sqlx::query!(r#"SELECT * FROM links WHERE slug = "shortened-link-12";"#)
        .fetch_one(&pool)
        .await?;

    // Assert
    assert_eq!(response.status(), StatusCode::OK, "status code not 200");
    assert_eq!(response.content_length(), Some(0), "content length not 0");

    assert_eq!(record.href, "https://google.com");

    Ok(())
}

#[sqlx::test]
async fn create_link_returns_error_for_slug_already_used(pool: Pool<Sqlite>) -> anyhow::Result<()> {
    // Arrange
    let url = utils::spawn_server(pool.clone()).await?;
    let client = Client::new();

    // Act
    let response_a = client
        .post(url.path("/api/links/create")?)
        .json(&HashMap::from([
            ("slug", "shortened-link"),
            ("href", "https://google.com"),
        ]))
        .send()
        .await
        .context("Failed to execute request")?;

    let response_b = client
        .post(url.path("/api/links/create")?)
        .json(&HashMap::from([
            ("slug", "shortened-link"),
            ("href", "https://github.com"),
        ]))
        .send()
        .await
        .context("Failed to execute request")?;

    // Assert
    assert_eq!(response_a.status(), StatusCode::OK, "status code not 200");
    assert!(
        response_b.status().is_client_error(),
        "status code is {}",
        response_b.status()
    );

    Ok(())
}

#[sqlx::test]
async fn create_link_returns_error_for_invalid_data(pool: Pool<Sqlite>) -> anyhow::Result<()> {
    // Arrange
    let url = utils::spawn_server(pool).await?;
    let client = Client::new();

    let test_cases = [
        (HashMap::from([("slug", "")]), "missing href"),
        (HashMap::from([("href", "")]), "missing slug"),
        (HashMap::new(), "missing both href and slug"),
        (
            HashMap::from([("slug", ""), ("href", "https://google.com")]),
            "slug is empty",
        ),
        (
            HashMap::from([("slug", "shortened-link-13"), ("href", "")]),
            "href is empty",
        ),
        (
            HashMap::from([("slug", ""), ("href", "")]),
            "both slug and href are empty",
        ),
        (
            HashMap::from([
                ("slug", "shortened/link/12"),
                ("href", "https://google.com"),
            ]),
            "slug contains forbidden characters",
        ),
        (
            HashMap::from([
                ("slug", "shortened-link"),
                ("href", "javascript:alert(\"Hi\")"),
            ]),
            "href is not a valid url",
        ),
    ];

    for case in test_cases {
        // Act
        let response = client
            .post(url.path("/api/links/create")?)
            .json(&case.0)
            .send()
            .await
            .context("Failed to execute request")?;

        // Assert
        assert!(
            response.status().is_client_error(),
            "status code is {} for: {}",
            response.status(),
            case.1
        );
    }

    Ok(())
}
