use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use parking_lot::RwLock;
use super::{
    models::{Prediction, PredictionType, PredictionSource, Confidence},
    context_analyzer::ContextAnalyzer,
    ranking::PredictionRanker,
    learning::UserLearningSystem,
    cache::PredictionCache,
};
use crate::{
    config::Config,
    suggestion::context::Context,
    shell::parser::{CommandParser, ParsedCommand},
    utils::perf_metrics::PerformanceMetrics,
};

/// Core prediction engine responsible for generating high-quality, low-latency predictions
pub struct PredictionEngine {
    config: Arc<Config>,
    context_analyzer: ContextAnalyzer,
    prediction_ranker: PredictionRanker,
    user_learning: UserLearningSystem,
    prediction_cache: PredictionCache,
    command_parser: CommandParser,
    performance_metrics: PerformanceMetrics,
}

impl PredictionEngine {
    pub fn new(config: &Config) -> Self {
        Self {
            config: Arc::new(config.clone()),
            context_analyzer: ContextAnalyzer::new(),
            prediction_ranker: PredictionRanker::new(),
            user_learning: UserLearningSystem::new(&config.general.user_data_dir),
            prediction_cache: PredictionCache::new(1000, Duration::from_secs(300)),
            command_parser: CommandParser::new(),
            performance_metrics: PerformanceMetrics::new("prediction_engine"),
        }
    }
    
    /// Generate predictions for the current input with ultra-low latency
    pub async fn predict(&self, input: &str, limit: usize) -> Vec<Prediction> {
        let _timing = self.performance_metrics.measure_operation("predict");
        
        // Fast path: Check cache first
        if let Some(predictions) = self.prediction_cache.get(input) {
            return predictions;
        }
        
        // Parse command and current context
        let cursor_pos = input.len(); // Assume cursor at end
        let parsed = match self.command_parser.parse(input, cursor_pos) {
            Ok(parsed) => parsed,
            Err(_) => return Vec::new(),
        };
        
        // Analyze context (filesystem, git, project type, etc.)
        let context = self.context_analyzer.analyze(input, &parsed).await;
        
        // Generate predictions concurrently from multiple sources
        let predictions = self.generate_predictions(input, &parsed, &context, limit).await;
        
        // Cache results
        self.prediction_cache.set(input.to_string(), predictions.clone());
        
        predictions
    }
    
    /// Generate predictions from multiple sources concurrently
    async fn generate_predictions(
        &self,
        input: &str,
        parsed: &ParsedCommand<'_>,
        context: &Context,
        limit: usize
    ) -> Vec<Prediction> {
        let (tx, mut rx) = mpsc::channel(8);
        
        // Clone what we need for async blocks
        let input_owned = input.to_string();
        let context_clone = context.clone();
        let tx1 = tx.clone();
        let tx2 = tx.clone();
        let tx3 = tx.clone();
        let tx4 = tx.clone();
        
        // 1. Generate history-based predictions (common commands)
        tokio::spawn(async move {
            let predictions = Self::predict_from_history(&input_owned);
            let _ = tx1.send(predictions).await;
        });
        
        // 2. Generate directory context predictions (files, paths)
        tokio::spawn(async move {
            let predictions = Self::predict_from_directory_context(&input_owned, &context_clone);
            let _ = tx2.send(predictions).await;
        });
        
        // 3. Generate project-specific predictions
        tokio::spawn(async move {
            let predictions = Self::predict_from_project_context(&input_owned, &context_clone);
            let _ = tx3.send(predictions).await;
        });
        
        // 4. Generate git-aware predictions if in a git repo
        if context.in_git_repo {
            tokio::spawn(async move {
                let predictions = Self::predict_from_git_context(&input_owned, &context_clone);
                let _ = tx4.send(predictions).await;
            });
        }
        
        // Drop original sender
        drop(tx);
        
        // Collect all predictions
        let mut all_predictions = Vec::new();
        while let Some(mut predictions) = rx.recv().await {
            all_predictions.append(&mut predictions);
        }
        
        // Apply user learning to adjust scores
        self.user_learning.adjust_scores(&mut all_predictions, input);
        
        // Rank and limit predictions
        self.prediction_ranker.rank(&mut all_predictions);
        all_predictions.truncate(limit);
        
        all_predictions
    }
    
