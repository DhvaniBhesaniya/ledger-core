use axum::{extract::Request, middleware::Next, response::Response};
use std::time::Instant;
use tracing::{error, info, warn};

pub async fn logging_middleware(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let path = uri.path().to_string();
    let query = uri.query().map(|q| q.to_string());
    let start = Instant::now();

    // Log incoming request
    info!(
        method = %method,
        path = %path,
        query = ?query,
        "→ Incoming request"
    );

    let response = next.run(req).await;

    let elapsed = start.elapsed();
    let status = response.status();
    let status_code = status.as_u16();

    // Log based on response status
    match status_code {
        200..=299 => {
            info!(
                method = %method,
                path = %path,
                status = status_code,
                duration_ms = elapsed.as_millis(),
                "✓ Request completed successfully"
            );
        }
        400..=499 => {
            warn!(
                method = %method,
                path = %path,
                status = status_code,
                duration_ms = elapsed.as_millis(),
                "⚠ Client error"
            );
        }
        500..=599 => {
            error!(
                method = %method,
                path = %path,
                status = status_code,
                duration_ms = elapsed.as_millis(),
                "✗ Server error"
            );
        }
        _ => {
            info!(
                method = %method,
                path = %path,
                status = status_code,
                duration_ms = elapsed.as_millis(),
                "Request completed"
            );
        }
    }

    response
}
