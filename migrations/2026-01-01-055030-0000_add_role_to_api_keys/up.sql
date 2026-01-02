-- Add role column with default 'customer'
ALTER TABLE api_keys 
ADD COLUMN role VARCHAR(20) NOT NULL DEFAULT 'customer';

-- Create index for role-based queries
CREATE INDEX idx_api_keys_role ON api_keys(role);
