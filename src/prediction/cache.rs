use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use parking_lot::RwLock;
use hashbrown::hash_map::Entry;
use super::models::Prediction;

/// Ultra-fast prediction cache for sub-millisecond response times
pub struct PredictionCache {
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    max_entries: usize,
    entry_ttl: Duration,
}

/// A cache entry with expiration time
struct CacheEntry {
    predictions: Vec<Prediction>,
    timestamp: Instant,
}

impl PredictionCache {
    pub fn new(max_entries: usize, entry_ttl: Duration) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::with_capacity(max_entries))),
            max_entries,
            entry_ttl,
        }
    }
    
    /// Get predictions from cache if they exist and aren't expired
    pub fn get(&self, key: &str) -> Option<Vec<Prediction>> {
        let cache = self.cache.read();
        
        if let Some(entry) = cache.get(key) {
            if entry.timestamp.elapsed() < self.entry_ttl {
                return Some(entry.predictions.clone());
            }
        }
        
        None
    }
    
    /// Set predictions in cache
    pub fn set(&self, key: String, predictions: Vec<Prediction>) {
        if predictions.is_empty() {
            return;
        }
        
        let mut cache = self.cache.write();
        
        // If cache is full, remove oldest entries
        if cache.len() >= self.max_entries {
            self.cleanup_cache(&mut cache);
        }
        
        cache.insert(key, CacheEntry {
            predictions,
            timestamp: Instant::now(),
        });
    }
    
    /// Check if cache contains an entry that's not expired
    pub fn contains(&self, key: &str) -> bool {
        let cache = self.cache.read();
        
        if let Some(entry) = cache.get(key) {
            entry.timestamp.elapsed() < self.entry_ttl
        } else {
            false
        }
    }
    
    /// Update specific prediction in cache if it exists
    pub fn update_prediction(&self, key: &str, old_prediction: &Prediction, new_prediction: Prediction) -> bool {
        let mut cache = self.cache.write();
        
        if let Entry::Occupied(mut entry) = cache.entry(key.to_string()) {
            let cache_entry = entry.get_mut();
            
            // Find and update the prediction
            for pred in &mut cache_entry.predictions {
                if pred.text == old_prediction.text {
                    *pred = new_prediction;
                    return true;
                }
            }
        }
        
        false
    }
    
    /// Remove entries that have expired or if cache is too large
    fn cleanup_cache(&self, cache: &mut HashMap<String, CacheEntry>) {
        // First remove expired entries
        let now = Instant::now();
        cache.retain(|_, entry| now.duration_since(entry.timestamp) < self.entry_ttl);
        
        // If still too large, remove oldest entries
        if cache.len() >= self.max_entries {
            let mut entries: Vec<_> = cache.iter().collect();
            entries.sort_by_key(|(_, entry)| entry.timestamp);
            
            // Remove oldest third of entries
            let to_remove = self.max_entries / 3;
            for (key, _) in entries.iter().take(to_remove) {
                cache.remove(*key);
            }
        }
    }
    
    /// Clear the entire cache
    pub fn clear(&self) {
        let mut cache = self.cache.write();
        cache.clear();
    }
}
