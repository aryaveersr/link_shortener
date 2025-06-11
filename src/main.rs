use sqlx::SqlitePool;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create database pool
    let pool = SqlitePool::connect(&dotenvy::var("DATABASE_URL")?).await?;

    // Init application
    let router = link_shortener::init(pool).await?;

    // Setup listener on localhost
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    info!("Listening on: http://{:?}", listener.local_addr()?);

    // Start the server
    axum::serve(listener, router).await?;

    Ok(())
}
