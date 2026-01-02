// cargo test --test transaction_tests --no-run
// cargo test test_rate_limiting
// cargo test test_rate_limiting   -- --nocapture   (this allows to see print statements)

use reqwest;
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;

const BASE_URL: &str = "http://localhost:8080";

// Test helper to create an account and get API key
async fn create_test_account(client: &reqwest::Client) -> (i64, String) {
    let response = client
        .post(&format!("{}/api/accounts", BASE_URL))
        .json(&json!({
            "business_name": "Test Account",
            "currency": "USD"
        }))
        .send()
        .await
        .expect("Failed to create account");

    assert_eq!(response.status(), 200);

    let body: serde_json::Value = response.json().await.expect("Failed to parse response");
    let account_id = body["account"]["id"].as_i64().expect("No account ID");
    let api_key = body["secret_api_key"]
        .as_str()
        .expect("No API key")
        .to_string();

    (account_id, api_key)
}

#[tokio::test]
async fn test_create_credit_transaction() {
    let client = reqwest::Client::new();
    let (account_id, api_key) = create_test_account(&client).await;

    // Create credit transaction
    let response = client
        .post(&format!("{}/api/transactions", BASE_URL))
        .header("x-api-key", &api_key)
        .json(&json!({
            "transaction_type": "credit",
            "to_account_id": account_id,
            "amount": 10000,
            "currency": "USD",
            "description": "Initial deposit",
            "idempotency_key": format!("test_credit_{}", uuid::Uuid::new_v4())
        }))
        .send()
        .await
        .expect("Failed to create transaction");

    assert_eq!(response.status(), 200);

    let body: serde_json::Value = response.json().await.expect("Failed to parse response");
    assert_eq!(body["transaction_type"], "credit");
    assert_eq!(body["amount"], 10000);
    assert_eq!(body["status"], "completed");
}

#[tokio::test]
async fn test_create_transfer_transaction() {
    let client = reqwest::Client::new();

    // Create two accounts
    let (account1_id, api_key1) = create_test_account(&client).await;
    let (account2_id, _api_key2) = create_test_account(&client).await;

    // Credit account 1 first
    client
        .post(&format!("{}/api/transactions", BASE_URL))
        .header("x-api-key", &api_key1)
        .json(&json!({
            "transaction_type": "credit",
            "to_account_id": account1_id,
            "amount": 20000,
            "currency": "USD",
            "idempotency_key": format!("test_credit_{}", uuid::Uuid::new_v4())
        }))
        .send()
        .await
        .expect("Failed to credit account");

    // Transfer from account 1 to account 2
    let response = client
        .post(&format!("{}/api/transactions", BASE_URL))
        .header("x-api-key", &api_key1)
        .json(&json!({
            "transaction_type": "transfer",
            "from_account_id": account1_id,
            "to_account_id": account2_id,
            "amount": 5000,
            "currency": "USD",
            "description": "Transfer test",
            "idempotency_key": format!("test_transfer_{}", uuid::Uuid::new_v4())
        }))
        .send()
        .await
        .expect("Failed to create transfer");

    assert_eq!(response.status(), 200);

    let body: serde_json::Value = response.json().await.expect("Failed to parse response");
    assert_eq!(body["transaction_type"], "transfer");
    assert_eq!(body["amount"], 5000);
    assert_eq!(body["from_account_id"], account1_id);
    assert_eq!(body["to_account_id"], account2_id);
}

#[tokio::test]
async fn test_get_transaction_authorization() {
    let client = reqwest::Client::new();

    // Create two accounts
    let (account1_id, api_key1) = create_test_account(&client).await;
    let (_account2_id, api_key2) = create_test_account(&client).await;

    // Create transaction for account 1
    let tx_response = client
        .post(&format!("{}/api/transactions", BASE_URL))
        .header("x-api-key", &api_key1)
        .json(&json!({
            "transaction_type": "credit",
            "to_account_id": account1_id,
            "amount": 1000,
            "currency": "USD",
            "idempotency_key": format!("test_auth_{}", uuid::Uuid::new_v4())
        }))
        .send()
        .await
        .expect("Failed to create transaction");

    let tx_body: serde_json::Value = tx_response.json().await.expect("Failed to parse");
    let tx_id = tx_body["id"].as_i64().expect("No transaction ID");

    // Account 1 should be able to see their transaction
    let response = client
        .get(&format!("{}/api/transactions/{}", BASE_URL, tx_id))
        .header("x-api-key", &api_key1)
        .send()
        .await
        .expect("Failed to get transaction");

    assert_eq!(response.status(), 200);

    // Account 2 should NOT be able to see account 1's transaction
    let response = client
        .get(&format!("{}/api/transactions/{}", BASE_URL, tx_id))
        .header("x-api-key", &api_key2)
        .send()
        .await
        .expect("Failed to get transaction");

    assert_eq!(response.status(), 403); // Forbidden
}

