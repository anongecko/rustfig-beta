use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

use super::keybindings::{Keybindings, KeyAction, KeyCombination};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// General configuration
    pub general: GeneralConfig,
    
    /// Terminal UI configuration
    pub ui: UiConfig,
    
    /// Suggestion engine configuration
    pub suggestions: SuggestionConfig,
    
    /// Prediction engine configuration
    pub prediction: PredictionConfig,
    
    /// AI integration configuration
    pub ai: AiConfig,
    
    /// Ollama integration configuration
    pub ollama: Option<OllamaConfig>,
    
    /// Shell-specific configuration
    pub shells: HashMap<String, ShellConfig>,
    
    /// Keybindings configuration
    pub keybindings: Option<Keybindings>,
    
    /// Plugin configuration
    pub plugins: Option<PluginConfig>,
    
    /// SSH configuration
    pub ssh: Option<SshConfig>,
    
    /// Telemetry configuration
    pub telemetry: Option<TelemetryConfig>,
    
    /// Performance tuning
    pub performance: Option<PerformanceConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            ui: UiConfig::default(),
            suggestions: SuggestionConfig::default(),
            prediction: PredictionConfig::default(),
            ai: AiConfig::default(),
            ollama: Some(OllamaConfig::default()),
            shells: HashMap::new(),
            keybindings: Some(Keybindings::default_bindings()),
            plugins: Some(PluginConfig::default()),
            ssh: Some(SshConfig::default()),
            telemetry: Some(TelemetryConfig::default()),
            performance: Some(PerformanceConfig::default()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeneralConfig {
    /// Debug mode
    pub debug: bool,
    
    /// Log file path
    pub log_file: Option<String>,
    
    /// Input timeout in milliseconds
    pub input_timeout_ms: u64,
    
    /// User data directory for storing history, patterns, etc.
    pub user_data_dir: PathBuf,
    
    /// Enable ghost text
    pub enable_ghost_text: Option<bool>,
    
    /// Maximum response latency for UI interactions (ms)
    pub max_ui_latency_ms: Option<u64>,
    
    /// Prefer local AI models when available
    pub prefer_local_models: Option<bool>,
    
    /// Automatic startup
    pub auto_start: Option<bool>,
    
    /// Show welcome message on first run
    pub show_welcome: Option<bool>,
    
    /// Enable verbose logging
    pub verbose_logging: Option<bool>,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            debug: false,
            log_file: None,
            input_timeout_ms: 10,
            user_data_dir: dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join(".rustfig"),
            enable_ghost_text: Some(true),
            max_ui_latency_ms: Some(5),
            prefer_local_models: Some(true),
            auto_start: Some(true),
            show_welcome: Some(true),
            verbose_logging: Some(false),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UiConfig {
    /// Dropdown width
    pub dropdown_width: u16,
    
    /// Maximum dropdown height
    pub dropdown_max_height: u16,
    
    /// Show icons in dropdowns
    pub show_icons: bool,
    
    /// Color theme
    pub theme: String,
    
    /// Ghost text color (RGB hex)
    pub ghost_text_color: Option<String>,
    
    /// Enable syntax highlighting
    pub syntax_highlighting: Option<bool>,
    
    /// Show command explanations in dropdown
    pub show_explanations: Option<bool>,
    
    /// Custom colors
    pub colors: Option<ColorConfig>,
    
    /// Animation speed (0=off, 1=slow, 10=fast)
    pub animation_speed: Option<u8>,
    
    /// Show dropdown automatically
    pub auto_show_dropdown: Option<bool>,
    
    /// Dropdown sort order
    pub dropdown_sort: Option<DropdownSortMode>,
    
    /// Dropdown appearance delay in ms
    pub dropdown_delay_ms: Option<u64>,
    
    /// Dropdown position (default/top/bottom)
    pub dropdown_position: Option<DropdownPosition>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DropdownSortMode {
    /// Sort by relevance score
    Relevance,
    /// Alphabetical sorting
    Alphabetical,
    /// Sort by most frequently used
    MostUsed,
    /// Sort by most recently used
    Recent,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DropdownPosition {
    /// Default position (below cursor)
    Default,
    /// Always at top of terminal
    Top,
    /// Always at bottom of terminal
    Bottom,
    /// Custom position
    Custom(u16, u16),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ColorConfig {
    /// Primary UI color
    pub primary: Option<String>,
    /// Secondary UI color
    pub secondary: Option<String>,
    /// Accent color
    pub accent: Option<String>,
    /// Background color
    pub background: Option<String>,
    /// Foreground text color
    pub foreground: Option<String>,
    /// Selected item background
    pub selected_bg: Option<String>,
    /// Selected item foreground
    pub selected_fg: Option<String>,
    /// Border color
    pub border: Option<String>,
    /// Error color
    pub error: Option<String>,
    /// Warning color
    pub warning: Option<String>,
    /// Success color
    pub success: Option<String>,
    /// Syntax highlighting colors
    pub syntax: Option<SyntaxColors>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SyntaxColors {
    /// Command color
    pub command: Option<String>,
    /// Argument color
    pub argument: Option<String>,
    /// Option/flag color
    pub option: Option<String>,
    /// Path color
    pub path: Option<String>,
    /// String color
    pub string: Option<String>,
    /// Variable color
    pub variable: Option<String>,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            dropdown_width: 50,
            dropdown_max_height: 10,
            show_icons: true,
            theme: "default".to_string(),
            ghost_text_color: Some("#666666".to_string()),
            syntax_highlighting: Some(true),
            show_explanations: Some(true),
            colors: None,
            animation_speed: Some(5),
            auto_show_dropdown: Some(false),
            dropdown_sort: Some(DropdownSortMode::Relevance),
            dropdown_delay_ms: Some(100),
            dropdown_position: Some(DropdownPosition::Default),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SuggestionConfig {
    /// Maximum number of suggestions to show
    pub max_suggestions: usize,
    
    /// Enable command suggestions
    pub enable_commands: bool,
    
    /// Enable path suggestions
    pub enable_paths: bool,
    
    /// Enable flag suggestions
    pub enable_flags: bool,
    
    /// Enable AI-powered suggestions
    pub enable_ai: bool,
    
    /// Directories to ignore for path completion
    pub ignored_dirs: Vec<String>,
    
    /// Cache lifetime in seconds
    pub cache_lifetime_secs: u64,
    
    /// Enable fuzzy matching
    pub fuzzy_matching: Option<bool>,
    
    /// Maximum number of history items to search
    pub max_history_items: Option<usize>,
    
    /// Advanced scoring options
    pub scoring: Option<ScoringConfig>,
    
    /// Enable snippet suggestions
    pub enable_snippets: Option<bool>,
    
    /// Enable variable expansion suggestions
    pub enable_variables: Option<bool>,
    
    /// Enable file content suggestions (grep, etc.)
    pub enable_file_content: Option<bool>,
    
    /// Enable completion while typing
    pub complete_while_typing: Option<bool>,
    
    /// Minimum prefix length for suggestions
    pub min_prefix_length: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScoringConfig {
    /// Recency weight (0.0-1.0)
    pub recency_weight: f32,
    /// Frequency weight (0.0-1.0)
    pub frequency_weight: f32,
    /// Context weight (0.0-1.0)
    pub context_weight: f32,
}

impl Default for SuggestionConfig {
    fn default() -> Self {
        Self {
            max_suggestions: 10,
            enable_commands: true,
            enable_paths: true,
            enable_flags: true,
            enable_ai: true,
            ignored_dirs: vec![
                ".git".to_string(),
                "node_modules".to_string(),
                "target".to_string(),
            ],
            cache_lifetime_secs: 60,
            fuzzy_matching: Some(true),
            max_history_items: Some(1000),
            scoring: Some(ScoringConfig {
                recency_weight: 0.7,
                frequency_weight: 0.8,
                context_weight: 0.9,
            }),
            enable_snippets: Some(true),
            enable_variables: Some(true),
            enable_file_content: Some(false),
            complete_while_typing: Some(true),
            min_prefix_length: Some(1),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PredictionConfig {
    /// Enable prediction system
    pub enable: bool,
    
    /// Maximum number of predictions to generate
    pub max_predictions: usize,
    
    /// Minimum confidence threshold for showing ghost text
    pub min_ghost_confidence: f32,
    
    /// Enable learning from user behavior
    pub enable_learning: bool,
    
    /// Maximum number of patterns to store in learning system
    pub max_learning_patterns: usize,
    
    /// Enable project-aware predictions
    pub enable_project_awareness: bool,
    
    /// Enable git-aware predictions
    pub enable_git_awareness: bool,
    
    /// Prediction cache size
    pub cache_size: usize,
    
    /// Prediction cache TTL in seconds
    pub cache_ttl_seconds: u64,
    
    /// Maximum latency for predictions to be considered (ms)
    pub max_prediction_latency_ms: Option<u64>,
    
    /// Enable context-based ranking of predictions
    pub enable_context_ranking: Option<bool>,
    
    /// Sources configuration
    pub sources: Option<SourcesConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SourcesConfig {
    /// Enable history-based predictions
    pub history: bool,
    /// Enable directory context predictions
    pub directory_context: bool,
    /// Enable project type predictions
    pub project_type: bool,
    /// Enable git context predictions
    pub git_context: bool,
    /// Enable command pattern predictions
    pub command_patterns: bool,
    /// Enable user pattern predictions
    pub user_patterns: bool,
}

impl Default for PredictionConfig {
    fn default() -> Self {
        Self {
            enable: true,
            max_predictions: 5,
            min_ghost_confidence: 0.4,
            enable_learning: true,
            max_learning_patterns: 10000,
            enable_project_awareness: true,
            enable_git_awareness: true,
            cache_size: 1000,
            cache_ttl_seconds: 300,
            max_prediction_latency_ms: Some(5),
            enable_context_ranking: Some(true),
            sources: Some(SourcesConfig {
                history: true,
                directory_context: true,
                project_type: true,
                git_context: true,
                command_patterns: true,
                user_patterns: true,
            }),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AiConfig {
    /// Enable AI integration
    pub enabled: bool,
    
    /// API endpoint
    pub api_endpoint: String,
    
    /// API key
    pub api_key: Option<String>,
    
    /// Timeout in seconds
    pub timeout_secs: u64,
    
    /// Cache AI responses
    pub enable_cache: bool,
    
    /// Maximum cache entries
    pub max_cache_entries: usize,
    
    /// Model to use (for OpenAI-compatible APIs)
    pub model: Option<String>,
    
    /// Temperature for API calls (0.0-1.0)
    pub temperature: Option<f32>,
    
    /// Max tokens for API responses
    pub max_tokens: Option<u32>,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            api_endpoint: "http://localhost:8000/v1".to_string(),
            api_key: None,
            timeout_secs: 5,
            enable_cache: true,
            max_cache_entries: 1000,
            model: Some("gpt-3.5-turbo".to_string()),
            temperature: Some(0.2),
            max_tokens: Some(100),
        }
    }
}

/// Ollama local model configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OllamaConfig {
    /// Enable Ollama integration
    pub enabled: bool,
    
    /// API URL for Ollama
    pub api_url: String,
    
    /// Model to use
    pub model: String,
    
    /// Timeout in seconds
    pub timeout_secs: u64,
    
    /// Cache responses
    pub enable_cache: bool,
    
    /// Maximum cache entries
    pub max_cache_entries: usize,
    
    /// Advanced parameters
    pub parameters: Option<OllamaParameters>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OllamaParameters {
    /// Temperature (0.0-1.0)
    pub temperature: f32,
    /// Top-p sampling
    pub top_p: f32,
    /// Maximum tokens to generate
    pub max_tokens: u32,
    /// System prompt for context
    pub system_prompt: String,
}

impl Default for OllamaConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            api_url: "http://localhost:11434".to_string(),
            model: "codellama:7b-instruct".to_string(),
            timeout_secs: 3,
            enable_cache: true,
            max_cache_entries: 500,
            parameters: Some(OllamaParameters {
                temperature: 0.1,
                top_p: 0.9,
                max_tokens: 100,
                system_prompt: "You are a helpful terminal assistant that provides accurate, concise shell command suggestions.".to_string(),
            }),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShellConfig {
    /// Shell command to execute for shell-specific operations
    pub command: String,
    
    /// Shell initialization file
    pub init_file: Option<String>,
    
    /// History file path
    pub history_file: Option<String>,
    
    /// Enable custom keybindings for this shell
    pub enable_keybindings: Option<bool>,
    
    /// Shell-specific aliases to load
    pub load_aliases: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PluginConfig {
    /// Enable the plugin system
    pub enabled: bool,
    
    /// Directory for plugins
    pub plugin_dir: PathBuf,
    
    /// Enabled plugins
    pub enabled_plugins: Vec<String>,
    
    /// Plugin-specific configurations
    pub plugin_configs: HashMap<String, serde_yaml::Value>,
}

impl Default for PluginConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            plugin_dir: dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join(".rustfig")
                .join("plugins"),
            enabled_plugins: Vec::new(),
            plugin_configs: HashMap::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SshConfig {
    /// Enable SSH-specific optimizations
    pub enable_optimizations: bool,
    
    /// Maximum bandwidth usage (KB/s)
    pub max_bandwidth_kb: Option<u32>,
    
    /// Enable command caching for SSH sessions
    pub enable_command_caching: Option<bool>,
    
    /// Disable expensive features in SSH sessions
    pub disable_expensive_features: Option<bool>,
    
    /// Reduce animation in SSH sessions
    pub reduce_animations: Option<bool>,
}

impl Default for SshConfig {
    fn default() -> Self {
        Self {
            enable_optimizations: true,
            max_bandwidth_kb: Some(50),
            enable_command_caching: Some(true),
            disable_expensive_features: Some(true),
            reduce_animations: Some(true),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TelemetryConfig {
    /// Enable telemetry
    pub enabled: bool,
    
    /// Telemetry data directory
    pub data_dir: Option<PathBuf>,
    
    /// Telemetry upload URL
    pub upload_url: String,
    
    /// Feedback submission URL
    pub feedback_url: String,
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            data_dir: None,
            upload_url: "https://api.rustfig.dev/telemetry".to_string(),
            feedback_url: "https://api.rustfig.dev/feedback".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PerformanceConfig {
    /// Number of worker threads (0 = auto-detect)
    pub worker_threads: usize,
    
    /// Maximum memory usage in MB (0 = unlimited)
    pub max_memory_mb: usize,
    
    /// Enable background cache warming
    pub enable_cache_warming: bool,
    
    /// Enable parallel suggestion generation
    pub parallel_suggestions: bool,
    
    /// I/O optimizations
    pub optimizations: Option<OptimizationConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OptimizationConfig {
    /// Use memory mapped files
    pub mmap_files: bool,
    
    /// Buffer size for file operations
    pub file_buffer_size: usize,
    
    /// Compress cached data
    pub compress_cache: bool,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            worker_threads: 0, // Auto-detect
            max_memory_mb: 0,  // Unlimited
            enable_cache_warming: true,
            parallel_suggestions: true,
            optimizations: Some(OptimizationConfig {
                mmap_files: true,
                file_buffer_size: 8192,
                compress_cache: true,
            }),
        }
    }
}
