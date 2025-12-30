#![allow(dead_code)]
use hex;
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};

pub fn hash_api_key(key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(key);
    format!("{:x}", hasher.finalize())
}

pub fn generate_webhook_signature(timestamp: i64, body: &str, secret: &str) -> String {
    let content = format!("{}.{}", timestamp, body);
    let mut mac =
        Hmac::<Sha256>::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");
    mac.update(content.as_bytes());
    hex::encode(mac.finalize().into_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // cargo test hash_api_key   or cargo test hash_api_key -- --nocapture
    fn test_hash_api_key() {
        let key = "sk_prod_test_key_123";
        let hash1 = hash_api_key(key);
        // println!("Hash: {}", hash1);
        // let hash2 = hash_api_key(key);
        // println!("Hash: {}", hash2);
        // let prefix = std::env::var("API_KEY_PREFIX").unwrap_or_else(|_| "sk_prod_".to_string());
        // let raw_key = format!("{}{}", prefix, Uuid::new_v4().to_string());
        // let key_hash = hash_api_key(&raw_key);
        // let key_prefix = raw_key[..20].to_string();
        // println!("Raw Key: {}", raw_key);
        // println!("Key Prefix: {}", key_prefix);

        // Hash should be consistent
        assert_eq!(
            hash1,
            "62ed4404305fafaa9061656b8c27f1611dff56e0b641f9ff5e7cd1e06ff075a7"
        );

        // Hash should be SHA256 length (64 chars)
        assert_eq!(hash1.len(), 64);

        // let other_key = "sk_test_987654321";
        // let other_hash = hash_api_key(other_key);

        // // Different keys should produce different hashes
        // assert_ne!(hash1, other_hash);
    }
}
