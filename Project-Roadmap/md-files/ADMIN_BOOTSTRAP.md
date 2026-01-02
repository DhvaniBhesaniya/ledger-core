# Admin Key Bootstrap Guide

## Overview
Since only admin keys can generate new API keys, you need to manually create the first admin key in the database.

## Steps to Create First Admin Key

### 1. Generate a Raw API Key
```bash
# Generate a UUID for your admin key
uuidgen
# Example output: 550e8400-e29b-41d4-a716-446655440000
```

### 2. Create the Full API Key
Combine the prefix with the UUID:
```
sk_prod_550e8400-e29b-41d4-a716-446655440000
```

### 3. Hash the API Key
You need to hash this key using SHA256. You can use the following methods:

**Option A: Using the application's crypto function**
```bash
# In your Rust code or a small utility:
use sha2::{Digest, Sha256};

let raw_key = "sk_prod_550e8400-e29b-41d4-a716-446655440000";
let mut hasher = Sha256::new();
hasher.update(raw_key);
let key_hash = format!("{:x}", hasher.finalize());
println!("Hash: {}", key_hash);
```

**Option B: Using command line**
```bash
echo -n "sk_prod_550e8400-e29b-41d4-a716-446655440000" | sha256sum
```

### 4. Insert into Database
```sql
-- Connect to your database
psql -U your_user -d transaction_service

-- Insert the admin key (account_id is NULL for admin keys)
INSERT INTO api_keys (
    account_id,
    key_hash,
    key_prefix,
    name,
    is_active,
    rate_limit_per_minute,
    role,
    created_at,
    updated_at
) VALUES (
    NULL,  -- Admin keys don't need an account
    'YOUR_HASHED_KEY_HERE',  -- The SHA256 hash from step 3
    'sk_prod_550e8400-e',     -- First 20 chars of raw key
    'Bootstrap Admin Key',
    true,
    1000,  -- Higher rate limit for admin
    'admin',
    NOW(),
    NOW()
);
```

### 5. Save Your Raw API Key
**IMPORTANT**: Save the raw API key (`sk_prod_550e8400-e29b-41d4-a716-446655440000`) securely. You will never be able to retrieve it again!

### 6. Test the Admin Key
```bash
# List all API keys (admin only)
curl -X GET http://localhost:8080/api/keys_list \
  -H "x-api-key: sk_prod_550e8400-e29b-41d4-a716-446655440000"

# Generate a new customer key
curl -X POST http://localhost:8080/api/key_generate \
  -H "x-api-key: sk_prod_550e8400-e29b-41d4-a716-446655440000" \
  -H "Content-Type: application/json" \
  -d '{
    "account_id": 2,
    "name": "Customer Key",
    "role": "customer"
  }'
```

## Security Recommendations

1. **Store the admin key securely** - Use a password manager or secrets vault
2. **Limit admin key distribution** - Only give to trusted administrators
3. **Rotate regularly** - Generate new admin keys periodically and revoke old ones
4. **Monitor usage** - Check `last_used_at` field regularly
5. **Use environment variables** - Never hardcode admin keys in your application

## Revoking an Admin Key

If an admin key is compromised:

```sql
-- Disable the key
UPDATE api_keys 
SET is_active = false 
WHERE key_prefix = 'sk_prod_550e8400-e';

-- Or delete it entirely
DELETE FROM api_keys 
WHERE key_prefix = 'sk_prod_550e8400-e';
```

Then create a new admin key following the steps above.
