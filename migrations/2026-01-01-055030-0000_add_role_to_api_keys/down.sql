-- Remove role column
DROP INDEX IF EXISTS idx_api_keys_role;
ALTER TABLE api_keys DROP COLUMN role;
