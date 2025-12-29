-- This file should undo anything in `up.sql`
-- migrations/2025-12-29-000003_create_transactions/down.sql
DROP TABLE transactions;
DROP TYPE transaction_status;
DROP TYPE transaction_type;