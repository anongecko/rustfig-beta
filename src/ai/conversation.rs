use std::collections::VecDeque;
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

use crate::ai::AiProvider;

/// Maximum number of messages to store in conversation history
const MAX_HISTORY_MESSAGES: usize = 20;

/// Represents a message in a conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationMessage {
    /// Role of the message sender (user/assistant)
    pub role: String,
    /// Content of the message
    pub content: String,
    /// Timestamp of the message
    pub timestamp: u64,
}

/// Maintains conversation state for chat sessions
#[derive(Debug)]
pub struct Conversation {
    /// Unique ID for this conversation
    id: String,
    /// Message history
    messages: VecDeque<ConversationMessage>,
    /// When the conversation was created
    created_at: u64,
    /// When the conversation was last used
    last_used: u64,
    /// Where conversation data is stored
    storage_path: PathBuf,
    /// AI provider to use for this conversation
    provider_name: String,
}

impl Conversation {
    /// Create a new conversation
    pub fn new(storage_dir: &Path, provider_name: &str) -> Self {
        // Generate a unique ID based on timestamp
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let id = format!("conv_{}", now);
        let storage_path = storage_dir.join(format!("{}.json", id));
        
        Self {
            id,
            messages: VecDeque::new(),
            created_at: now,
            last_used: now,
            storage_path,
            provider_name: provider_name.to_string(),
        }
    }
    
    /// Load an existing conversation
    pub fn load(path: &Path) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);
        let data: ConversationData = serde_json::from_reader(reader)?;
        
        Ok(Self {
            id: data.id,
            messages: VecDeque::from(data.messages),
            created_at: data.created_at,
            last_used: data.last_used,
            storage_path: path.to_path_buf(),
            provider_name: data.provider_name,
        })
    }
    
    /// Save conversation to disk
    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let data = ConversationData {
            id: self.id.clone(),
            messages: self.messages.iter().cloned().collect(),
            created_at: self.created_at,
            last_used: self.last_used,
            provider_name: self.provider_name.clone(),
        };
        
        let json = serde_json::to_string_pretty(&data)?;
        let mut file = File::create(&self.storage_path)?;
        file.write_all(json.as_bytes())?;
        
        Ok(())
    }
    
    /// Add a user message to the conversation
    pub fn add_user_message(&mut self, content: &str) {
        self.add_message("user", content);
    }
    
    /// Add an assistant message to the conversation
    pub fn add_assistant_message(&mut self, content: &str) {
        self.add_message("assistant", content);
    }
    
    /// Add a message to the conversation
    fn add_message(&mut self, role: &str, content: &str) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        self.messages.push_back(ConversationMessage {
            role: role.to_string(),
            content: content.to_string(),
            timestamp: now,
        });
        
        self.last_used = now;
        
        // Maintain maximum history size
        while self.messages.len() > MAX_HISTORY_MESSAGES {
            self.messages.pop_front();
        }
    }
    
    /// Get all messages in the conversation
    pub fn get_messages(&self) -> Vec<&ConversationMessage> {
        self.messages.iter().collect()
    }
    
    /// Get conversation ID
    pub fn id(&self) -> &str {
        &self.id
    }
    
    /// Get AI provider name
    pub fn provider_name(&self) -> &str {
        &self.provider_name
    }
    
    /// Build prompt with conversation history for the AI
    pub fn build_prompt(&self) -> String {
        let mut prompt = String::new();
        
        for message in &self.messages {
            match message.role.as_str() {
                "user" => prompt.push_str(&format!("User: {}\n", message.content)),
                "assistant" => prompt.push_str(&format!("Assistant: {}\n", message.content)),
                _ => {}
            }
        }
        
        prompt.push_str("Assistant:");
        prompt
    }
}

/// Serializable conversation data for storage
#[derive(Serialize, Deserialize)]
struct ConversationData {
    id: String,
    messages: Vec<ConversationMessage>,
    created_at: u64,
    last_used: u64,
    provider_name: String,
}

/// Manages conversation sessions
pub struct ConversationManager {
    conversations: Vec<Conversation>,
    active_conversation_id: Option<String>,
    storage_dir: PathBuf,
}

impl ConversationManager {
    /// Create a new conversation manager
    pub fn new(storage_dir: &Path) -> Result<Self, Box<dyn Error>> {
        fs::create_dir_all(storage_dir)?;
        
        let mut manager = Self {
            conversations: Vec::new(),
            active_conversation_id: None,
            storage_dir: storage_dir.to_path_buf(),
        };
        
        // Load existing conversations
        manager.load_conversations()?;
        
        Ok(manager)
    }
    
    /// Load existing conversations from the storage directory
    fn load_conversations(&mut self) -> Result<(), Box<dyn Error>> {
        for entry in fs::read_dir(&self.storage_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                match Conversation::load(&path) {
                    Ok(conversation) => {
                        self.conversations.push(conversation);
                    },
                    Err(_) => {
                        // Skip invalid files
                    }
                }
            }
        }
        
        // Sort by most recently used
        self.conversations.sort_by(|a, b| b.last_used.cmp(&a.last_used));
        
        // Set most recent as active if any exist
        if !self.conversations.is_empty() {
            self.active_conversation_id = Some(self.conversations[0].id.clone());
        }
        
