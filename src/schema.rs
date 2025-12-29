// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "transaction_status"))]
    pub struct TransactionStatus;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "transaction_type"))]
    pub struct TransactionType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "webhook_status"))]
    pub struct WebhookStatus;
}

diesel::table! {
    accounts (id) {
        id -> Int8,
        #[max_length = 255]
        business_name -> Varchar,
        balance -> Int8,
        #[max_length = 3]
        currency -> Varchar,
        is_active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    api_keys (id) {
        id -> Int8,
        account_id -> Int8,
        #[max_length = 255]
        key_hash -> Varchar,
        #[max_length = 20]
        key_prefix -> Varchar,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        is_active -> Nullable<Bool>,
        rate_limit_per_minute -> Int4,
        last_used_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    idempotency_cache (id) {
        id -> Int8,
        #[max_length = 255]
        idempotency_key -> Varchar,
        response_status -> Int4,
        response_body -> Text,
        created_at -> Timestamp,
        expires_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TransactionType;
    use super::sql_types::TransactionStatus;

    transactions (id) {
        id -> Int8,
        from_account_id -> Nullable<Int8>,
        to_account_id -> Nullable<Int8>,
        amount -> Int8,
        tx_type -> TransactionType,
        status -> TransactionStatus,
        #[max_length = 500]
        description -> Nullable<Varchar>,
        #[max_length = 255]
        idempotency_key -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    webhook_endpoints (id) {
        id -> Int8,
        account_id -> Int8,
        #[max_length = 2048]
        url -> Varchar,
        #[max_length = 255]
        secret -> Varchar,
        events -> Jsonb,
        is_active -> Bool,
        retry_max_attempts -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::WebhookStatus;

    webhook_events (id) {
        id -> Int8,
        webhook_endpoint_id -> Int8,
        #[max_length = 100]
        event_type -> Varchar,
        payload -> Jsonb,
        status -> WebhookStatus,
        attempt_count -> Int4,
        next_retry_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(api_keys -> accounts (account_id));
diesel::joinable!(webhook_endpoints -> accounts (account_id));
diesel::joinable!(webhook_events -> webhook_endpoints (webhook_endpoint_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    api_keys,
    idempotency_cache,
    transactions,
    webhook_endpoints,
    webhook_events,
);
