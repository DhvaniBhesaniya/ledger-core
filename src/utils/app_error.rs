#![allow(dead_code)]
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub code: String,
}

#[derive(Debug)]
pub enum AppError {
    // 400
    BadRequest(String),

    // 401
    Unauthorized,
    InvalidApiKey,

    // 403
    Forbidden,

    // 404
    NotFound,
    AccountNotFound,
    TransactionNotFound,
    WebhookNotFound,

    // 409
    Conflict(String),
    InsufficientBalance,
    DuplicateIdempotencyKey,

    // 429
    RateLimitExceeded,

    // 500
    DatabaseError(String),
    InternalError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "BAD_REQUEST", msg),
            AppError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "UNAUTHORIZED",
                "Authentication required".to_string(),
            ),
            AppError::InvalidApiKey => (
                StatusCode::UNAUTHORIZED,
                "INVALID_API_KEY",
                "Invalid API key".to_string(),
            ),
            AppError::Forbidden => (
                StatusCode::FORBIDDEN,
                "FORBIDDEN",
                "You do not have permission to access this resource".to_string(),
            ),
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                "NOT_FOUND",
                "Resource not found".to_string(),
            ),
            AppError::AccountNotFound => (
                StatusCode::NOT_FOUND,
                "ACCOUNT_NOT_FOUND",
                "Account not found".to_string(),
            ),
            AppError::TransactionNotFound => (
                StatusCode::NOT_FOUND,
                "TRANSACTION_NOT_FOUND",
                "Transaction not found".to_string(),
            ),
            AppError::WebhookNotFound => (
                StatusCode::NOT_FOUND,
                "WEBHOOK_NOT_FOUND",
                "Webhook not found".to_string(),
            ),
            AppError::InsufficientBalance => (
                StatusCode::CONFLICT,
                "INSUFFICIENT_BALANCE",
                "Insufficient balance".to_string(),
            ),
            AppError::DuplicateIdempotencyKey => (
                StatusCode::CONFLICT,
                "DUPLICATE_REQUEST",
                "Duplicate request".to_string(),
            ),
            AppError::RateLimitExceeded => (
                StatusCode::TOO_MANY_REQUESTS,
                "RATE_LIMIT_EXCEEDED",
                "Too many requests".to_string(),
            ),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, "CONFLICT", msg),
            AppError::DatabaseError(ref msg) => {
                tracing::error!(error = %msg, "Database error occurred");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "DATABASE_ERROR",
                    msg.clone(),
                )
            }
            AppError::InternalError(ref msg) => {
                tracing::error!(error = %msg, "Internal server error occurred");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_ERROR",
                    msg.clone(),
                )
            }
        };

        // Log client and server errors
        if status.is_client_error() {
            tracing::warn!(code = code, message = %message, "Client error");
        } else if status.is_server_error() {
            tracing::error!(code = code, message = %message, "Server error");
        }

        let error_response = ErrorResponse {
            error: message,
            code: code.to_string(),
        };

        (status, Json(error_response)).into_response()
    }
}
