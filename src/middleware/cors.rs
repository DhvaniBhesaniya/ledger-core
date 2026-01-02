use axum::http::{Method, header};
use tower_http::cors::CorsLayer;

pub fn create_cors_layer() -> CorsLayer {
    let cors_origins_str = std::env::var("CORS_ALLOWED_ORIGINS")
        .unwrap_or_else(|_| "http://localhost:3000,http://localhost:8080".to_string());

    let cors_origins: Vec<header::HeaderValue> = cors_origins_str
        .split(',')
        .map(|s| {
            s.trim()
                .parse::<header::HeaderValue>()
                .expect("Invalid CORS origin")
        })
        .collect();

    CorsLayer::new()
        .allow_origin(cors_origins)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(tower_http::cors::Any)
}
