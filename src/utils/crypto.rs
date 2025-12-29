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
