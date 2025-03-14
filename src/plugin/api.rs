use std::error::Error;
use crate::suggestion::{Suggestion, SuggestionKind};

// Plugin API for extending RustFig
pub trait CompletionProvider: Send + Sync {
    fn name(&self) -> &str;
    
    fn can_provide_completions(&self, command: &str) -> bool;
    
    fn provide_completions(&self, command: &str, args: &[&str], current_arg: &str) -> Vec<Suggestion>;
}

// Helper to create a standard suggestion
pub fn create_suggestion(display: &str, completion: &str, kind: SuggestionKind, description: &str) -> Suggestion {
    Suggestion::new(
        display.to_string(),
        completion.to_string(),
        kind
    )
    .with_description(description.to_string())
    .with_score(80.0)
}

// Registry for completion providers
pub struct CompletionRegistry {
    providers: Vec<Box<dyn CompletionProvider>>,
}

impl CompletionRegistry {
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
        }
    }
    
    pub fn register(&mut self, provider: Box<dyn CompletionProvider>) {
        self.providers.push(provider);
    }
    
    pub fn get_completions(&self, command: &str, args: &[&str], current_arg: &str) -> Vec<Suggestion> {
        let mut all_suggestions = Vec::new();
        
        for provider in &self.providers {
            if provider.can_provide_completions(command) {
                let suggestions = provider.provide_completions(command, args, current_arg);
                all_suggestions.extend(suggestions);
            }
        }
        
        all_suggestions
    }
}
