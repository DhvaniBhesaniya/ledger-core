-- Your SQL goes here
-- migrations/YYYY-MM-DD-HH0000_create_webhook_endpoints/up.sql
CREATE TABLE webhook_endpoints (
    id BIGSERIAL PRIMARY KEY,
    account_id BIGINT NOT NULL REFERENCES accounts(id),
    url VARCHAR(2048) NOT NULL,
    secret VARCHAR(255) NOT NULL,  -- HMAC secret
    events JSONB NOT NULL,  -- ["transaction.created", "transaction.completed"]
    is_active BOOLEAN DEFAULT TRUE,
    retry_max_attempts INT DEFAULT 10,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_webhook_endpoints_account_id ON webhook_endpoints(account_id);