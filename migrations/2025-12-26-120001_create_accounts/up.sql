-- Your SQL goes here
-- migrations/YYYY-MM-DD-HH0000_create_accounts/up.sql
CREATE TABLE accounts (
    id BIGSERIAL PRIMARY KEY,
    business_name VARCHAR(255) NOT NULL,
    balance BIGINT NOT NULL DEFAULT 0,
    currency VARCHAR(3) NOT NULL DEFAULT 'USD',
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_accounts_business_name ON accounts(business_name);
CREATE INDEX idx_accounts_is_active ON accounts(is_active);