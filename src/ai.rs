// Re-export from the ai module
pub mod client;
pub mod cache;
pub mod ollama;

use std::error::Error;
use std::time::Duration;
use async_trait::async_trait;

pub use self::client::AiClient;
pub use self::cache::AiCache;
pub use self::ollama::OllamaClient;

/// Common trait for AI providers
#[async_trait]
pub trait AiProvider: Send + Sync {
    /// Query the AI with a prompt
    async fn query(&self, prompt: &str) -> Result<String, Box<dyn Error>>;
    
    /// Check if the provider is available
    async fn is_available(&self) -> bool;
    
    /// Get the name of the provider
    fn name(&self) -> &str;
}

#[async_trait]
impl AiProvider for AiClient {
    async fn query(&self, prompt: &str) -> Result<String, Box<dyn Error>> {
        self.query(prompt).await
    }
    
    async fn is_available(&self) -> bool {
        true // Simple API client is always considered available
    }
    
    fn name(&self) -> &str {
        "OpenAI-compatible API"
    }
}

#[async_trait]
impl AiProvider for OllamaClient {
    async fn query(&self, prompt: &str) -> Result<String, Box<dyn Error>> {
        self.query(prompt).await
    }
    
    async fn is_available(&self) -> bool {
        self.is_available().await
    }
    
    fn name(&self) -> &str {
        "Ollama"
    }
}

/// Factory for creating AI providers
pub struct AiProviderFactory;

impl AiProviderFactory {
    /// Create an AI provider based on configuration
    pub async fn create_provider(
        config: &crate::config::Config
    ) -> Option<Box<dyn AiProvider>> {
        // Try Ollama first if enabled
        if let Some(ollama_config) = &config.ollama {
            if ollama_config.enabled {
                if let Ok(client) = OllamaClient::new(ollama_config) {
                    if client.is_available().await {
                        return Some(Box::new(client));
                    }
                }
            }
        }
        
        // Fall back to API if enabled
        if config.ai.enabled {
            if let Ok(client) = AiClient::new(
                config.ai.api_endpoint.clone(),
                config.ai.api_key.clone()
            ) {
                return Some(Box::new(client));
            }
        }
        
        None
    }
}
