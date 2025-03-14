use std::time::{Duration, Instant};
use hashbrown::HashMap;

/// Confidence level for a prediction
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Confidence(pub f32);

impl Confidence {
    pub const LOW: Confidence = Confidence(0.3);
    pub const MEDIUM: Confidence = Confidence(0.6);
    pub const HIGH: Confidence = Confidence(0.9);
    
    #[inline]
    pub fn value(&self) -> f32 {
        self.0
    }
    
    #[inline]
    pub fn is_high_enough_for_ghost(&self) -> bool {
        self.0 >= 0.4
    }
}

/// Type of prediction
#[derive(Debug, Clone, PartialEq)]
pub enum PredictionType {
    /// Complete command prediction
    FullCommand,
    /// Next token/word prediction
    NextToken,
    /// Flag or option suggestion
    Flag,
    /// Argument value suggestion
    ArgumentValue,
    /// File or path suggestion
    Path,
}

/// Source of the prediction
#[derive(Debug, Clone, PartialEq)]
pub enum PredictionSource {
    /// From command history
    History,
    /// From current directory context
    DirectoryContext,
    /// From project-specific analysis
    ProjectType,
    /// From git status
    GitContext,
    /// From common command patterns
    CommandPatterns,
    /// From user's personal patterns
    UserPatterns,
}

/// A command prediction with metadata
#[derive(Debug, Clone)]
pub struct Prediction {
    /// The predicted text
    pub text: String,
    
    /// Display text (may include additional info)
    pub display_text: String,
    
    /// Type of prediction
    pub prediction_type: PredictionType,
    
    /// Source of the prediction
    pub source: PredictionSource,
    
    /// Confidence score (0.0-1.0)
    pub confidence: Confidence,
    
    /// How many tokens this prediction completes
    pub tokens_completed: usize,
    
    /// Optional explanation of what this command does
    pub explanation: Option<String>,
    
    /// User acceptance count (how often user selected this)
    pub usage_count: usize,
    
    /// Generation timestamp
    pub timestamp: Instant,
    
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl Prediction {
    /// Create a new prediction
    pub fn new(text: String, prediction_type: PredictionType, source: PredictionSource, confidence: Confidence) -> Self {
        Self {
            display_text: text.clone(),
            text,
            prediction_type,
            source,
            confidence,
            tokens_completed: 1,
            explanation: None,
            usage_count: 0,
            timestamp: Instant::now(),
            metadata: HashMap::new(),
        }
    }
    
    /// Check if prediction is still fresh
    #[inline]
    pub fn is_fresh(&self, max_age: Duration) -> bool {
        self.timestamp.elapsed() < max_age
    }
    
    /// Get text for ghost display
    #[inline]
    pub fn get_ghost_text(&self, current_input: &str) -> String {
        if current_input.is_empty() {
            return self.text.clone();
        }
        
        // Only show the part of the prediction that hasn't been typed yet
        if self.text.starts_with(current_input) {
            self.text[current_input.len()..].to_string()
        } else {
            String::new()
        }
    }
    
    /// Add metadata to the prediction
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }
    
    /// Add explanation to the prediction
    pub fn with_explanation(mut self, explanation: &str) -> Self {
        self.explanation = Some(explanation.to_string());
        self
    }
    
    /// Set number of tokens completed
    pub fn with_tokens_completed(mut self, tokens: usize) -> Self {
        self.tokens_completed = tokens;
        self
    }
    
    /// Set display text
    pub fn with_display_text(mut self, display_text: &str) -> Self {
        self.display_text = display_text.to_string();
        self
    }
    
    /// Record that user accepted this prediction
    pub fn record_usage(&mut self) {
        self.usage_count += 1;
        self.timestamp = Instant::now();
    }
}
