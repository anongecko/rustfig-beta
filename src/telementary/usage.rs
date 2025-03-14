use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tokio::time;
use uuid::Uuid;
use crate::config::TelemetryConfig;

use super::is_telemetry_enabled;

const USAGE_FILE_NAME: &str = "usage_data.json";
const UPLOAD_INTERVAL: Duration = Duration::from_secs(3600); // 1 hour

/// Tracks usage statistics for RustFig
pub struct UsageTracker {
    /// User ID (anonymous)
    user_id: String,
    /// Whether telemetry is enabled
    config: TelemetryConfig,
    /// Path to usage data file
    data_path: PathBuf,
    /// Event queue
    event_queue: Mutex<Vec<UsageEvent>>,
    /// Shutdown signal
    shutdown_tx: Option<mpsc::Sender<()>>,
}

/// Represents a usage event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsageEvent {
    /// Type of event
    pub event_type: String,
    /// Event properties
    pub properties: HashMap<String, String>,
    /// Timestamp
    pub timestamp: u64,
}

/// Aggregated usage data
#[derive(Clone, Debug, Serialize, Deserialize)]
struct UsageData {
    /// User ID (anonymous)
    user_id: String,
    /// Installation ID
    installation_id: String,
    /// RustFig version
    version: String,
    /// Operating system
    os: String,
    /// OS version
    os_version: String,
    /// CPU architecture
    arch: String,
    /// Usage events
    events: Vec<UsageEvent>,
    /// Last upload timestamp
    last_upload: u64,
}

impl UsageTracker {
    /// Create a new usage tracker
    pub fn new(config: TelemetryConfig) -> Self {
        // Determine data path
        let data_dir = config.data_dir.clone()
            .unwrap_or_else(|| {
                dirs::data_dir()
                    .unwrap_or_else(|| PathBuf::from("."))
                    .join("rustfig")
                    .join("telemetry")
            });
        
        // Get or create user ID
        let user_id = Self::get_or_create_user_id(&data_dir).unwrap_or_else(|_| {
            Uuid::new_v4().to_string()
        });
        
        Self {
            user_id,
            config,
            data_path: data_dir.join(USAGE_FILE_NAME),
            event_queue: Mutex::new(Vec::new()),
            shutdown_tx: None,
        }
    }
    
    /// Start the usage tracker
    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        // Create directory if it doesn't exist
        if let Some(parent) = self.data_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Start upload task
        if is_telemetry_enabled() {
            let (tx, mut rx) = mpsc::channel::<()>(1);
            self.shutdown_tx = Some(tx);
            
            let data_path = self.data_path.clone();
            let upload_url = self.config.upload_url.clone();
            
            tokio::spawn(async move {
                let mut interval = time::interval(UPLOAD_INTERVAL);
                
                loop {
                    tokio::select! {
                        _ = interval.tick() => {
                            if let Err(e) = Self::upload_usage_data(&data_path, &upload_url).await {
                                eprintln!("Failed to upload usage data: {}", e);
                            }
                        }
                        _ = rx.recv() => {
                            break;
                        }
                    }
                }
            });
        }
        
