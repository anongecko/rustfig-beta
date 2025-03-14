use std::path::PathBuf;
use std::process::Command;
use crate::shell::parser::ParsedCommand;
use crate::suggestion::context::{Context, ProjectType};

/// Analyzes current terminal context for more accurate predictions
pub struct ContextAnalyzer;

impl ContextAnalyzer {
    pub fn new() -> Self {
        Self
    }
    
    /// Analyze the current context to enable smarter predictions
    pub async fn analyze(&self, input: &str, parsed: &ParsedCommand<'_>) -> Context {
        // Get current directory
        let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        
        // Determine if we're in a git repository
        let in_git_repo = self.is_git_repository(&current_dir);
        
        // Determine if we're in a docker context
        let in_docker_context = self.is_docker_context(&current_dir);
        
        // Detect project type
        let project_type = self.detect_project_type(&current_dir);
        
        // Create context
        Context {
            current_dir,
            in_git_repo,
            in_docker_context,
            current_command: parsed.command.to_string(),
            project_type,
        }
    }
    
    /// Check if current directory is a git repository
    fn is_git_repository(&self, dir: &PathBuf) -> bool {
        // Fast check: see if .git directory exists
        if dir.join(".git").exists() {
            return true;
        }
        
        // Slower but more reliable check: try git command
        match Command::new("git")
            .args(["rev-parse", "--is-inside-work-tree"])
            .current_dir(dir)
            .output() 
        {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }
    
    /// Check if current directory is a docker context
    fn is_docker_context(&self, dir: &PathBuf) -> bool {
        dir.join("Dockerfile").exists() || dir.join("docker-compose.yml").exists()
    }
    
    /// Detect project type based on files in directory
    fn detect_project_type(&self, dir: &PathBuf) -> ProjectType {
        // Check for Rust project
        if dir.join("Cargo.toml").exists() {
            return ProjectType::Rust;
        }
        
        // Check for Node project
        if dir.join("package.json").exists() {
            return ProjectType::Node;
        }
        
        // Check for Python project
        if dir.join("requirements.txt").exists() || dir.join("setup.py").exists() {
            return ProjectType::Python;
        }
        
        // Check for Go project
        if dir.join("go.mod").exists() {
            return ProjectType::Go;
        }
        
        ProjectType::Unknown
    }
    
    /// Get git branches (async to avoid blocking)
    pub async fn get_git_branches(&self, dir: &PathBuf) -> Vec<String> {
        // Spawn a tokio task to run the command
        let dir_clone = dir.clone();
        let branches = tokio::task::spawn_blocking(move || {
            let output = Command::new("git")
                .args(["branch"])
                .current_dir(dir_clone)
                .output();
            
            match output {
                Ok(output) if output.status.success() => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    stdout.lines()
                        .filter_map(|line| {
                            let trimmed = line.trim();
                            if trimmed.starts_with('*') {
                                Some(trimmed[2..].to_string())
                            } else {
                                Some(trimmed.to_string())
                            }
                        })
                        .collect()
                },
                _ => Vec::new(),
            }
        }).await;
        
        branches.unwrap_or_default()
    }
    
    /// Get information about current git status
    pub async fn get_git_status(&self, dir: &PathBuf) -> Option<GitStatus> {
        let dir_clone = dir.clone();
        let status = tokio::task::spawn_blocking(move || {
            let output = Command::new("git")
                .args(["status", "--porcelain"])
                .current_dir(dir_clone)
                .output();
            
            match output {
                Ok(output) if output.status.success() => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let mut modified = false;
                    let mut untracked = false;
                    
                    for line in stdout.lines() {
                        if line.starts_with("M") || line.starts_with(" M") {
                            modified = true;
                        } else if line.starts_with("??") {
                            untracked = true;
                        }
                    }
                    
                    Some(GitStatus {
                        has_modified: modified,
                        has_untracked: untracked,
                    })
                },
                _ => None,
            }
        }).await;
        
        status.unwrap_or(None)
    }
}

/// Git repository status information
#[derive(Debug, Clone)]
pub struct GitStatus {
    pub has_modified: bool,
    pub has_untracked: bool,
}
