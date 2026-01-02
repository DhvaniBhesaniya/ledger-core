use std::collections::HashMap;
use parking_lot::RwLock;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct RateLimiter {
    buckets: RwLock<HashMap<i64, TokenBucket>>,
}

#[derive(Clone, Copy)]
struct TokenBucket {
    tokens: i32,
    last_reset: u64,
}

impl RateLimiter {
    pub fn new() -> Self {
        RateLimiter {
            buckets: RwLock::new(HashMap::new()),
        }
    }

    pub fn check_limit(&self, key_id: i64, rate_limit: i32) -> Result<(), ()> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut buckets = self.buckets.write();

        let bucket = buckets
            .entry(key_id)
            .or_insert_with(|| TokenBucket {
                tokens: rate_limit,
                last_reset: now,
            });

        if now - bucket.last_reset >= 60 {
            bucket.tokens = rate_limit;
            bucket.last_reset = now;
        }

        if bucket.tokens > 0 {
            bucket.tokens -= 1;
            Ok(())
        } else {
            Err(())
        }
    }
}