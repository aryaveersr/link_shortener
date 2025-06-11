use axum::Router;
use reqwest::Url;
use sqlx::{Pool, Sqlite};
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub struct UrlHelper {
    base: Url,
}

impl UrlHelper {
    pub fn path(&self, input: &str) -> anyhow::Result<Url> {
        Ok(self.base.join(input)?)
    }
}

impl TryFrom<SocketAddr> for UrlHelper {
    type Error = anyhow::Error;

    fn try_from(value: SocketAddr) -> anyhow::Result<Self> {
        Ok(Self {
            base: Url::parse(&format!("http://localhost:{}", value.port()))?,
        })
    }
}

async fn start(listener: TcpListener, router: Router) {
    axum::serve(listener, router).await.unwrap();
}

pub async fn spawn_server(pool: Pool<Sqlite>) -> anyhow::Result<UrlHelper> {
    // Init application
    let router = link_shortener::init(pool).await?;

    // Create listener
    let listener = TcpListener::bind("localhost:0").await.unwrap();
    let addr = listener.local_addr()?;

    // Start the server
    tokio::spawn(start(listener, router));

    Ok(addr.try_into()?)
}
