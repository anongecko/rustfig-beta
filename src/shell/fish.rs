use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::env;

use super::ShellIntegration;

pub struct FishIntegration {
    history_file: Option<PathBuf>,
}

impl FishIntegration {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let history_file = if let Ok(home) = env::var("HOME") {
            let path = PathBuf::from(home).join(".local/share/fish/fish_history");
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
            // Fish history is stored in a more complex format
            // This is a simplified version
            let content = fs::read_to_string(history_file)?;
            let mut lines = Vec::new();
            
            for line in content.lines() {
                if line.contains("cmd: ") {
                    if let Some(cmd_start) = line.find("cmd: ") {
                        let cmd = &line[cmd_start + 5..];
                        lines.push(cmd.trim().to_string());
                        
                        if lines.len() >= limit {
                            break;
                        }
                    }
                }
            }
            
            Ok(lines)
        } else {
            Ok(Vec::new())
        }
    }
}

impl ShellIntegration for FishIntegration {
    fn get_current_command_line(&self) -> Result<String, Box<dyn Error>> {
        // In a real implementation, this would use a named pipe or other IPC
        // For now, we'll just simulate
        
        // For testing purposes, let's return a dummy command
        Ok(String::from("echo 'Hello from fish'"))
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
        // In a real implementation, this would use a named pipe or other IPC
        println!("Applied completion in fish: {}", completion);
        Ok(())
    }
    
    fn get_shell_name(&self) -> &str {
        "fish"
    }
}
