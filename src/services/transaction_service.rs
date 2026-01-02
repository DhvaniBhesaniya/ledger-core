use crate::{models::*, repositories, utils::app_error::AppError};
use diesel::PgConnection;

pub fn create_transaction(
    _account_id: i64,
    req: CreateTransactionRequest,
    conn: &mut PgConnection,
) -> Result<TransactionResponse, AppError> {
    // Validate request
    if req.amount <= 0 {
        return Err(AppError::BadRequest("Amount must be positive".to_string()));
    }

    let new_tx = NewTransaction {
        from_account_id: req.from_account_id,
        to_account_id: req.to_account_id,
        amount: req.amount,
        tx_type: req.tx_type,
        status: TransactionStatus::Completed,
        description: req.description,
        idempotency_key: req.idempotency_key.clone(),
    };

    // Handle different transaction types
    match req.tx_type {
        TransactionType::Transfer => {
            let from_id = req
                .from_account_id
                .ok_or(AppError::BadRequest("from_account_id required".to_string()))?;
            let to_id = req
                .to_account_id
                .ok_or(AppError::BadRequest("to_account_id required".to_string()))?;

            if from_id == to_id {
                return Err(AppError::BadRequest(
                    "Cannot transfer to same account".to_string(),
                ));
            }

            // Atomic transfer
            repositories::debit_account(from_id, req.amount, conn)?;
            repositories::credit_account(to_id, req.amount, conn)?;
        }
        TransactionType::Credit => {
            let to_id = req
                .to_account_id
                .ok_or(AppError::BadRequest("to_account_id required".to_string()))?;
            repositories::credit_account(to_id, req.amount, conn)?;
        }
        TransactionType::Debit => {
            let from_id = req
                .from_account_id
                .ok_or(AppError::BadRequest("from_account_id required".to_string()))?;
            repositories::debit_account(from_id, req.amount, conn)?;
        }
    }

    let tx = repositories::create_transaction(&new_tx, conn)?;
    Ok(tx.into())
}

pub fn get_transaction(id: i64, conn: &mut PgConnection) -> Result<TransactionResponse, AppError> {
    let tx = repositories::get_transaction_by_id(id, conn)?;
    Ok(tx.into())
}

// List all transactions for an account (both sent and received)
pub fn list_account_transactions(
    account_id: i64,
    conn: &mut PgConnection,
) -> Result<Vec<TransactionResponse>, AppError> {
    let transactions = repositories::get_account_transactions(account_id, conn)?;
    Ok(transactions.into_iter().map(Into::into).collect())
}
