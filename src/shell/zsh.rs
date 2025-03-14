use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::env;

use super::ShellIntegration;

pub struct ZshIntegration {
    history_file: Option<PathBuf>,
}

impl ZshIntegration {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let history_file = if let Ok(home) = env::var("HOME") {
            let path = PathBuf::from(home).join(".zsh_history");
            if path.exists() {
                Some(path)
            } else {
                None
            }
        } else {
            None
        };
        
        Ok(Self {
            history_file,
        })
    }
    
    fn read_history_file(&self, limit: usize) -> Result<Vec<String>, Box<dyn Error>> {
        if let Some(history_file) = &self.history_file {
            let content = fs::read_to_string(history_file)?;
            let lines: Vec<String> = content
                .lines()
                .filter(|line| !line.trim().is_empty())
                .filter_map(|line| {
                    // Zsh history format is more complex, we need to parse it
                    if let Some(idx) = line.find(';') {
                        Some(line[idx+1..].to_string())
                    } else {
                        None
                    }
                })
                .take(limit)
                .collect();
            
            Ok(lines)
        } else {
            Ok(Vec::new())
        }
    }
}

impl ShellIntegration for ZshIntegration {
    fn get_current_command_line(&self) -> Result<String, Box<dyn Error>> {
        // In a real implementation, this would use FFI to access zle
        // For now, we'll just simulate
        
        // For testing purposes, let's return a dummy command
        Ok(String::from("echo 'Hello from zsh'"))
    }
    
    fn get_current_directory(&self) -> Result<String, Box<dyn Error>> {
        let output = Command::new("pwd")
            .output()?;
        
        if output.status.success() {
            let pwd = String::from_utf8(output.stdout)?;
            Ok(pwd.trim().to_string())
        } else {
            env::current_dir()
                .map(|p| p.to_string_lossy().to_string())
                .map_err(|e| e.into())
        }
    }
    
    fn get_history(&self, limit: usize) -> Result<Vec<String>, Box<dyn Error>> {
        self.read_history_file(limit)
    }
    
    fn apply_completion(&self, completion: &str) -> Result<(), Box<dyn Error>> {
        // In a real implementation, this would use FFI to modify zle buffer
        println!("Applied completion in zsh: {}", completion);
        Ok(())
    }
    
    fn get_shell_name(&self) -> &str {
        "zsh"
    }
}
