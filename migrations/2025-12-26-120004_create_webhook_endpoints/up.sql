-- Your SQL goes here
-- migrations/2025-12-29-000004_create_webhook_endpoints/up.sql
CREATE TABLE webhook_endpoints (
    id BIGSERIAL PRIMARY KEY,
    account_id BIGINT NOT NULL REFERENCES accounts(id),
    url VARCHAR(2048) NOT NULL,
    secret VARCHAR(255) NOT NULL, -- HMAC secret
    events JSONB NOT NULL DEFAULT '[]', -- ["transaction.created", "transaction.completed"]
    is_active BOOLEAN NOT NULL DEFAULT true,
    retry_max_attempts INT NOT NULL DEFAULT 3,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_webhook_endpoints_account_id ON webhook_endpoints(account_id);