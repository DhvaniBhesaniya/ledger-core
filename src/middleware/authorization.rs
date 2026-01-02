use crate::{middleware::ApiKeyAuth, utils::app_error::AppError};

/// Require admin role for the request
pub fn require_admin(auth: &ApiKeyAuth) -> Result<(), AppError> {
    if auth.role != "admin" {
        return Err(AppError::Forbidden);
    }
    Ok(())
}

/// Require account access (admin can access any, customer only their own)
pub fn require_account_access(auth: &ApiKeyAuth, account_id: i64) -> Result<(), AppError> {
    if auth.role == "admin" {
        return Ok(()); // Admin can access any account
    }

    // Customer must have an account_id and it must match
    match auth.account_id {
        Some(auth_account_id) if auth_account_id == account_id => Ok(()),
        _ => Err(AppError::Forbidden),
    }
}
