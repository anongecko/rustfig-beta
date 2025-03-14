// Re-export from the config module
pub mod loader;
pub mod schema;

pub use self::schema::Config;
pub use self::loader::load_config;

// This allows importing these structs directly from config
pub use self::schema::{
    GeneralConfig,
    UiConfig,
    SuggestionConfig,
    AiConfig,
    ShellConfig,
};
