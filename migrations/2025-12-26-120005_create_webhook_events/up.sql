-- Your SQL goes here
-- migrations/2025-12-29-000005_create_webhook_events/up.sql
CREATE TYPE webhook_status AS ENUM ('pending', 'delivered', 'failed');

CREATE TABLE webhook_events (
    id BIGSERIAL PRIMARY KEY,
    webhook_endpoint_id BIGINT NOT NULL REFERENCES webhook_endpoints(id),
    event_type VARCHAR(100) NOT NULL,
    payload JSONB NOT NULL,
    status webhook_status NOT NULL DEFAULT 'pending',
    attempt_count INT NOT NULL DEFAULT 0,
    next_retry_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_webhook_events_endpoint_id ON webhook_events(webhook_endpoint_id);
CREATE INDEX idx_webhook_events_status ON webhook_events(status);
CREATE INDEX idx_webhook_events_next_retry ON webhook_events(next_retry_at);
