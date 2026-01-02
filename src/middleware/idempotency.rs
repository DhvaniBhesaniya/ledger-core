#![allow(dead_code)]
use parking_lot::RwLock;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct IdempotencyCache {
    cache: RwLock<HashMap<String, CachedResponse>>,
}

#[derive(Clone)]
pub struct CachedResponse {
    pub status: u16,
    pub body: String,
    pub timestamp: u64,
}

impl IdempotencyCache {
    pub fn new() -> Self {
        IdempotencyCache {
            cache: RwLock::new(HashMap::new()),
        }
    }

    pub fn get(&self, key: &str) -> Option<CachedResponse> {
        let cache = self.cache.read();
        cache.get(key).cloned()
    }

    pub fn set(&self, key: String, status: u16, body: String) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        self.cache.write().insert(
            key,
            CachedResponse {
                status,
                body,
                timestamp,
            },
        );
    }
}
