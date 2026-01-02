# Database Reset Guide

Complete guide for resetting the Ledger Core database and starting fresh.

## 🔄 Quick Reset (Recommended)

This is the fastest and cleanest way to reset everything:

```bash
# Stop all services and remove volumes
docker-compose down -v

# Start PostgreSQL
docker-compose up -d postgres

# Wait for database to be ready
sleep 10

# Run all migrations
diesel migration run

# Verify tables were created
docker-compose exec postgres psql -U postgres -d transaction_service -c "\dt"
```

**Expected Output:**
```
                   List of relations
 Schema |            Name            | Type  |  Owner   
--------+----------------------------+-------+----------
 public | __diesel_schema_migrations | table | postgres
 public | accounts                   | table | postgres
 public | api_keys                   | table | postgres
 public | idempotency_cache          | table | postgres
 public | transactions               | table | postgres
 public | webhook_endpoints          | table | postgres
 public | webhook_events             | table | postgres
```

---

## 📋 Reset Options

### Option 1: Docker Compose with Volume Removal (Cleanest)

**Use when:** You want a complete fresh start

```bash
# Stop and remove everything including data
docker-compose down -v

# Start PostgreSQL
docker-compose up -d postgres

# Wait for PostgreSQL to initialize
sleep 10

# Run migrations
diesel migration run

# Verify
docker-compose exec postgres psql -U postgres -d transaction_service -c "SELECT COUNT(*) FROM accounts;"
```

---

### Option 2: Drop All Tables Manually

**Use when:** You want to keep Docker containers running

```bash
# Connect to database
docker-compose exec postgres psql -U postgres -d transaction_service

# Drop all tables (paste this in psql)
DROP TABLE IF EXISTS webhook_events CASCADE;
DROP TABLE IF EXISTS webhook_endpoints CASCADE;
DROP TABLE IF EXISTS transactions CASCADE;
DROP TABLE IF EXISTS idempotency_cache CASCADE;
DROP TABLE IF EXISTS api_keys CASCADE;
DROP TABLE IF EXISTS accounts CASCADE;
DROP TABLE IF EXISTS __diesel_schema_migrations CASCADE;

# Exit psql
\q

# Rerun migrations
diesel migration run
```

---

### Option 3: Drop and Recreate Database

**Use when:** You want to completely recreate the database

```bash
# Connect to PostgreSQL server
docker-compose exec postgres psql -U postgres

# Drop and recreate database
DROP DATABASE IF EXISTS transaction_service;
CREATE DATABASE transaction_service;

# Exit
\q

# Run migrations
diesel migration run
```

---

### Option 4: Nuclear Reset (Complete Cleanup)

**Use when:** Everything is broken and you want to start completely fresh

```bash
# Stop and remove all Docker resources
docker-compose down -v --remove-orphans

# Clean up Docker system
docker system prune -f

# Remove Rust build artifacts (optional)
rm -rf target/

# Start fresh
docker-compose up -d postgres

# Wait for database
sleep 10

# Run migrations
diesel migration run

# Verify everything
docker-compose ps
curl http://localhost:8080/health
```

---

## 🔑 Creating Admin Key After Reset

After resetting the database, you need to create a new admin key:

### Step 1: Generate UUID and Key

```bash
# Generate UUID
uuidgen
# Example output: a1b2c3d4-e5f6-7890-abcd-ef1234567890

# Your full key will be:
# sk_prod_a1b2c3d4-e5f6-7890-abcd-ef1234567890
```

### Step 2: Hash the Key

```bash
# Hash your key (replace with your actual key)
echo -n "sk_prod_a1b2c3d4-e5f6-7890-abcd-ef1234567890" | sha256sum

# Example output:
# 5f8a9b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a  -
```

### Step 3: Insert into Database

```bash
# Connect to database
docker-compose exec postgres psql -U postgres -d transaction_service
```