#[tokio::test]
async fn test_list_account_transactions() {
    let client = reqwest::Client::new();

    // Create two accounts
    let (account1_id, api_key1) = create_test_account(&client).await;
    let (account2_id, _api_key2) = create_test_account(&client).await;

    // Create credit transaction for account 1
    client
        .post(&format!("{}/api/transactions", BASE_URL))
        .header("x-api-key", &api_key1)
        .json(&json!({
            "transaction_type": "credit",
            "to_account_id": account1_id,
            "amount": 10000,
            "currency": "USD",
            "idempotency_key": format!("test_list_credit_{}", uuid::Uuid::new_v4())
        }))
        .send()
        .await
        .expect("Failed to create credit");

    // Create transfer from account 1 to account 2
    client
        .post(&format!("{}/api/transactions", BASE_URL))
        .header("x-api-key", &api_key1)
        .json(&json!({
            "transaction_type": "transfer",
            "from_account_id": account1_id,
            "to_account_id": account2_id,
            "amount": 3000,
            "currency": "USD",
            "idempotency_key": format!("test_list_transfer_{}", uuid::Uuid::new_v4())
        }))
        .send()
        .await
        .expect("Failed to create transfer");

    // List all transactions for account 1
    let response = client
        .get(&format!(
            "{}/api/transactions/account/{}",
            BASE_URL, account1_id
        ))
        .header("x-api-key", &api_key1)
        .send()
        .await
        .expect("Failed to list transactions");

    assert_eq!(response.status(), 200);

    let transactions: Vec<serde_json::Value> = response.json().await.expect("Failed to parse");

    // Account 1 should see BOTH transactions (credit and transfer)
    assert_eq!(transactions.len(), 2);

    // Verify both transaction types are present
    let has_credit = transactions
        .iter()
        .any(|tx| tx["transaction_type"] == "credit");
    let has_transfer = transactions
        .iter()
        .any(|tx| tx["transaction_type"] == "transfer");

    assert!(has_credit, "Should have credit transaction");
    assert!(has_transfer, "Should have transfer transaction");
}

#[tokio::test]
async fn test_idempotency() {
    let client = reqwest::Client::new();
    let (account_id, api_key) = create_test_account(&client).await;

    let idempotency_key = format!("test_idempotency_{}", uuid::Uuid::new_v4());

    // Create transaction with idempotency key
    let response1 = client
        .post(&format!("{}/api/transactions", BASE_URL))
        .header("x-api-key", &api_key)
        .json(&json!({
            "transaction_type": "credit",
            "to_account_id": account_id,
            "amount": 5000,
            "currency": "USD",
            "idempotency_key": &idempotency_key
        }))
        .send()
        .await
        .expect("Failed to create transaction");

    assert_eq!(response1.status(), 200);

    // Try to create same transaction again with same idempotency key
    let response2 = client
        .post(&format!("{}/api/transactions", BASE_URL))
        .header("x-api-key", &api_key)
        .json(&json!({
            "transaction_type": "credit",
            "to_account_id": account_id,
            "amount": 5000,
            "currency": "USD",
            "idempotency_key": &idempotency_key
        }))
        .send()
        .await
        .expect("Failed to create transaction");

    // Should return error for duplicate idempotency key
    assert_eq!(response2.status(), 409); // Conflict
}

