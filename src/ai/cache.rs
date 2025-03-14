use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::sync::Arc;
use parking_lot::RwLock;

pub struct AiCache {
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    max_entries: usize,
    ttl: Duration,
}

struct CacheEntry {
    value: String,
    timestamp: Instant,
}

impl AiCache {
    pub fn new(max_entries: usize, ttl_seconds: u64) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            max_entries,
            ttl: Duration::from_secs(ttl_seconds),
        }
    }
    
    pub fn get(&self, key: &str) -> Option<String> {
        let cache = self.cache.read();
        
        if let Some(entry) = cache.get(key) {
            if entry.timestamp.elapsed() < self.ttl {
                return Some(entry.value.clone());
            }
        }
        
        None
    }
    
    pub fn set(&self, key: &str, value: String) {
        let mut cache = self.cache.write();
        
        // Clean up expired entries if cache is full
        if cache.len() >= self.max_entries {
            let now = Instant::now();
            cache.retain(|_, v| v.timestamp.elapsed() < self.ttl);
            
            // If still full after cleanup, remove oldest entry
            if cache.len() >= self.max_entries {
                let oldest_key = cache.iter()
                    .min_by_key(|(_, v)| v.timestamp)
                    .map(|(k, _)| k.clone());
                
                if let Some(oldest) = oldest_key {
                    cache.remove(&oldest);
                }
            }
        }
        
        cache.insert(
            key.to_string(), 
            CacheEntry {
                value,
                timestamp: Instant::now(),
            }
        );
    }
    
    pub fn clear(&self) {
        let mut cache = self.cache.write();
        cache.clear();
    }
}