```sql
-- Insert admin key (replace YOUR_HASH_HERE with actual hash)
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
    NULL,                                                                      -- Admin keys don't need account
    'YOUR_HASH_HERE',                                                         -- SHA256 hash from step 2
    'sk_prod_a1b2c3d4-e',                                                     -- First 20 chars of key
    'Bootstrap Admin Key',
    true,
    1000,
    'admin',
    NOW(),
    NOW()
);

-- Verify it was created
SELECT id, name, role, is_active FROM api_keys;

-- Exit
\q
```

### Step 4: Save Your Key

**⚠️ CRITICAL:** Save your raw API key somewhere secure!

```
Raw Key: sk_prod_a1b2c3d4-e5f6-7890-abcd-ef1234567890
Hash: 5f8a9b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a
Prefix: sk_prod_a1b2c3d4-e
```

You cannot retrieve the raw key later - it's only shown once!

---

## ✅ Verification Steps

After reset, verify everything is working:

### 1. Check Database Tables

```bash
docker-compose exec postgres psql -U postgres -d transaction_service -c "\dt"
```

**Expected:** 7 tables listed

### 2. Check Table Counts

```bash
docker-compose exec postgres psql -U postgres -d transaction_service
```

```sql
SELECT 'accounts' as table_name, COUNT(*) FROM accounts
UNION ALL
SELECT 'api_keys', COUNT(*) FROM api_keys
UNION ALL
SELECT 'transactions', COUNT(*) FROM transactions
UNION ALL
SELECT 'webhook_endpoints', COUNT(*) FROM webhook_endpoints
UNION ALL
SELECT 'webhook_events', COUNT(*) FROM webhook_events;
```

**Expected:** All counts should be 0 (except api_keys if you added admin key)

### 3. Test Health Endpoint

```bash
curl http://localhost:8080/health
```

**Expected:** `OK`

### 4. Test Admin Key

```bash
# List all keys (admin only)
curl http://localhost:8080/api/keys_list \
  -H "x-api-key: YOUR_ADMIN_KEY"
```

**Expected:** JSON array with your admin key

### 5. Create Test Account

```bash
curl -X POST http://localhost:8080/api/accounts \
  -H "Content-Type: application/json" \
  -d '{
    "business_name": "Test Business",
    "currency": "USD"
  }'
```

**Expected:** JSON response with account details and customer API key

---

## 🚀 Automated Reset Script

Create a file `reset_database.sh`:

```bash
#!/bin/bash

echo "🔄 Starting database reset..."

# Stop services
echo "⏹️  Stopping services..."
docker-compose down -v

# Start PostgreSQL
echo "🚀 Starting PostgreSQL..."
docker-compose up -d postgres

# Wait for database
echo "⏳ Waiting for PostgreSQL to be ready..."
sleep 10

# Run migrations
echo "📊 Running migrations..."
diesel migration run

# Verify
echo "✅ Verifying tables..."
docker-compose exec postgres psql -U postgres -d transaction_service -c "\dt"

echo ""
echo "✅ Database reset complete!"
echo ""
echo "📝 Next steps:"
echo "1. Create your admin API key (see instructions above)"
echo "2. Start the application: cargo run"
echo "3. Test: curl http://localhost:8080/health"
```

Make it executable and run:

```bash
chmod +x reset_database.sh
./reset_database.sh
```

---

## 🐛 Troubleshooting

### "Database does not exist"

```bash
# Recreate database
docker-compose exec postgres psql -U postgres -c "CREATE DATABASE transaction_service;"
diesel migration run
```

### "Connection refused"

```bash
# Check if PostgreSQL is running
docker-compose ps

# Restart if needed
docker-compose restart postgres
sleep 10
diesel migration run
```

### "Migration already applied"

```bash
# Revert all migrations
diesel migration revert --all

# Rerun
diesel migration run
```

### "Port already in use"

```bash
# Find process using port
lsof -i :5432

# Kill it
kill -9 <PID>

# Or change port in docker-compose.yml
```

### Tables still have data after reset

```bash
# Make sure you used -v flag
docker-compose down -v

# Check volumes
docker volume ls

# Remove specific volume if needed
docker volume rm ledger-core_postgres_data
```

