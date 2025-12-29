-- Your SQL goes here
-- migrations/YYYY-MM-DD-HH0000_create_api_keys/up.sql
CREATE TABLE api_keys (
    id BIGSERIAL PRIMARY KEY,
    account_id BIGINT NOT NULL REFERENCES accounts(id),
    key_hash VARCHAR(255) NOT NULL UNIQUE,  -- Never store plain key!
    key_prefix VARCHAR(20) NOT NULL,         -- Show user "sk_prod_xxxx"
    name VARCHAR(255),
    is_active BOOLEAN DEFAULT true,
    rate_limit_per_minute INT NOT NULL DEFAULT 60,
    last_used_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_api_keys_account_id ON api_keys(account_id);
CREATE INDEX idx_api_keys_key_hash ON api_keys(key_hash);