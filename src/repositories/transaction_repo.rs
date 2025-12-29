#![allow(dead_code)]
use crate::error::AppError;
use crate::models::{NewTransaction, Transaction, TransactionStatus};
use crate::schema::transactions;
use diesel::prelude::*;

pub fn create_transaction(
    new_tx: &NewTransaction,
    conn: &mut PgConnection,
) -> Result<Transaction, AppError> {
    // Check for duplicate idempotency key
    if let Some(ref key) = new_tx.idempotency_key {
        if transactions::table
            .filter(transactions::idempotency_key.eq(key))
            .first::<Transaction>(conn)
            .ok()
            .is_some()
        {
            return Err(AppError::DuplicateIdempotencyKey);
        }
    }

    diesel::insert_into(transactions::table)
        .values(new_tx)
        .get_result(conn)
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}

pub fn get_transaction_by_id(id: i64, conn: &mut PgConnection) -> Result<Transaction, AppError> {
    transactions::table
        .find(id)
        .first(conn)
        .map_err(|_| AppError::TransactionNotFound)
}

pub fn get_pending_transactions(conn: &mut PgConnection) -> Result<Vec<Transaction>, AppError> {
    transactions::table
        .filter(transactions::status.eq(TransactionStatus::Pending))
        .load(conn)
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}
