use sqlx::{Pool, Sqlite};
use tokio::net::TcpListener;
use url::Url;

pub async fn spawn_server(pool: Pool<Sqlite>) -> anyhow::Result<Url> {
    // Init application
    let router = link_shortener::init(pool).await?;

    // Create listener
    // Using `0` as the port means we leave it up to the OS to assign us any available port.
    let listener = TcpListener::bind("localhost:0").await.unwrap();
    let addr = listener.local_addr()?;

    // Start the server
    tokio::spawn(async move {
        axum::serve(listener, router).await.unwrap();
    });

    Ok(Url::parse(&format!("http://localhost:{}/", addr.port()))?)
}
