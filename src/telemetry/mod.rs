use axum::{extract::Request, middleware::Next, response::Response};
use std::time::Instant;
use tracing::{Instrument, debug, info_span};
use uuid::Uuid;

pub async fn middleware(request: Request, next: Next) -> Response {
    let start = Instant::now();

    let method = request.method();
    let uri = request.uri();
    let id = Uuid::new_v4();

    let span = info_span!("Request", %id, %method, %uri);

    async {
        debug!(target: "", "Started Processing Request");

        let response = next.run(request).await;

        let latency = format!("{}ms", start.elapsed().as_millis());
        let status = response.status();

        debug!(target: "", %latency, %status, "Finished Processing Request");

        response
    }
    .instrument(span)
    .await
}
