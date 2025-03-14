mod usage;
mod feedback;

pub use usage::UsageTracker;
pub use feedback::FeedbackCollector;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use once_cell::sync::Lazy;

// Global telemetry enabled flag
static TELEMETRY_ENABLED: Lazy<Arc<AtomicBool>> = Lazy::new(|| {
    Arc::new(AtomicBool::new(false))
});

/// Initialize telemetry system
pub fn init(config: &crate::config::Config) {
    let enabled = config.telemetry.as_ref()
        .map(|t| t.enabled)
        .unwrap_or(false);
    
    set_telemetry_enabled(enabled);
}

/// Check if telemetry is enabled
pub fn is_telemetry_enabled() -> bool {
    TELEMETRY_ENABLED.load(Ordering::Relaxed)
}

/// Set telemetry enabled state
pub fn set_telemetry_enabled(enabled: bool) {
    TELEMETRY_ENABLED.store(enabled, Ordering::Relaxed);
}

/// Create a new usage tracker instance
pub fn create_usage_tracker(config: &crate::config::Config) -> UsageTracker {
    let telemetry_config = config.telemetry.clone().unwrap_or_default();
    UsageTracker::new(telemetry_config)
}

/// Create a new feedback collector instance
pub fn create_feedback_collector(config: &crate::config::Config) -> FeedbackCollector {
    let telemetry_config = config.telemetry.clone().unwrap_or_default();
    FeedbackCollector::new(telemetry_config)
}
