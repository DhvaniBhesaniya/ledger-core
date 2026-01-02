# Transaction Integration Tests

Comprehensive integration tests for the Ledger Core transaction system, including rate limiting tests.

## 📋 Test Coverage

### Transaction Tests (`tests/transaction_tests.rs`)

1. **Basic Transaction Operations**
   - ✅ Create credit transaction
   - ✅ Create transfer transaction
   - ✅ Create debit transaction

2. **Authorization Tests**
   - ✅ Customer can only see their own transactions
   - ✅ Customer cannot see other account's transactions (403 Forbidden)
   - ✅ Admin can see all transactions

3. **Transaction Listing**
   - ✅ List all transactions for an account
   - ✅ Verify both sent and received transactions appear
   - ✅ Transactions ordered by most recent first

4. **Idempotency Tests**
   - ✅ Duplicate idempotency key returns error (409 Conflict)
   - ✅ Same transaction cannot be created twice

5. **Rate Limiting Tests** ⭐
   - ✅ Rate limit enforced at 60 requests/minute
   - ✅ 429 Too Many Requests returned after limit
   - ✅ Rate limit resets after 60 seconds
   - ✅ Detailed logging of rate limit behavior

6. **Validation Tests**
   - ✅ Negative amount rejected (400 Bad Request)
   - ✅ Zero amount rejected (400 Bad Request)
   - ✅ Transfer to same account rejected (400 Bad Request)

## 🚀 Running the Tests

### Prerequisites

1. **Start the server:**
   ```bash
   cargo run
   ```

2. **In a new terminal, run tests:**

### Run All Tests
```bash
cargo test --test transaction_tests
```

### Run Specific Test
```bash
# Test rate limiting
cargo test --test transaction_tests test_rate_limiting -- --nocapture

# Test authorization
cargo test --test transaction_tests test_get_transaction_authorization

# Test transaction listing
cargo test --test transaction_tests test_list_account_transactions
```

### Run with Output
```bash
# See detailed output (recommended for rate limit tests)
cargo test --test transaction_tests -- --nocapture

# Run specific test with output
cargo test --test transaction_tests test_rate_limiting -- --nocapture --test-threads=1
```

## 📊 Rate Limit Test Details

The rate limit test (`test_rate_limiting`) performs the following:

1. Creates a test account with default rate limit (60 req/min)
2. Rapidly sends 70 transaction requests
3. Tracks successful vs rate-limited responses
4. Verifies:
   - Maximum 60 successful requests
   - Remaining 10+ requests return 429 status
   - Rate limit is properly enforced

**Expected Output:**
```
🧪 Starting rate limit test...
📊 Default rate limit: 60 requests/minute
✅ Request 10: Success (total: 10)
✅ Request 20: Success (total: 20)
✅ Request 30: Success (total: 30)
✅ Request 40: Success (total: 40)
✅ Request 50: Success (total: 50)
✅ Request 60: Success (total: 60)
⚠️  Request 61: Rate limited! (first occurrence)

📈 Rate Limit Test Results:
   ✅ Successful requests: 60
   ⚠️  Rate limited requests: 10
   📊 Total requests: 70
✅ Rate limit test passed!
```

## 🧪 Rate Limit Recovery Test

The recovery test (`test_rate_limit_recovery`) verifies that rate limits reset:

1. Exhausts the rate limit (65 requests)
2. Waits 60 seconds
3. Verifies new requests succeed

**Note:** This test takes ~60 seconds to complete.

```bash
cargo test --test transaction_tests test_rate_limit_recovery -- --nocapture
```

## 🔧 Test Configuration

### Modify Rate Limits for Testing

To test with different rate limits, update the API key in the database:

```sql
-- Set lower rate limit for faster testing
UPDATE api_keys SET rate_limit_per_minute = 10 WHERE id = 1;

-- Set higher rate limit
UPDATE api_keys SET rate_limit_per_minute = 100 WHERE id = 1;
```

### Test Environment Variables

```bash
# Set base URL if running on different port
export TEST_BASE_URL=http://localhost:8080

# Enable debug logging
RUST_LOG=debug cargo test --test transaction_tests
```

## 📝 Writing New Tests

### Template for New Test

```rust
#[tokio::test]
async fn test_your_feature() {
    let client = reqwest::Client::new();
    let (account_id, api_key) = create_test_account(&client).await;

    // Your test logic here
    let response = client
        .post(&format!("{}/api/transactions", BASE_URL))
        .header("x-api-key", &api_key)
        .json(&json!({
            // Your request body
        }))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 200);
}
```

## 🐛 Troubleshooting

### "Connection refused"
```bash
# Make sure server is running
cargo run

# Check server is listening
curl http://localhost:8080/health
```

### "Test failed: rate limit not enforced"
```bash
# Check rate limit middleware is enabled
# Verify in src/routes/mod.rs that rate_limit middleware is applied

# Check API key rate limit in database
docker-compose exec postgres psql -U postgres -d transaction_service \
  -c "SELECT id, rate_limit_per_minute FROM api_keys;"
```

### Tests are slow
```bash
# Run tests in parallel (default)
cargo test --test transaction_tests

# Run tests sequentially (for debugging)
cargo test --test transaction_tests -- --test-threads=1
```

## 📈 CI/CD Integration

### GitHub Actions Example

```yaml
name: Integration Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    
    services:
      postgres:
        image: postgres:15
        env:
          POSTGRES_PASSWORD: password
          POSTGRES_DB: transaction_service
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Run migrations
        run: diesel migration run
      
      - name: Start server
        run: cargo run &
        
      - name: Wait for server
        run: sleep 10
      
      - name: Run tests
        run: cargo test --test transaction_tests
```

## 🎯 Test Metrics

| Test | Duration | Requests | Assertions |
|------|----------|----------|------------|
| `test_create_credit_transaction` | ~100ms | 2 | 4 |
| `test_create_transfer_transaction` | ~200ms | 4 | 5 |
| `test_get_transaction_authorization` | ~150ms | 3 | 2 |
| `test_list_account_transactions` | ~300ms | 4 | 4 |
| `test_idempotency` | ~100ms | 3 | 2 |
| `test_rate_limiting` | ~5s | 72 | 3 |
| `test_rate_limit_recovery` | ~61s | 67 | 1 |
| `test_invalid_transaction_amount` | ~100ms | 3 | 2 |
| `test_transfer_to_same_account` | ~150ms | 3 | 1 |

**Total:** ~67 seconds (including 60s wait in recovery test)

## ✅ Success Criteria

All tests should pass with:
- ✅ 200 OK for valid requests
- ✅ 403 Forbidden for unauthorized access
- ✅ 400 Bad Request for invalid data
- ✅ 409 Conflict for duplicate idempotency keys
- ✅ 429 Too Many Requests when rate limited
- ✅ Rate limit enforced at configured threshold
- ✅ Rate limit resets after time window

---

**Last Updated:** 2026-01-01
**Test Framework:** Tokio + Reqwest
**Coverage:** Transaction API + Rate Limiting
