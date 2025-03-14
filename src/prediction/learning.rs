use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use parking_lot::RwLock;
use super::models::Prediction;

// Constants for learning system
const MAX_PATTERNS: usize = 10000;
const SAVE_INTERVAL: usize = 100; // Save after this many new entries

/// System that learns from user behavior to improve predictions
pub struct UserLearningSystem {
    data_file: PathBuf,
    command_patterns: Arc<RwLock<HashMap<String, PatternData>>>,
    context_patterns: Arc<RwLock<HashMap<String, Vec<ContextPattern>>>>,
    modification_count: Arc<AtomicUsize>,
}

/// Data about a command pattern
#[derive(Debug, Clone)]
struct PatternData {
    count: usize,
    last_used: u64, // Timestamp
}

/// Context-based pattern
#[derive(Debug, Clone)]
struct ContextPattern {
    context_key: String,
    command: String,
    count: usize,
}

impl UserLearningSystem {
    pub fn new(data_dir: &Path) -> Self {
        // Ensure data directory exists
        let data_dir = if data_dir.exists() && data_dir.is_dir() {
            data_dir.to_path_buf()
        } else {
            Path::new(&dirs::home_dir().unwrap_or_default())
                .join(".rustfig")
                .join("data")
        };
        
        fs::create_dir_all(&data_dir).unwrap_or_default();
        let data_file = data_dir.join("learning_data.bin");
        
        let mut system = Self {
            data_file,
            command_patterns: Arc::new(RwLock::new(HashMap::new())),
            context_patterns: Arc::new(RwLock::new(HashMap::new())),
            modification_count: Arc::new(AtomicUsize::new(0)),
        };
        
        // Load existing data
        system.load_data();
        
        system
    }
    
    /// Record a prediction that the user accepted
    pub fn record_accepted_prediction(&self, prediction: &Prediction) {
        let command = prediction.text.clone();
        
        // Update command pattern
        {
            let mut patterns = self.command_patterns.write();
            let entry = patterns.entry(command.clone()).or_insert_with(|| PatternData {
                count: 0,
                last_used: 0,
            });
            
            entry.count += 1;
            entry.last_used = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
        }
        
        // Record modification and possibly save
        let count = self.modification_count.fetch_add(1, Ordering::SeqCst);
        if count % SAVE_INTERVAL == 0 {
            self.save_data();
        }
    }
    
    /// Adjust prediction scores based on learned patterns
    pub fn adjust_scores(&self, predictions: &mut Vec<Prediction>, input: &str) {
        let patterns = self.command_patterns.read();
        
        for prediction in predictions.iter_mut() {
            // Check if this prediction matches a learned pattern
            if let Some(pattern) = patterns.get(&prediction.text) {
                let boost = (pattern.count as f32).min(10.0) / 10.0; // Max boost of 1.0
                let current = prediction.confidence.0;
                prediction.confidence.0 = (current + boost).min(1.0);
            }
        }
    }
    
    /// Load learning data from disk
    fn load_data(&mut self) {
        if !self.data_file.exists() {
            return;
        }
        
        match File::open(&self.data_file) {
            Ok(mut file) => {
                let mut buffer = Vec::new();
                if file.read_to_end(&mut buffer).is_ok() {
                    if let Ok(data) = bincode::deserialize::<SerializedData>(&buffer) {
                        *self.command_patterns.write() = data.command_patterns;
                    }
                }
            },
            Err(_) => {
                // Failed to open file - start fresh
            }
        }
    }
    
    /// Save learning data to disk
    fn save_data(&self) {
        let data = SerializedData {
            command_patterns: self.command_patterns.read().clone(),
            version: 1,
        };
        
        if let Ok(serialized) = bincode::serialize(&data) {
            if let Ok(mut file) = File::create(&self.data_file) {
                let _ = file.write_all(&serialized);
            }
        }
    }
}

/// Data structure for serialization
#[derive(serde::Serialize, serde::Deserialize)]
struct SerializedData {
    command_patterns: HashMap<String, PatternData>,
    version: u32,
}
