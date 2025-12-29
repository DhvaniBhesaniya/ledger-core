-- This file should undo anything in `up.sql`
-- migrations/2025-12-29-000005_create_webhook_events/down.sql
DROP TABLE webhook_events;
DROP TYPE webhook_status;