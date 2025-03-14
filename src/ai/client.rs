use std::error::Error;
use reqwest::{Client, header};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct ApiRequest {
    prompt: String,
    max_tokens: u32,
    temperature: f32,
}

#[derive(Deserialize)]
struct ApiResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    text: String,
}

pub struct AiClient {
    client: Client,
    api_endpoint: String,
    api_key: Option<String>,
}

impl AiClient {
    pub fn new(api_endpoint: String, api_key: Option<String>) -> Result<Self, Box<dyn Error>> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build()?;
        
        Ok(Self {
            client,
            api_endpoint,
            api_key,
        })
    }
    
    pub async fn query(&self, prompt: &str) -> Result<String, Box<dyn Error>> {
        let mut headers = header::HeaderMap::new();
        
        if let Some(api_key) = &self.api_key {
            headers.insert(
                header::AUTHORIZATION,
                header::HeaderValue::from_str(&format!("Bearer {}", api_key))?,
            );
        }
        
        let request = ApiRequest {
            prompt: prompt.to_string(),
            max_tokens: 100,
            temperature: 0.7,
        };
        
        let response = self.client
            .post(&self.api_endpoint)
            .headers(headers)
            .json(&request)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(format!("API error: {}", response.status()).into());
        }
        
        let api_response: ApiResponse = response.json().await?;
        
        if api_response.choices.is_empty() {
            return Err("No response from AI".into());
        }
        
        Ok(api_response.choices[0].text.clone())
    }
}
