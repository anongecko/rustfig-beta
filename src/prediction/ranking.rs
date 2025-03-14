use super::models::{Prediction, PredictionSource, PredictionType};

/// Ranks predictions by relevance and confidence
pub struct PredictionRanker;

impl PredictionRanker {
    pub fn new() -> Self {
        Self
    }
    
    /// Rank predictions and sort them by score
    pub fn rank(&self, predictions: &mut Vec<Prediction>) {
        // Apply source-based scoring
        for prediction in predictions.iter_mut() {
            // Base score from confidence
            let mut score = prediction.confidence.0;
            
            // Adjust based on source
            match prediction.source {
                PredictionSource::History => score *= 1.2, // Boost history-based
                PredictionSource::UserPatterns => score *= 1.3, // Boost user patterns
                PredictionSource::GitContext => score *= 1.1, // Boost git context
                _ => {}
            }
            
            // Adjust based on prediction type
            match prediction.prediction_type {
                PredictionType::FullCommand => score *= 1.1, // Boost full commands
                _ => {}
            }
            
            // Adjust based on usage count
            if prediction.usage_count > 0 {
                let usage_boost = (prediction.usage_count as f32).min(5.0) / 5.0 * 0.2;
                score += usage_boost;
            }
            
            // Normalize score
            prediction.confidence.0 = score.min(1.0);
        }
        
        // Sort by confidence score (descending)
        predictions.sort_by(|a, b| {
            b.confidence.0.partial_cmp(&a.confidence.0)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        
        // Remove duplicates, keeping the highest scored one
        Self::dedup_predictions(predictions);
    }
    
    /// Remove duplicate predictions keeping the highest scored one
    fn dedup_predictions(predictions: &mut Vec<Prediction>) {
        let mut seen = std::collections::HashSet::new();
        let mut i = 0;
        
        while i < predictions.len() {
            let text = &predictions[i].text;
            
            if seen.contains(text) {
                predictions.remove(i);
            } else {
                seen.insert(text.clone());
                i += 1;
            }
        }
    }
    
    /// Filter predictions that are appropriate for ghost text display
    pub fn filter_for_ghost(&self, predictions: &[Prediction]) -> Option<&Prediction> {
        predictions.iter()
            .find(|p| p.confidence.is_high_enough_for_ghost())
    }
}
