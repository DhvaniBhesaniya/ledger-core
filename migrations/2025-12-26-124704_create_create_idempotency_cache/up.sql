-- Your SQL goes here
-- migrations/YYYY-MM-DD-HH0000_create_idempotency_cache/up.sql
CREATE TABLE idempotency_cache (
    id BIGSERIAL PRIMARY KEY,
    idempotency_key VARCHAR(255) NOT NULL UNIQUE,
    response_status INT NOT NULL,
    response_body TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP NOT NULL  -- Auto-cleanup after 24h
);

CREATE INDEX idx_idempotency_cache_expires ON idempotency_cache(expires_at);