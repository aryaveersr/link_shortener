use link_shortener::AppState;
use sqlx::SqlitePool;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Init logging
    tracing_subscriber::fmt::init();

    // Setup database pool
    let pool = SqlitePool::connect(&dotenvy::var("DATABASE_URL")?).await?;

    // Setup listener on localhost
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    info!("Listening on: http://{:?}", listener.local_addr()?);

    // Start the server
    axum::serve(
        listener,
        link_shortener::routes().with_state(AppState { pool }),
    )
    .await?;

    Ok(())
}
