mod utils;

use anyhow::Context;
use reqwest::{Client, StatusCode, header};
use sqlx::{Pool, Sqlite};
use tracing::debug;

#[sqlx::test]
async fn _404_template_returned_for_invalid_path(pool: Pool<Sqlite>) -> anyhow::Result<()> {
    // # Arrange
    let url = utils::spawn_server(pool).await?;

    let test_cases = [
        "route-that-does-not-exist",
        "invalid<script>tag",
        // https://github.com/tower-rs/tower-http/issues/573
        "servedir%00",
    ];

    for case in test_cases {
        // # Act
        let response = Client::new()
            .get(url.join(case)?)
            .send()
            .await
            .context("Failed to execute request")?;

        // # Assert
        debug!(r#"Asserting for case: "{case}""#);

        assert!(response.content_length().unwrap() > 0);
        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let content_type = response
            .headers()
            .get(header::CONTENT_TYPE)
            .unwrap()
            .to_str()
            .unwrap();

        assert!(
            content_type.contains("text/html"),
            r#"Content type is "{content_type}""#
        );
    }

    Ok(())
}