        Ok(())
    }
    
    /// Start a new conversation
    pub fn new_conversation(&mut self, provider_name: &str) -> Result<String, Box<dyn Error>> {
        let conversation = Conversation::new(&self.storage_dir, provider_name);
        let id = conversation.id().to_string();
        
        self.conversations.push(conversation);
        self.active_conversation_id = Some(id.clone());
        
        self.save_active_conversation()?;
        
        Ok(id)
    }
    
    /// Get the active conversation, or create one if none exists
    pub fn get_active_conversation(&mut self, provider_name: &str) -> Result<&mut Conversation, Box<dyn Error>> {
        // If no active conversation, create one
        if self.active_conversation_id.is_none() {
            self.new_conversation(provider_name)?;
        }
        
        let active_id = self.active_conversation_id.as_ref().unwrap().clone();
        
        // Find the active conversation
        for conversation in &mut self.conversations {
            if conversation.id() == active_id {
                return Ok(conversation);
            }
        }
        
        // If not found (should not happen), create a new one
        self.new_conversation(provider_name)?;
        let active_id = self.active_conversation_id.as_ref().unwrap().clone();
        
        for conversation in &mut self.conversations {
            if conversation.id() == active_id {
                return Ok(conversation);
            }
        }
        
        Err("Could not find or create conversation".into())
    }
    
    /// Set the active conversation by ID
    pub fn set_active_conversation(&mut self, id: &str) -> Result<(), Box<dyn Error>> {
        // Check if conversation exists
        let exists = self.conversations.iter().any(|c| c.id() == id);
        
        if exists {
            self.active_conversation_id = Some(id.to_string());
            Ok(())
        } else {
            Err(format!("Conversation with ID {} not found", id).into())
        }
    }
    
    /// List all available conversations
    pub fn list_conversations(&self) -> Vec<(String, u64)> {
        self.conversations
            .iter()
            .map(|c| (c.id().to_string(), c.last_used))
            .collect()
    }
    
    /// Save the active conversation
    fn save_active_conversation(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(active_id) = &self.active_conversation_id {
            for conversation in &mut self.conversations {
                if conversation.id() == active_id {
                    conversation.save()?;
                    break;
                }
            }
        }
        
        Ok(())
    }
    
    /// Send a message to the active conversation and get response
    pub async fn send_message(&mut self, 
                             message: &str, 
                             ai_provider: &dyn AiProvider) -> Result<String, Box<dyn Error>> {
        // Get active conversation
        let conversation = match self.active_conversation_id {
            Some(ref id) => {
                // Find the conversation
                let pos = self.conversations.iter().position(|c| c.id() == id);
                match pos {
                    Some(idx) => &mut self.conversations[idx],
                    None => return Err("Active conversation not found".into()),
                }
            },
            None => return Err("No active conversation".into()),
        };
        
        // Add user message
        conversation.add_user_message(message);
        
        // Build prompt with conversation history
        let prompt = conversation.build_prompt();
        
        // Query AI provider
        let response = ai_provider.query(&prompt).await?;
        
        // Add assistant response
        conversation.add_assistant_message(&response);
        
        // Save conversation
        conversation.save()?;
        
        Ok(response)
    }
    
    /// Delete a conversation by ID
    pub fn delete_conversation(&mut self, id: &str) -> Result<(), Box<dyn Error>> {
        let pos = self.conversations.iter().position(|c| c.id() == id);
        
        if let Some(idx) = pos {
            // Get path before removing
            let path = self.conversations[idx].storage_path.clone();
            
            // Remove from memory
            self.conversations.remove(idx);
            
            // Remove from disk
            if path.exists() {
                fs::remove_file(path)?;
            }
            
            // If this was the active conversation, set a new one
            if self.active_conversation_id.as_deref() == Some(id) {
                self.active_conversation_id = self.conversations.first().map(|c| c.id().to_string());
            }
            
            Ok(())
        } else {
            Err(format!("Conversation with ID {} not found", id).into())
        }
    }
    
    /// Run an interactive chat session in the terminal
    pub async fn run_interactive_session(&mut self, 
                                        ai_provider: &dyn AiProvider) -> Result<(), Box<dyn Error>> {
        println!("RustFig AI Chat (type 'exit' to quit, 'clear' to start new conversation)");
        
        // Create a new conversation if none exists
        if self.active_conversation_id.is_none() {
            self.new_conversation(ai_provider.name())?;
        }
        
        // Get active conversation
        let conversation_id = self.active_conversation_id.clone().unwrap();
        println!("Chat ID: {}", conversation_id);
        
        // Print existing conversation
        let conversation = self.get_active_conversation(ai_provider.name())?;
        for msg in conversation.get_messages() {
            let prefix = match msg.role.as_str() {
                "user" => "You: ",
                "assistant" => "AI: ",
                _ => "",
            };
            println!("{}{}", prefix, msg.content);
        }
        
        let stdin = io::stdin();
        let mut reader = stdin.lock();
        let mut input = String::new();
        
        loop {
            // Print prompt
            print!("> ");
            io::stdout().flush()?;
            
            // Read user input
            input.clear();
            reader.read_line(&mut input)?;
            let input = input.trim();
            
            // Check for exit command
            if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("quit") {
                break;
            }
            
            // Check for clear command
            if input.eq_ignore_ascii_case("clear") {
                self.new_conversation(ai_provider.name())?;
                println!("Started new conversation");
                continue;
            }
            
            // Skip empty input
            if input.is_empty() {
                continue;
            }
            
            // Send message and get response
            match self.send_message(input, ai_provider).await {
                Ok(response) => {
                    println!("AI: {}", response);
                },
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
        
        Ok(())
    }
}
