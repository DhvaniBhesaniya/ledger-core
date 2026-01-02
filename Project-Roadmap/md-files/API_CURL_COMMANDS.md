# Ledger Core API - cURL Commands

## System

### Health Check (Public)
```bash
curl http://localhost:8080/health
```

---

## Accounts

### Create Account (Public - Returns API Key)
```bash
curl -X POST http://localhost:8080/api/accounts \
  -H "Content-Type: application/json" \
  -d '{
    "business_name": "My Business",
    "currency": "USD"
  }'
```

### Get Account (Protected)
```bash
curl -X GET http://localhost:8080/api/accounts/1 \
  -H "x-api-key: YOUR_API_KEY"
```

### Get Balance (Protected)
```bash
curl -X GET http://localhost:8080/api/accounts/1/balance \
  -H "x-api-key: YOUR_API_KEY"
```

### List Account Keys (Protected)
```bash
curl -X GET http://localhost:8080/api/accounts/1/keys \
  -H "x-api-key: YOUR_API_KEY"
```

---

## Transactions

### Create Transaction - Deposit (Protected)
```bash
curl -X POST http://localhost:8080/api/transactions \
  -H "Content-Type: application/json" \
  -H "x-api-key: YOUR_API_KEY" \
  -d '{
    "amount": 1000,
    "tx_type": "deposit",
    "description": "Initial Funding",
    "idempotency_key": "unique_id_123"
  }'
```

### Create Transaction - Withdrawal (Protected)
```bash
curl -X POST http://localhost:8080/api/transactions \
  -H "Content-Type: application/json" \
  -H "x-api-key: YOUR_API_KEY" \
  -d '{
    "from_account_id": 1,
    "amount": 500,
    "tx_type": "withdrawal",
    "description": "Cash withdrawal",
    "idempotency_key": "unique_id_456"
  }'
```

### Create Transaction - Transfer (Protected)
```bash
curl -X POST http://localhost:8080/api/transactions \
  -H "Content-Type: application/json" \
  -H "x-api-key: YOUR_API_KEY" \
  -d '{
    "from_account_id": 1,
    "to_account_id": 2,
    "amount": 250,
    "tx_type": "transfer",
    "description": "Payment to vendor",
    "idempotency_key": "unique_id_789"
  }'
```

### Get Transaction (Protected)
```bash
curl -X GET http://localhost:8080/api/transactions/1 \
  -H "x-api-key: YOUR_API_KEY"
```

---

## API Keys

### Generate API Key (Public)
```bash
curl -X POST http://localhost:8080/api/keys \
  -H "Content-Type: application/json" \
  -d '{
    "account_id": 1,
    "name": "Backup Key",
    "rate_limit_per_minute": 60
  }'
```

### List All API Keys - Admin (Protected)
```bash
curl -X GET http://localhost:8080/api/keys \
  -H "x-api-key: YOUR_API_KEY"
```

### Update API Key (Protected)
```bash
# Update key name
curl -X PATCH http://localhost:8080/api/keys/1 \
  -H "x-api-key: YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"name": "Production Key"}'

# Update rate limit
curl -X PATCH http://localhost:8080/api/keys/1 \
  -H "x-api-key: YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"rate_limit_per_minute": 100}'

# Disable a key
curl -X PATCH http://localhost:8080/api/keys/2 \
  -H "x-api-key: YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"is_active": false}'

# Update multiple fields
curl -X PATCH http://localhost:8080/api/keys/1 \
  -H "x-api-key: YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Updated Key",
    "rate_limit_per_minute": 120,
    "is_active": true
  }'
```

---

## Webhooks

### Register Webhook (Protected)
```bash
curl -X POST http://localhost:8080/api/webhooks \
  -H "Content-Type: application/json" \
  -H "x-api-key: YOUR_API_KEY" \
  -d '{
    "url": "https://example.com/webhook",
    "events": ["transaction.created", "account.updated"]
  }'
```

### Get Webhook (Protected)
```bash
curl -X GET http://localhost:8080/api/webhooks/1 \
  -H "x-api-key: YOUR_API_KEY"
```

### Delete Webhook (Protected)
```bash
curl -X DELETE http://localhost:8080/api/webhooks/1 \
  -H "x-api-key: YOUR_API_KEY"
```

---

## Quick Start Flow

1. **Create an account** (receives API key in response):
```bash
curl -X POST http://localhost:8080/api/accounts \
  -H "Content-Type: application/json" \
  -d '{"business_name": "Test Corp", "currency": "USD"}'
```

2. **Save the `secret_api_key` from the response** and use it in subsequent requests.

3. **Create a deposit**:
```bash
curl -X POST http://localhost:8080/api/transactions \
  -H "Content-Type: application/json" \
  -H "x-api-key: sk_prod_..." \
  -d '{
    "amount": 5000,
    "tx_type": "deposit",
    "description": "Initial deposit"
  }'
```

4. **Check balance**:
```bash
curl -X GET http://localhost:8080/api/accounts/1/balance \
  -H "x-api-key: sk_prod_..."
```

---

## Notes

- **Public endpoints**: `/health`, `POST /api/accounts`, `POST /api/keys`
- **Protected endpoints**: All others (require `x-api-key` header)
- **Auto-generated key**: Creating an account automatically generates a "Root Key"
- **One-time secret**: The `secret_api_key` is only shown once in the account creation response
- Replace `YOUR_API_KEY` with your actual API key (starts with `sk_prod_`)
