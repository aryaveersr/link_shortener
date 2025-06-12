mod utils;

use anyhow::Context;
use reqwest::{Client, StatusCode};
use sqlx::{Pool, Sqlite};
use std::collections::HashMap;

#[sqlx::test]
async fn create_link_returns_200_for_valid_data_and_stores_it(
    pool: Pool<Sqlite>,
) -> anyhow::Result<()> {
    // Arrange
    const SLUG: &'static str = "shortened-link";
    let url = utils::spawn_server(pool.clone()).await?;

    // Act
    let response = Client::new()
        .post(url.path("/api/links/create")?)
        .json(&HashMap::from([
            ("slug", SLUG),
            ("href", "https://google.com"),
        ]))
        .send()
        .await
        .context("Failed to execute request")?;

    let record = sqlx::query!(r#"SELECT * FROM links WHERE slug = $1;"#, SLUG)
        .fetch_one(&pool)
        .await?;

    // Assert
    assert_eq!(response.status(), StatusCode::OK, "Status code isn't 200");
    assert_eq!(response.content_length(), Some(0), "Content length isn't 0");
    assert!(
        record.href.starts_with("https://google.com"),
        "Record stored doesn't have same href"
    );

    Ok(())
}

#[sqlx::test]
async fn create_link_returns_error_for_slug_already_used(pool: Pool<Sqlite>) -> anyhow::Result<()> {
    // Arrange
    const SLUG: &'static str = "shortened-link";
    let url = utils::spawn_server(pool.clone()).await?;
    let client = Client::new();

    // Act
    let response_a = client
        .post(url.path("/api/links/create")?)
        .json(&HashMap::from([
            ("slug", SLUG),
            ("href", "https://google.com"),
        ]))
        .send()
        .await
        .context("Failed to execute request")?;

    let response_b = client
        .post(url.path("/api/links/create")?)
        .json(&HashMap::from([
            ("slug", SLUG),
            ("href", "https://github.com"),
        ]))
        .send()
        .await
        .context("Failed to execute request")?;

    // Assert
    assert_eq!(response_a.status(), StatusCode::OK, "Status code isn't 200");
    assert!(
        response_b.status().is_client_error(),
        "Status code is {}",
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
        (vec![("slug", "")], "Missing `href`"),
        (vec![("href", "")], "Missing `slug`"),
        (vec![], "Missing both `href` and `slug`"),
        (
            vec![("slug", ""), ("href", "https://google.com")],
            "`slug` is empty",
        ),
        (
            vec![("slug", "shortened-link-13"), ("href", "")],
            "`href` is empty",
        ),
        (
            vec![("slug", ""), ("href", "")],
            "Both `slug` and `href` are empty",
        ),
        (
            vec![
                ("slug", "shortened/link/12"),
                ("href", "https://google.com"),
            ],
            "`slug` has invalid characters",
        ),
        (
            vec![
                ("slug", "shortened-link"),
                ("href", "javascript:alert(\"Hi\")"),
            ],
            "`href` is not a valid url",
        ),
    ]
    .map(|(pairs, err)| {
        let mut map = HashMap::new();

        for (k, v) in pairs {
            map.insert(k, v);
        }

        (map, err)
    });

    for (body, err) in test_cases {
        // Act
        let response = client
            .post(url.path("/api/links/create")?)
            .json(&body)
            .send()
            .await
            .context("Failed to execute request")?;

        // Assert
        assert!(
            response.status().is_client_error(),
            "status code is {} for: {err}",
            response.status()
        );
    }

    Ok(())
}