---

## 📊 Database Schema After Reset

After running migrations, you'll have these tables:

```
accounts
├─ id (BIGSERIAL PRIMARY KEY)
├─ business_name (VARCHAR NOT NULL)
├─ balance (BIGINT DEFAULT 0)
├─ currency (VARCHAR DEFAULT 'USD')
├─ is_active (BOOLEAN DEFAULT true)
├─ created_at (TIMESTAMP)
└─ updated_at (TIMESTAMP)

api_keys
├─ id (BIGSERIAL PRIMARY KEY)
├─ account_id (BIGINT NULLABLE) ← NULL for admin keys
├─ key_hash (VARCHAR UNIQUE)
├─ key_prefix (VARCHAR)
├─ name (VARCHAR)
├─ is_active (BOOLEAN)
├─ rate_limit_per_minute (INT)
├─ last_used_at (TIMESTAMP)
├─ role (VARCHAR) ← 'admin' or 'customer'
├─ created_at (TIMESTAMP)
└─ updated_at (TIMESTAMP)

transactions
├─ id (BIGSERIAL PRIMARY KEY)
├─ from_account_id (BIGINT)
├─ to_account_id (BIGINT)
├─ amount (BIGINT)
├─ transaction_type (VARCHAR)
├─ status (VARCHAR)
├─ description (TEXT)
├─ idempotency_key (VARCHAR UNIQUE)
├─ created_at (TIMESTAMP)
└─ updated_at (TIMESTAMP)

webhook_endpoints
├─ id (BIGSERIAL PRIMARY KEY)
├─ account_id (BIGINT)
├─ url (VARCHAR)
├─ secret (VARCHAR)
├─ events (TEXT[])
├─ is_active (BOOLEAN)
├─ created_at (TIMESTAMP)
└─ updated_at (TIMESTAMP)

webhook_events
├─ id (BIGSERIAL PRIMARY KEY)
├─ webhook_endpoint_id (BIGINT)
├─ event_type (VARCHAR)
├─ payload (JSONB)
├─ status (VARCHAR)
├─ attempt_count (INT)
├─ next_retry_at (TIMESTAMP)
├─ created_at (TIMESTAMP)
└─ updated_at (TIMESTAMP)

idempotency_cache
├─ id (BIGSERIAL PRIMARY KEY)
├─ idempotency_key (VARCHAR UNIQUE)
├─ response_data (JSONB)
├─ created_at (TIMESTAMP)
└─ expires_at (TIMESTAMP)
```

---

## 🎯 Common Reset Scenarios

### Scenario 1: Testing Fresh Install

```bash
docker-compose down -v
docker-compose up -d postgres
sleep 10
diesel migration run
# Create admin key
# Start testing
```

### Scenario 2: Corrupted Data

```bash
docker-compose down -v
docker-compose up -d postgres
sleep 10
diesel migration run
# Restore from backup if available
```

### Scenario 3: Migration Issues

```bash
# Revert all migrations
diesel migration revert --all

# Fix migration files
# Rerun
diesel migration run
```

### Scenario 4: Development Reset (Keep Some Data)

```bash
# Just truncate tables instead of dropping
docker-compose exec postgres psql -U postgres -d transaction_service

TRUNCATE webhook_events, webhook_endpoints, transactions, api_keys, accounts RESTART IDENTITY CASCADE;
```

---

## 📝 Best Practices

1. **Always backup before reset** (if in production)
2. **Use `-v` flag** to ensure volumes are removed
3. **Wait for PostgreSQL** to be ready before running migrations
4. **Save admin keys** immediately after creation
5. **Verify tables** after reset
6. **Test health endpoint** before using API
7. **Document your keys** in a secure location

---

## 🔐 Security Notes

- Never commit admin keys to git
- Store keys in environment variables or secrets manager
- Rotate admin keys regularly
- Use different keys for development and production
- Monitor admin key usage via `last_used_at` field

---

**Last Updated:** 2026-01-01
**Status:** ✅ Tested and Working
