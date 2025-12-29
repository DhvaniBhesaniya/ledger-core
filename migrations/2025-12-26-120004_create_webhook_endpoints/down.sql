-- This file should undo anything in `up.sql`
events JSONB NOT NULL,  -- ["transaction.created", "transaction.completed"]