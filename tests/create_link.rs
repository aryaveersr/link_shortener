mod utils;

use anyhow::Context;
use reqwest::{Client, StatusCode};
use sqlx::{Pool, Sqlite};

#[sqlx::test]
async fn create_link_returns_200_for_valid_data(pool: Pool<Sqlite>) -> anyhow::Result<()> {
    // Arrange
    let url = utils::spawn_server(pool).await?;

    // Act
    let response = Client::new()
        .post(url.path("/api/links/create")?)
        .form(&[
            ("slug", "shortened-link-12"),
            ("href", "https://google.com"),
        ])
        .send()
        .await
        .context("Failed to execute request")?;

    // Assert
    assert_eq!(response.status(), StatusCode::OK, "status code not 200");
    assert_eq!(response.content_length(), Some(0), "content length not 0");

    Ok(())
}

#[sqlx::test]
async fn create_link_returns_400_for_invalid_data(pool: Pool<Sqlite>) -> anyhow::Result<()> {
    // Arrange
    let url = utils::spawn_server(pool).await?;
    let client = Client::new();

    let test_cases = [
        (vec![("slug", "")], "missing href"),
        (vec![("href", "")], "missing slug"),
        (vec![], "missing both href and slug"),
        (
            vec![
                ("slug", "shortened/link/12"),
                ("href", "https://google.com"),
            ],
            "slug contains forbidden characters",
        ),
        (
            vec![("slug", ""), ("href", "https://google.com")],
            "slug is empty",
        ),
        (
            vec![("slug", "shortened-link-13"), ("href", "")],
            "href is empty",
        ),
        (
            vec![("slug", ""), ("href", "")],
            "both slug and href are empty",
        ),
        (
            vec![
                ("slug", "shortened-link"),
                ("href", "javascript:alert(\"Hi\")"),
            ],
            "href is not a valid url",
        ),
    ];

    for case in test_cases {
        // Act
        let response = client
            .post(url.path("/api/links/create")?)
            .form(&case.0)
            .send()
            .await
            .context("Failed to execute request")?;

        // Assert
        assert!(
            response.status().is_client_error(),
            "status code is {} for: {}",
            response.status().as_str(),
            case.1
        );
    }

    Ok(())
}
