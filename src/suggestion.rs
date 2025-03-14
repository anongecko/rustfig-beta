// Re-export from the suggestion module
pub mod engine;
pub mod command;
pub mod path;
pub mod context;

pub use self::engine::{Suggestion, SuggestionKind, SuggestionEngine};
pub use self::context::{Context, ContextDetector, ProjectType};
pub use self::command::CommandSuggester;
pub use self::path::PathSuggester;
