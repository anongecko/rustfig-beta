use std::error::Error;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use reqwest::{Client, header};
use tokio::time::timeout;
use crate::config::OllamaConfig;

/// Ollama API integration for local LLM inference
pub struct OllamaClient {
    client: Client,
    base_url: String,
    model: String,
    timeout_duration: Duration,
}

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
    options: OllamaOptions,
}

#[derive(Serialize)]
struct OllamaOptions {
    temperature: f32,
    num_predict: u32,
}

#[derive(Deserialize)]
struct OllamaResponse {
    model: String,
    response: String,
    done: bool,
}

impl OllamaClient {
    pub fn new(config: &OllamaConfig) -> Result<Self, Box<dyn Error>> {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_secs))
            .build()?;
        
        Ok(Self {
            client,
            base_url: config.api_url.clone(),
            model: config.model.clone(),
            timeout_duration: Duration::from_secs(config.timeout_secs),
        })
    }
    
    /// Check if Ollama is available
    pub async fn is_available(&self) -> bool {
        match self.client.get(&format!("{}/api/tags", self.base_url)).send().await {
            Ok(response) => response.status().is_success(),
            Err(_) => false,
        }
    }
    
    /// Query Ollama model for command prediction or explanation
    pub async fn query(&self, prompt: &str) -> Result<String, Box<dyn Error>> {
        // Create the request
        let request = OllamaRequest {
            model: self.model.clone(),
            prompt: prompt.to_string(),
            stream: false,
            options: OllamaOptions {
                temperature: 0.1, // Low temperature for deterministic responses
                num_predict: 100, // Limit token count for speed
            },
        };
        
        // Execute with timeout
        let response = timeout(
            self.timeout_duration,
            self.client
                .post(&format!("{}/api/generate", self.base_url))
                .json(&request)
                .send()
        ).await??;
        
        if !response.status().is_success() {
            return Err(format!("Ollama API error: {}", response.status()).into());
        }
        
        let ollama_response: OllamaResponse = response.json().await?;
        Ok(ollama_response.response)
    }
    
    /// Generate command suggestions based on user input and context
    pub async fn suggest_command(
        &self, 
        partial_command: &str, 
        current_dir: &str, 
        environment: &str
    ) -> Result<Vec<String>, Box<dyn Error>> {
        let prompt = format!(
            "You are a terminal assistant that completes commands. Current directory: {}\nEnvironment: {}\n\
            Provide 3 possible completions for this command: '{}'\n\
            Format as JSON array of strings with just the commands, no explanation.",
            current_dir, environment, partial_command
        );
        
        let result = self.query(&prompt).await?;
        
        // Try to extract JSON array from the response
        if let Some(json_start) = result.find('[') {
            if let Some(json_end) = result.rfind(']') {
                let json_str = &result[json_start..=json_end];
                match serde_json::from_str::<Vec<String>>(json_str) {
                    Ok(commands) => return Ok(commands),
                    Err(_) => {
                        // If JSON parsing fails, try to extract line by line
                        return Ok(result
                            .lines()
                            .filter(|line| line.starts_with("- ") || line.starts_with("* "))
                            .map(|line| line[2..].trim().to_string())
                            .collect());
                    }
                }
            }
        }
        
        // Fallback: just split by newlines and clean up
        Ok(result
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.trim().to_string())
            .collect())
    }
    
    /// Explain what a command does
    pub async fn explain_command(&self, command: &str) -> Result<String, Box<dyn Error>> {
        let prompt = format!(
            "You are a helpful terminal assistant. Briefly explain what this command does in 1-2 sentences: '{}'",
            command
        );
        
        self.query(&prompt).await
    }
}
