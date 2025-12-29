#![allow(dead_code)]
use axum::{extract::Request, middleware::Next, response::Response};
use std::time::Instant;
use tracing::info;

pub async fn logging_middleware(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let start = Instant::now();

    let response = next.run(req).await;

    let elapsed = start.elapsed();
    info!(
        method = %method,
        uri = %uri,
        status = response.status().as_u16(),
        duration_ms = elapsed.as_millis(),
        "Request completed"
    );

    response
}
