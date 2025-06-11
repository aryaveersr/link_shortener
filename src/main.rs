use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Listening on: http://{:?}", listener.local_addr().unwrap());

    axum::serve(listener, link_shortener::routes())
        .await
        .unwrap();
}