    /// Predict based on command history
    fn predict_from_history(input: &str) -> Vec<Prediction> {
        let mut predictions = Vec::new();
        
        // Example hard-coded history predictions
        if input.is_empty() {
            // Common starting commands
            predictions.push(Prediction::new(
                "ls -la".to_string(), 
                PredictionType::FullCommand,
                PredictionSource::History,
                Confidence(0.8)
            ));
            
            predictions.push(Prediction::new(
                "git status".to_string(), 
                PredictionType::FullCommand,
                PredictionSource::History,
                Confidence(0.7)
            ));
        } else if input == "g" {
            predictions.push(Prediction::new(
                "git ".to_string(), 
                PredictionType::NextToken,
                PredictionSource::History,
                Confidence(0.9)
            ));
        } else if input.starts_with("git") {
            if input == "git " {
                predictions.push(Prediction::new(
                    "git status".to_string(), 
                    PredictionType::FullCommand,
                    PredictionSource::History,
                    Confidence(0.85)
                ));
                
                predictions.push(Prediction::new(
                    "git pull".to_string(), 
                    PredictionType::FullCommand,
                    PredictionSource::History,
                    Confidence(0.8)
                ));
            }
        }
        
        // In a real implementation, we would analyze user's command history
        
        predictions
    }
    
    /// Predict based on directory context
    fn predict_from_directory_context(input: &str, context: &Context) -> Vec<Prediction> {
        let mut predictions = Vec::new();
        
        // Example predictions based on directory contents
        if input.is_empty() || input == "." || input == "./" {
            match context.project_type {
                crate::suggestion::context::ProjectType::Rust => {
                    predictions.push(Prediction::new(
                        "cargo run".to_string(), 
                        PredictionType::FullCommand,
                        PredictionSource::DirectoryContext,
                        Confidence(0.85)
                    ));
                    
                    predictions.push(Prediction::new(
                        "cargo build".to_string(), 
                        PredictionType::FullCommand,
                        PredictionSource::DirectoryContext,
                        Confidence(0.8)
                    ));
                },
                crate::suggestion::context::ProjectType::Node => {
                    predictions.push(Prediction::new(
                        "npm run dev".to_string(), 
                        PredictionType::FullCommand,
                        PredictionSource::DirectoryContext,
                        Confidence(0.85)
                    ));
                    
                    predictions.push(Prediction::new(
                        "npm install".to_string(), 
                        PredictionType::FullCommand,
                        PredictionSource::DirectoryContext,
                        Confidence(0.8)
                    ));
                },
                _ => {}
            }
        }
        
        // In a real implementation, we would analyze files in the current directory
        
        predictions
    }
    
    /// Predict based on project context
    fn predict_from_project_context(input: &str, context: &Context) -> Vec<Prediction> {
        let mut predictions = Vec::new();
        
        // Example predictions based on project type
        if input.is_empty() {
            match context.project_type {
                crate::suggestion::context::ProjectType::Rust => {
                    predictions.push(Prediction::new(
                        "cargo test".to_string(), 
                        PredictionType::FullCommand,
                        PredictionSource::ProjectType,
                        Confidence(0.7)
                    ));
                },
                crate::suggestion::context::ProjectType::Python => {
                    predictions.push(Prediction::new(
                        "python -m venv .venv".to_string(), 
                        PredictionType::FullCommand,
                        PredictionSource::ProjectType,
                        Confidence(0.7)
                    ));
                },
                _ => {}
            }
        }
        
        predictions
    }
    
    /// Predict based on git context
    fn predict_from_git_context(input: &str, context: &Context) -> Vec<Prediction> {
        let mut predictions = Vec::new();
        
        if input.is_empty() {
            predictions.push(Prediction::new(
                "git status".to_string(), 
                PredictionType::FullCommand,
                PredictionSource::GitContext,
                Confidence(0.8)
            ));
        } else if input == "git " {
            predictions.push(Prediction::new(
                "git checkout ".to_string(), 
                PredictionType::NextToken,
                PredictionSource::GitContext,
                Confidence(0.7)
            ));
            
            predictions.push(Prediction::new(
                "git pull origin main".to_string(), 
                PredictionType::FullCommand,
                PredictionSource::GitContext,
                Confidence(0.75)
            ));
        }
        
        predictions
    }
    
    /// Record that a prediction was accepted
    pub fn record_prediction_accepted(&self, prediction: &Prediction) {
        self.user_learning.record_accepted_prediction(prediction);
    }
}
