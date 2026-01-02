-- Make account_id nullable to allow admin keys without accounts
ALTER TABLE api_keys 
ALTER COLUMN account_id DROP NOT NULL;

-- Update the foreign key constraint to allow NULL
-- (PostgreSQL foreign keys already allow NULL by default, so no change needed)