#[tokio::test]
async fn test_rate_limiting() {
    let client = reqwest::Client::new();
    // let (account_id, api_key) = create_test_account(&client).await;
    let account_id = 1;
    let api_key = "sk_prod_69ff2af3-9c94-4a55-ac87-a568f3146a9c".to_string();

    println!("🧪 Starting rate limit test...");
    println!("📊 Default rate limit: 60 requests/minute");

    let mut success_count = 0;
    let mut rate_limited_count = 0;

    // Try to make 70 requests rapidly (should hit rate limit at 60)
    for i in 1..=70 {
        let response = client
            .get(&format!("{}/api/transactions/account/{}", BASE_URL, account_id))
            .header("x-api-key", &api_key)
            // .json(&json!({
            //     "transaction_type": "credit",
            //     "to_account_id": account_id,
            //     "amount": 100,
            //     "currency": "USD",
            //     "idempotency_key": format!("rate_test_{}_{}", uuid::Uuid::new_v4(), i)
            // }))
            .send()
            .await
            .expect("Failed to send request");

        match response.status().as_u16() {
            200 => {
                success_count += 1;
                if i % 10 == 0 {
                    println!("✅ Request {}: Success (total: {})", i, success_count);
                }
            }
            429 => {
                rate_limited_count += 1;
                if rate_limited_count == 1 {
                    println!("⚠️  Request {}: Rate limited! (first occurrence)", i);
                }
            }
            status => {
                println!("❌ Request {}: Unexpected status {}", i, status);
            }
        }

        // Small delay to avoid overwhelming the server
        sleep(Duration::from_millis(10)).await;
    }

    println!("\n📈 Rate Limit Test Results:");
    println!("   ✅ Successful requests: {}", success_count);
    println!("   ⚠️  Rate limited requests: {}", rate_limited_count);
    println!(
        "   📊 Total requests: {}",
        success_count + rate_limited_count
    );

    // Assertions
    assert!(success_count <= 60, "Should not exceed rate limit of 60");
    assert!(
        rate_limited_count > 0,
        "Should have some rate limited requests"
    );
    assert_eq!(
        success_count + rate_limited_count,
        70,
        "Should have made 70 total requests"
    );

    println!("✅ Rate limit test passed!");
}

#[tokio::test]
async fn test_rate_limit_recovery() {
    let client = reqwest::Client::new();
    let (account_id, api_key) = create_test_account(&client).await;

    println!("🧪 Testing rate limit recovery...");

    // Make 65 requests to hit rate limit
    for i in 1..=65 {
        client
            .post(&format!("{}/api/transactions", BASE_URL))
            .header("x-api-key", &api_key)
            .json(&json!({
                "transaction_type": "credit",
                "to_account_id": account_id,
                "amount": 100,
                "currency": "USD",
                "idempotency_key": format!("recovery_test_{}", i)
            }))
            .send()
            .await
            .ok();
    }

    println!("⏳ Waiting 60 seconds for rate limit to reset...");
    sleep(Duration::from_secs(60)).await;

    // Try again after waiting
    let response = client
        .post(&format!("{}/api/transactions", BASE_URL))
        .header("x-api-key", &api_key)
        .json(&json!({
            "transaction_type": "credit",
            "to_account_id": account_id,
            "amount": 100,
            "currency": "USD",
            "idempotency_key": format!("recovery_test_after_wait")
        }))
        .send()
        .await
        .expect("Failed to send request");

    println!("✅ After 60s wait, status: {}", response.status());
    assert_eq!(
        response.status(),
        200,
        "Should succeed after rate limit reset"
    );
}

#[tokio::test]
async fn test_invalid_transaction_amount() {
    let client = reqwest::Client::new();
    let (account_id, api_key) = create_test_account(&client).await;

    // Try to create transaction with negative amount
    let response = client
        .post(&format!("{}/api/transactions", BASE_URL))
        .header("x-api-key", &api_key)
        .json(&json!({
            "transaction_type": "credit",
            "to_account_id": account_id,
            "amount": -1000,
            "currency": "USD"
        }))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 400); // Bad Request

    // Try with zero amount
    let response = client
        .post(&format!("{}/api/transactions", BASE_URL))
        .header("x-api-key", &api_key)
        .json(&json!({
            "transaction_type": "credit",
            "to_account_id": account_id,
            "amount": 0,
            "currency": "USD"
        }))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 400); // Bad Request
}

#[tokio::test]
async fn test_transfer_to_same_account() {
    let client = reqwest::Client::new();
    let (account_id, api_key) = create_test_account(&client).await;

    // Credit account first
    client
        .post(&format!("{}/api/transactions", BASE_URL))
        .header("x-api-key", &api_key)
        .json(&json!({
            "transaction_type": "credit",
            "to_account_id": account_id,
            "amount": 10000,
            "currency": "USD",
            "idempotency_key": format!("same_acc_{}", uuid::Uuid::new_v4())
        }))
        .send()
        .await
        .expect("Failed to credit");

    // Try to transfer to same account
    let response = client
        .post(&format!("{}/api/transactions", BASE_URL))
        .header("x-api-key", &api_key)
        .json(&json!({
            "transaction_type": "transfer",
            "from_account_id": account_id,
            "to_account_id": account_id,
            "amount": 1000,
            "currency": "USD"
        }))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 400); // Bad Request
}
