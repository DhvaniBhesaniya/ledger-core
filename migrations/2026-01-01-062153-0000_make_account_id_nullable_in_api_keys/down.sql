-- Revert account_id to NOT NULL
ALTER TABLE api_keys 
ALTER COLUMN account_id SET NOT NULL;
