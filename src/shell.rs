// Re-export from the shell module
pub mod parser;
pub mod bash;
pub mod zsh;
pub mod fish;

pub use self::parser::CommandParser;
use self::bash::BashIntegration;
use self::zsh::ZshIntegration;
use self::fish::FishIntegration;

use std::env;
use std::error::Error;

/// Interface for shell integrations
pub trait ShellIntegration: Send + Sync {
    /// Get the current command line from the shell
    fn get_current_command_line(&self) -> Result<String, Box<dyn Error>>;
    
    /// Get the current working directory
    fn get_current_directory(&self) -> Result<String, Box<dyn Error>>;
    
    /// Get command history
    fn get_history(&self, limit: usize) -> Result<Vec<String>, Box<dyn Error>>;
    
    /// Apply a completion to the current command line
    fn apply_completion(&self, completion: &str) -> Result<(), Box<dyn Error>>;
    
    /// Get shell name
    fn get_shell_name(&self) -> &str;
}

/// Detect the current shell and initialize the appropriate integration
pub fn detect_and_initialize() -> Result<Box<dyn ShellIntegration>, Box<dyn Error>> {
    // Check for environment variables to determine shell
    if let Ok(shell) = env::var("SHELL") {
        let shell_path = shell.to_lowercase();
        
        if shell_path.contains("bash") {
            return Ok(Box::new(BashIntegration::new()?));
        } else if shell_path.contains("zsh") {
            return Ok(Box::new(ZshIntegration::new()?));
        } else if shell_path.contains("fish") {
            return Ok(Box::new(FishIntegration::new()?));
        }
    }
    
    // Default to bash if we can't detect
    Ok(Box::new(BashIntegration::new()?))
}
