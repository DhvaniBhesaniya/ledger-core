use crate::models::{TransactionStatus, TransactionType};
use crate::schema::transactions;
use chrono::NaiveDateTime;
use diesel::{
    Selectable,
    prelude::{Insertable, Queryable},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Queryable, Selectable, Serialize)]
#[diesel(table_name = transactions)]
pub struct Transaction {
    pub id: i64,
    pub from_account_id: Option<i64>,
    pub to_account_id: Option<i64>,
    pub amount: i64,
    pub tx_type: TransactionType,
    pub status: TransactionStatus,
    pub description: Option<String>,
    pub idempotency_key: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = transactions)]
pub struct NewTransaction {
    pub from_account_id: Option<i64>,
    pub to_account_id: Option<i64>,
    pub amount: i64,
    pub tx_type: TransactionType,
    pub status: TransactionStatus,
    pub description: Option<String>,
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTransactionRequest {
    pub from_account_id: Option<i64>,
    pub to_account_id: Option<i64>,
    pub amount: i64,
    pub tx_type: TransactionType,
    pub description: Option<String>,
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TransactionResponse {
    pub id: i64,
    pub from_account_id: Option<i64>,
    pub to_account_id: Option<i64>,
    pub amount: i64,
    pub tx_type: TransactionType,
    pub status: TransactionStatus,
    pub created_at: NaiveDateTime,
}

impl From<Transaction> for TransactionResponse {
    fn from(tx: Transaction) -> Self {
        TransactionResponse {
            id: tx.id,
            from_account_id: tx.from_account_id,
            to_account_id: tx.to_account_id,
            amount: tx.amount,
            tx_type: tx.tx_type,
            status: tx.status,
            created_at: tx.created_at,
        }
    }
}
