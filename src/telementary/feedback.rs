use std::error::Error;
use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use reqwest::Client;
use crate::config::TelemetryConfig;

use super::is_telemetry_enabled;

/// Collects user feedback
pub struct FeedbackCollector {
    /// Feedback upload URL
    upload_url: String,
    /// HTTP client
    client: Client,
    /// Whether telemetry is enabled
    enabled: bool,
    /// Path to local feedback storage
    storage_path: PathBuf,
}

/// User feedback data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feedback {
    /// Unique ID for this feedback
    pub id: String,
    /// Feedback category
    pub category: FeedbackCategory,
    /// Feedback rating (1-5)
    pub rating: Option<u8>,
    /// Feedback text content
    pub content: String,
    /// Contact email (optional)
    pub email: Option<String>,
    /// Whether this is a bug report
    pub is_bug_report: bool,
    /// System information
    pub system_info: SystemInfo,
    /// Timestamp
    pub timestamp: u64,
}

/// Feedback categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeedbackCategory {
    /// General feedback
    General,
    /// Suggestion system feedback
    Suggestions,
    /// AI integration feedback
    AI,
    /// UI feedback
    UI,
    /// Performance feedback
    Performance,
    /// Bug report
    BugReport,
    /// Feature request
    FeatureRequest,
}

/// System information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    /// RustFig version
    pub version: String,
    /// Operating system
    pub os: String,
    /// OS version
    pub os_version: String,
    /// CPU architecture
    pub arch: String,
    /// Shell
    pub shell: String,
    /// Terminal
    pub terminal: String,
}

impl FeedbackCollector {
    /// Create a new feedback collector
    pub fn new(config: TelemetryConfig) -> Self {
        // Determine storage path
        let storage_path = config.data_dir.clone()
            .unwrap_or_else(|| {
                dirs::data_dir()
                    .unwrap_or_else(|| PathBuf::from("."))
                    .join("rustfig")
                    .join("feedback")
            });
        
        // Create directory if it doesn't exist
        if let Err(e) = fs::create_dir_all(&storage_path) {
            eprintln!("Failed to create feedback storage directory: {}", e);
        }
        
        Self {
            upload_url: config.feedback_url.clone(),
            client: Client::new(),
            enabled: is_telemetry_enabled(),
            storage_path,
        }
    }
    
    /// Submit feedback
    pub async fn submit_feedback(&self, feedback: Feedback) -> Result<(), Box<dyn Error>> {
        // Save locally always
        self.save_feedback_locally(&feedback)?;
        
        // Upload if telemetry is enabled
        if self.enabled {
            self.upload_feedback(&feedback).await?;
        }
        
        Ok(())
    }
    
    /// Save feedback locally
    fn save_feedback_locally(&self, feedback: &Feedback) -> Result<(), Box<dyn Error>> {
        let file_path = self.storage_path.join(format!("feedback_{}.json", feedback.id));
        let json = serde_json::to_string_pretty(feedback)?;
        fs::write(file_path, json)?;
        Ok(())
    }
    
    /// Upload feedback to server
    async fn upload_feedback(&self, feedback: &Feedback) -> Result<(), Box<dyn Error>> {
        let response = self.client.post(&self.upload_url)
            .json(feedback)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(format!("Failed to upload feedback: {}", response.status()).into());
        }
        
        Ok(())
    }
    
    /// Create new feedback object
    pub fn create_feedback(&self, 
                          category: FeedbackCategory, 
                          content: String, 
                          rating: Option<u8>,
                          email: Option<String>,
                          is_bug_report: bool) -> Feedback {
        Feedback {
            id: Uuid::new_v4().to_string(),
            category,
            rating,
            content,
            email,
            is_bug_report,
            system_info: SystemInfo {
                version: env!("CARGO_PKG_VERSION").to_string(),
                os: std::env::consts::OS.to_string(),
                os_version: std::env::consts::FAMILY.to_string(),
                arch: std::env::consts::ARCH.to_string(),
                shell: std::env::var("SHELL").unwrap_or_default(),
                terminal: std::env::var("TERM").unwrap_or_default(),
            },
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }
    
    /// List locally saved feedback
    pub fn list_local_feedback(&self) -> Result<Vec<Feedback>, Box<dyn Error>> {
        let mut feedback_list = Vec::new();
        
        for entry in fs::read_dir(&self.storage_path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                let json = fs::read_to_string(&path)?;
                if let Ok(feedback) = serde_json::from_str::<Feedback>(&json) {
                    feedback_list.push(feedback);
                }
            }
        }
        
        Ok(feedback_list)
    }
}