        Ok(())
    }
    
    /// Stop the usage tracker
    pub async fn stop(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(()).await;
        }
    }
    
    /// Record a usage event
    pub fn record_event(&self, event_type: &str, properties: HashMap<String, String>) {
        if !is_telemetry_enabled() {
            return;
        }
        
        // Create event
        let event = UsageEvent {
            event_type: event_type.to_string(),
            properties,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        };
        
        // Queue event
        if let Ok(mut queue) = self.event_queue.lock() {
            queue.push(event.clone());
            
            // Save immediately if queue gets too large
            if queue.len() >= 100 {
                let events = std::mem::take(&mut *queue);
                if let Err(e) = self.save_events(&events) {
                    eprintln!("Failed to save usage events: {}", e);
                    
                    // Put events back in queue if save failed
                    queue.extend(events);
                }
            }
        }
    }
    
    /// Record command execution
    pub fn record_command(&self, command: &str) {
        let mut properties = HashMap::new();
        properties.insert("command".to_string(), command.to_string());
        
        self.record_event("command_executed", properties);
    }
    
    /// Record suggestion acceptance
    pub fn record_suggestion_accepted(&self, suggestion: &str, source: &str) {
        let mut properties = HashMap::new();
        properties.insert("suggestion".to_string(), suggestion.to_string());
        properties.insert("source".to_string(), source.to_string());
        
        self.record_event("suggestion_accepted", properties);
    }
    
    /// Record ghost text acceptance
    pub fn record_ghost_accepted(&self, ghost_text: &str) {
        let mut properties = HashMap::new();
        properties.insert("ghost_text".to_string(), ghost_text.to_string());
        
        self.record_event("ghost_accepted", properties);
    }
    
    /// Record AI query
    pub fn record_ai_query(&self, query_type: &str) {
        let mut properties = HashMap::new();
        properties.insert("type".to_string(), query_type.to_string());
        
        self.record_event("ai_query", properties);
    }
    
    /// Flush events to disk
    pub fn flush(&self) -> Result<(), Box<dyn Error>> {
        if let Ok(mut queue) = self.event_queue.lock() {
            let events = std::mem::take(&mut *queue);
            if !events.is_empty() {
                self.save_events(&events)?;
            }
        }
        
        Ok(())
    }
    
    /// Save events to disk
    fn save_events(&self, new_events: &[UsageEvent]) -> Result<(), Box<dyn Error>> {
        // Load existing data
        let mut data = self.load_usage_data()?;
        
        // Add new events
        data.events.extend_from_slice(new_events);
        
        // Save data
        let json = serde_json::to_string_pretty(&data)?;
        fs::write(&self.data_path, json)?;
        
        Ok(())
    }
    
    /// Load usage data from disk
    fn load_usage_data(&self) -> Result<UsageData, Box<dyn Error>> {
        if self.data_path.exists() {
            let json = fs::read_to_string(&self.data_path)?;
            let data: UsageData = serde_json::from_str(&json)?;
            Ok(data)
        } else {
            // Create new data
            Ok(UsageData {
                user_id: self.user_id.clone(),
                installation_id: Uuid::new_v4().to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                os: std::env::consts::OS.to_string(),
                os_version: std::env::consts::FAMILY.to_string(),
                arch: std::env::consts::ARCH.to_string(),
                events: Vec::new(),
                last_upload: 0,
            })
        }
    }
    
    /// Get or create user ID
    fn get_or_create_user_id(data_dir: &Path) -> Result<String, Box<dyn Error>> {
        let id_file = data_dir.join("user_id");
        
        if id_file.exists() {
            let id = fs::read_to_string(id_file)?;
            Ok(id.trim().to_string())
        } else {
            let id = Uuid::new_v4().to_string();
            fs::create_dir_all(data_dir)?;
            fs::write(id_file, &id)?;
            Ok(id)
        }
    }
    
    /// Upload usage data
    async fn upload_usage_data(data_path: &Path, upload_url: &str) -> Result<(), Box<dyn Error>> {
        if !data_path.exists() {
            return Ok(());
        }
        
        // Load data
        let json = fs::read_to_string(data_path)?;
        let mut data: UsageData = serde_json::from_str(&json)?;
        
        // Check if we have events to upload
        if data.events.is_empty() {
            return Ok(());
        }
        
        // Upload data
        let client = reqwest::Client::new();
        let response = client.post(upload_url)
            .json(&data)
            .send()
            .await?;
        
        if response.status().is_success() {
            // Clear events and update timestamp
            data.events.clear();
            data.last_upload = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            
            // Save updated data
            let new_json = serde_json::to_string_pretty(&data)?;
            fs::write(data_path, new_json)?;
        }
        
        Ok(())
    }
}
