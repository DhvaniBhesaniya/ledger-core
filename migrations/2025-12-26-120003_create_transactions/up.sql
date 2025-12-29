-- Your SQL goes here
-- migrations/YYYY-MM-DD-HH0000_create_transactions/up.sql
CREATE TYPE transaction_type AS ENUM ('credit', 'debit', 'transfer');
CREATE TYPE transaction_status AS ENUM ('pending', 'completed', 'failed', 'cancelled');

CREATE TABLE transactions (
    id BIGSERIAL PRIMARY KEY,
    from_account_id BIGINT REFERENCES accounts(id),
    to_account_id BIGINT REFERENCES accounts(id),
    amount BIGINT NOT NULL,  -- in cents
    tx_type transaction_type NOT NULL,
    status transaction_status NOT NULL DEFAULT 'pending',
    description VARCHAR(500),
    idempotency_key VARCHAR(255),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_transactions_from_account ON transactions(from_account_id);
CREATE INDEX idx_transactions_to_account ON transactions(to_account_id);
CREATE INDEX idx_transactions_idempotency_key ON transactions(idempotency_key);
CREATE UNIQUE INDEX idx_transactions_unique_idempotency ON transactions(idempotency_key)
    WHERE idempotency_key IS NOT NULL;