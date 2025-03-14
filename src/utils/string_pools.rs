use std::collections::HashSet;
use std::sync::Arc;
use parking_lot::RwLock;

/// String interning pool for zero-copy operations
pub struct StringPool {
    pool: Arc<RwLock<HashSet<Arc<String>>>>,
}

impl StringPool {
    pub fn new() -> Self {
        Self {
            pool: Arc::new(RwLock::new(HashSet::new())),
        }
    }
    
    /// Get an existing string from the pool or add it if it doesn't exist
    pub fn intern(&self, s: &str) -> Arc<String> {
        // Fast path: check if string already exists
        {
            let pool = self.pool.read();
            for existing in pool.iter() {
                if existing.as_str() == s {
                    return Arc::clone(existing);
                }
            }
        }
        
        // Slow path: add to pool
        let s_arc = Arc::new(s.to_string());
        let mut pool = self.pool.write();
        
        // Double-check in case another thread added it while we were waiting
        for existing in pool.iter() {
            if existing.as_str() == s {
                return Arc::clone(existing);
            }
        }
        
        pool.insert(Arc::clone(&s_arc));
        s_arc
    }
    
    /// Clear the pool
    pub fn clear(&self) {
        let mut pool = self.pool.write();
        pool.clear();
    }
    
    /// Get the current size of the pool
    pub fn size(&self) -> usize {
        let pool = self.pool.read();
        pool.len()
    }
}
