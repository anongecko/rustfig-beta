use std::error::Error;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::fs;
use std::time::Instant;

use crate::ai::{AiProvider, AiProviderFactory};
use crate::config;
use crate::utils::ssh::is_ssh_session;

/// Run initial setup
pub fn cmd_setup(minimal: bool, verbose: bool) -> Result<(), Box<dyn Error>> {
    println!("Setting up RustFig...");
    
    // Initialize configuration files
    config::init::initialize_config_files()?;
    
    // Detect shell
    let shell = detect_current_shell()?;
    
    if verbose {
        println!("Detected shell: {}", shell);
    }
    
    // Install shell integration
    cmd_install(Some(shell.as_str()), false)?;
    
    if !minimal {
        // Start service
        cmd_service_start(verbose)?;
    }
    
    println!("RustFig setup complete!");
    println!("Restart your terminal or run 'source ~/.{}rc' to activate.", shell);
    
    Ok(())
}

/// Generate shell integration code
pub fn cmd_init(shell: &str, minimal: bool) -> Result<String, Box<dyn Error>> {
    let integration_code = match shell {
        "bash" => {
            if minimal {
                include_str!("../../resources/shell/bash/minimal.sh").to_string()
            } else {
                include_str!("../../resources/shell/bash/full.sh").to_string()
            }
        },
        "zsh" => {
            if minimal {
                include_str!("../../resources/shell/zsh/minimal.zsh").to_string()
            } else {
                include_str!("../../resources/shell/zsh/full.zsh").to_string()
            }
        },
        "fish" => {
            if minimal {
                include_str!("../../resources/shell/fish/minimal.fish").to_string()
            } else {
                include_str!("../../resources/shell/fish/full.fish").to_string()
            }
        },
        _ => return Err(format!("Unsupported shell: {}", shell).into()),
    };
    
    Ok(integration_code)
}

/// Install shell integration
pub fn cmd_install(shell_override: Option<&str>, force: bool) -> Result<(), Box<dyn Error>> {
    // Determine shell
    let shell = if let Some(shell) = shell_override {
        shell.to_string()
    } else {
        detect_current_shell()?
    };
    
    // Generate integration code
    let integration_code = cmd_init(&shell, false)?;
    
    // Determine the appropriate RC file
    let rc_file = match shell.as_str() {
        "bash" => {
            if cfg!(target_os = "macos") {
                dirs::home_dir().unwrap().join(".bash_profile")
            } else {
                dirs::home_dir().unwrap().join(".bashrc")
            }
        },
        "zsh" => dirs::home_dir().unwrap().join(".zshrc"),
        "fish" => dirs::home_dir().unwrap().join(".config/fish/config.fish"),
        _ => return Err(format!("Unsupported shell: {}", shell).into()),
    };
    
    // Check if RC file exists
    if !rc_file.exists() && !force {
        return Err(format!("Shell RC file not found: {}. Use --force to create it.", rc_file.display()).into());
    }
    
    // Read existing content
    let content = if rc_file.exists() {
        fs::read_to_string(&rc_file)?
    } else {
        String::new()
    };
    
    // Check if already installed
    if content.contains("# RustFig integration START") && !force {
        return Err("RustFig is already installed. Use --force to reinstall.".into());
    }
    
    // Add integration code or replace existing integration
    let new_content = if content.contains("# RustFig integration START") {
        let start_marker = "# RustFig integration START";
        let end_marker = "# RustFig integration END";
        
        let start_pos = content.find(start_marker).unwrap();
        let end_pos = content.find(end_marker).unwrap() + end_marker.len();
        
        format!(
            "{}{}{}",
            &content[..start_pos],
            integration_code,
            &content[end_pos..]
        )
    } else {
        // Just append
        format!("{}\n\n{}", content, integration_code)
    };
    
    // Ensure parent directory exists
    if let Some(parent) = rc_file.parent() {
        fs::create_dir_all(parent)?;
    }
    
    // Write back
    fs::write(&rc_file, new_content)?;
    
    println!("RustFig shell integration installed to {}", rc_file.display());
    println!("Restart your terminal or run 'source {}' to activate.", rc_file.display());
    
    Ok(())
}

/// Uninstall shell integration
pub fn cmd_uninstall(shell_override: Option<&str>) -> Result<(), Box<dyn Error>> {
    // Determine shell
    let shell = if let Some(shell) = shell_override {
        shell.to_string()
    } else {
        detect_current_shell()?
    };
    
    // Determine the appropriate RC file
    let rc_file = match shell.as_str() {
        "bash" => {
            if cfg!(target_os = "macos") {
                dirs::home_dir().unwrap().join(".bash_profile")
            } else {
                dirs::home_dir().unwrap().join(".bashrc")
            }
        },
        "zsh" => dirs::home_dir().unwrap().join(".zshrc"),
        "fish" => dirs::home_dir().unwrap().join(".config/fish/config.fish"),
        _ => return Err(format!("Unsupported shell: {}", shell).into()),
    };
    
    // Check if RC file exists
    if !rc_file.exists() {
        return Err(format!("Shell RC file not found: {}.", rc_file.display()).into());
    }
    
    // Read existing content
    let content = fs::read_to_string(&rc_file)?;
    
    // Check if installed
    if !content.contains("# RustFig integration START") {
        return Err("RustFig is not installed in this shell.".into());
    }
    
    // Remove integration code
    let new_content = if content.contains("# RustFig integration START") {
        let start_marker = "# RustFig integration START";
        let end_marker = "# RustFig integration END";
        
        let start_pos = content.find(start_marker).unwrap();
        let end_pos = content.find(end_marker).unwrap() + end_marker.len();
        
        format!(
            "{}{}",
            &content[..start_pos],
            &content[end_pos..]
        )
    } else {
        content
    };
    
    // Write back
    fs::write(&rc_file, new_content)?;
    
    // Stop service
    let _ = cmd_service_stop(false);
    
    println!("RustFig shell integration removed from {}", rc_file.display());
    println!("Restart your terminal or run 'source {}' to apply changes.", rc_file.display());
    
    Ok(())
}

/// Run system checks
pub fn cmd_doctor(fix: bool, verbose: bool) -> Result<(), Box<dyn Error>> {
    println!("Running RustFig diagnostics...");
    
    let mut issues_found = false;
    
    // Check configuration files
    let config_dir = config::loader::get_config_dir()?;
    let config_file = config_dir.join("config.yaml");
    
    println!("\nChecking configuration:");
    println!("  Config directory: {}", config_dir.display());
    
    if !config_file.exists() {
        println!("  [✗] Main configuration file missing");
        issues_found = true;
        
        if fix {
            println!("    Generating default configuration...");
            config::init::generate_default_config(&config_file)?;
            println!("    Created {}", config_file.display());
        }
    } else {
        println!("  [✓] Configuration file: {}", config_file.display());
        
        // Validate config
        match config::validator::validate_config(&config_file) {
            Ok(_) => println!("  [✓] Configuration is valid"),
            Err(e) => {
                println!("  [✗] Configuration validation failed: {}", e);
                issues_found = true;
                
                if fix {
                    println!("    Creating backup and generating new configuration...");
                    let backup_path = config_file.with_extension("yaml.bak");
                    fs::copy(&config_file, &backup_path)?;
                    println!("    Backup created at {}", backup_path.display());
                    
                    config::init::generate_default_config(&config_file)?;
                    println!("    Created fresh configuration at {}", config_file.display());
                }
            }
        }
    }
    
    // Check shell integration
    println!("\nChecking shell integration:");
    let shell = detect_current_shell()?;
    println!("  Detected shell: {}", shell);
    
    let rc_file = match shell.as_str() {
        "bash" => {
            if cfg!(target_os = "macos") {
                dirs::home_dir().unwrap().join(".bash_profile")
            } else {
                dirs::home_dir().unwrap().join(".bashrc")
            }
        },
        "zsh" => dirs::home_dir().unwrap().join(".zshrc"),
        "fish" => dirs::home_dir().unwrap().join(".config/fish/config.fish"),
        _ => {
            println!("  [✗] Unsupported shell: {}", shell);
            issues_found = true;
            return Ok(());
        }
    };
    
    if !rc_file.exists() {
        println!("  [✗] Shell RC file not found: {}", rc_file.display());
        issues_found = true;
    } else {
        let content = fs::read_to_string(&rc_file)?;
        if content.contains("# RustFig integration START") {
            println!("  [✓] Shell integration installed");
        } else {
            println!("  [✗] Shell integration not installed");
            issues_found = true;
            
            if fix {
                println!("    Installing shell integration...");
                cmd_install(Some(&shell), true)?;
            }
        }
    }
    
    // Check service
    println!("\nChecking RustFig service:");
    match cmd_service_status_internal() {
        Ok(running) => {
            if running {
                println!("  [✓] Service is running");
            } else {
                println!("  [✗] Service is not running");
                issues_found = true;
                
                if fix {
                    println!("    Starting service...");
                    cmd_service_start(false)?;
                }
            }
        },
        Err(e) => {
            println!("  [✗] Failed to check service: {}", e);
            issues_found = true;
        }
    }
    
    // Check for AI capabilities
    println!("\nChecking AI capabilities:");
    let config = config::loader::load_config()?;
    
    if let Some(ai_provider) = AiProviderFactory::create_provider(&config).await {
        if ai_provider.is_available().await {
            println!("  [✓] AI provider '{}' is available", ai_provider.name());
        } else {
            println!("  [✗] AI provider '{}' is not responding", ai_provider.name());
            issues_found = true;
        }
    } else {
        println!("  [✗] No AI provider configured");
        if verbose {
            println!("    Configure either 'ai' or 'ollama' section in config.yaml");
        }
        issues_found = true;
    }
    
    // Check if running in SSH session
    if is_ssh_session() {
        println!("\nRunning in SSH session:");
        if config.ssh.as_ref().map_or(false, |s| s.enable_optimizations) {
            println!("  [✓] SSH optimizations enabled");
        } else {
            println!("  [✗] SSH optimizations disabled");
            
            if fix {
                println!("    Enabling SSH optimizations...");
                cmd_config_set("ssh.enable_optimizations", "true")?;
            }
        }
    }
    
    // System information
    println!("\nSystem information:");
    println!("  OS: {}", std::env::consts::OS);
    println!("  Architecture: {}", std::env::consts::ARCH);
    println!("  RustFig version: {}", env!("CARGO_PKG_VERSION"));
    
    if verbose {
        // Additional checks for verbose mode
        println!("\nAdditional information:");
        
        // Check themes
        let themes_dir = config_dir.join("themes");
        if themes_dir.exists() {
            let theme_count = fs::read_dir(&themes_dir)
                .map(|entries| entries.count())
                .unwrap_or(0);
            println!("  Themes directory: {} ({} themes)", themes_dir.display(), theme_count);
        } else {
            println!("  [✗] Themes directory missing: {}", themes_dir.display());
        }
        
        // Check permissions
        let home_dir = dirs::home_dir().unwrap();
        println!("  Home directory: {}", home_dir.display());
        
        // Check if we can write to the necessary directories
        let temp_file = config_dir.join(".write_test");
        match fs::File::create(&temp_file) {
            Ok(_) => {
                println!("  [✓] Write access to config directory");
                let _ = fs::remove_file(temp_file);
            },
            Err(e) => {
                println!("  [✗] Cannot write to config directory: {}", e);
                issues_found = true;
            }
        }
    }
    
    // Summary
    println!("\nDiagnostics summary:");
    if issues_found {
        println!("  [✗] Issues were found. Some features may not work correctly.");
        if !fix {
            println!("  Run 'rustfig doctor --fix' to attempt automatic fixes.");
        }
    } else {
        println!("  [✓] All checks passed! RustFig is properly configured.");
    }
    
    Ok(())
}

/// Service: Start
pub fn cmd_service_start(verbose: bool) -> Result<(), Box<dyn Error>> {
    // Check if already running
    if cmd_service_status_internal()? {
        println!("RustFig service is already running.");
        return Ok(());
    }
    
    if verbose {
        println!("Starting RustFig service...");
    }
    
    // Start the service in the background
    let executable = std::env::current_exe()?;
    
    let mut command = if cfg!(target_os = "windows") {
        let mut cmd = Command::new("cmd");
        cmd.args(["/C", "start", "/B"]);
        cmd.arg(executable);
        cmd.arg("service");
        cmd.arg("run");
        cmd
    } else {
        let mut cmd = Command::new(executable);
        cmd.arg("service");
        cmd.arg("run");
        cmd.arg("--daemon");
        cmd
    };
    
    command.stdin(Stdio::null());
    command.stdout(Stdio::null());
    command.stderr(Stdio::null());
    
    command.spawn()?;
    
    // Wait for service to start
    let mut attempts = 0;
    while attempts < 10 {
        if cmd_service_status_internal()? {
            if verbose {
                println!("RustFig service started successfully.");
            } else {
                println!("RustFig service started.");
            }
            return Ok(());
        }
        
        std::thread::sleep(std::time::Duration::from_millis(100));
        attempts += 1;
    }
    
    if verbose {
        println!("Warning: Service may not have started properly. Check logs for details.");
    }
    
    Ok(())
}

/// Service: Stop
pub fn cmd_service_stop(force: bool) -> Result<(), Box<dyn Error>> {
    // Check if running
    if !cmd_service_status_internal()? && !force {
        println!("RustFig service is not running.");
        return Ok(());
    }
    
    println!("Stopping RustFig service...");
    
    // Send stop signal
    let executable = std::env::current_exe()?;
    let mut command = Command::new(executable);
    command.arg("service");
    command.arg("signal");
    command.arg("stop");
    
    let output = command.output()?;
    
    if !output.status.success() && !force {
        let error = String::from_utf8_lossy(&output.stderr);
        println!("Error stopping service: {}", error);
        
        if force {
            println!("Forcefully terminating service...");
            
            // Find and kill the process
            if cfg!(target_os = "windows") {
                Command::new("taskkill")
                    .args(["/F", "/IM", "rustfig.exe"])
                    .output()?;
            } else {
                Command::new("pkill")
                    .args(["-f", "rustfig service run"])
                    .output()?;
            }
        } else {
            return Err("Failed to stop service. Use --force to forcefully terminate.".into());
        }
    }
    
    println!("RustFig service stopped.");
    Ok(())
}

/// Service: Status (internal implementation)
fn cmd_service_status_internal() -> Result<bool, Box<dyn Error>> {
    // Check if the service is running
    let executable = std::env::current_exe()?;
    let mut command = Command::new(executable);
    command.arg("service");
    command.arg("ping");
    
    let output = command.output()?;
    
    Ok(output.status.success())
}

/// Service: Status
pub fn cmd_service_status(verbose: bool) -> Result<(), Box<dyn Error>> {
    let running = cmd_service_status_internal()?;
    
    if running {
        println!("RustFig service: RUNNING");
        
        if verbose {
            // Get service details
            let executable = std::env::current_exe()?;
            let mut command = Command::new(executable);
            command.arg("service");
            command.arg("info");
            
            let output = command.output()?;
            
            if output.status.success() {
                let info = String::from_utf8_lossy(&output.stdout);
                println!("\nService details:");
                println!("{}", info);
            }
        }
    } else {
        println!("RustFig service: NOT RUNNING");
    }
    
    Ok(())
}

/// Config: get a specific value
pub fn cmd_config_get(key: &str, format: &str) -> Result<(), Box<dyn Error>> {
    let config = config::loader::load_config()?;
    
    // Parse the key path (e.g., "ui.theme")
    let parts: Vec<&str> = key.split('.').collect();
    
    // Navigate the configuration structure
    let mut current_value = serde_yaml::to_value(&config)?;
    
    for part in parts {
        // Check if the current value is a mapping
        if !current_value.is_mapping() {
            return Err(format!("Invalid configuration path: {}", key).into());
        }
        
        // Try to get the next part
        match current_value.get(part) {
            Some(value) => {
                current_value = value.clone();
            }
            None => {
                return Err(format!("Configuration key not found: {}", key).into());
            }
        }
    }
    
    // Output according to requested format
    match format {
        "yaml" => {
            let yaml = serde_yaml::to_string(&current_value)?;
            println!("{}", yaml);
        }
        "json" => {
            let json = serde_json::to_string_pretty(&current_value)?;
            println!("{}", json);
        }
        "text" | _ => {
            if current_value.is_mapping() || current_value.is_sequence() {
                let yaml = serde_yaml::to_string(&current_value)?;
                println!("{}", yaml);
            } else {
                println!("{}", current_value);
            }
        }
    }
    
    Ok(())
}

/// Config: set a specific value
pub fn cmd_config_set(key: &str, value: &str) -> Result<(), Box<dyn Error>> {
    let config_dir = config::loader::get_config_dir()?;
    let config_file = config_dir.join("config.yaml");
    
    // Load the existing config as YAML Value
    let yaml_str = fs::read_to_string(&config_file)?;
    let mut yaml_value: serde_yaml::Value = serde_yaml::from_str(&yaml_str)?;
    
    // Parse the key path (e.g., "ui.theme")
    let parts: Vec<&str> = key.split('.').collect();
    
    // Convert the value string to YAML Value
    let new_value: serde_yaml::Value = match value {
        "true" => serde_yaml::Value::Bool(true),
        "false" => serde_yaml::Value::Bool(false),
        "null" => serde_yaml::Value::Null,
        _ => {
            // Try to parse as integer
            if let Ok(int_val) = value.parse::<i64>() {
                serde_yaml::Value::Number(serde_yaml::Number::from(int_val))
            }
            // Try to parse as float
            else if let Ok(float_val) = value.parse::<f64>() {
                serde_yaml::to_value(float_val)?
            }
            // Default to string
            else {
                serde_yaml::Value::String(value.to_string())
            }
        }
    };
    
    // Navigate and update the configuration structure
    let mut current_value = &mut yaml_value;
    
    for (i, part) in parts.iter().enumerate() {
        if i == parts.len() - 1 {
            // Last part, set the value
            current_value[part] = new_value.clone();
        } else {
            // Create the path if it doesn't exist
            if !current_value.get(part).is_some() {
                current_value[part] = serde_yaml::Value::Mapping(serde_yaml::Mapping::new());
            }
            current_value = &mut current_value[part];
        }
    }
    
    // Write the updated config back
    let yaml_str = serde_yaml::to_string(&yaml_value)?;
    fs::write(&config_file, yaml_str)?;
    
    println!("Configuration updated: {} = {}", key, value);
    
    Ok(())
}

/// Ask an AI question
pub async fn cmd_ask(question: &str, model: Option<&str>, markdown: bool) -> Result<(), Box<dyn Error>> {
    let config = config::loader::load_config()?;
    
    // Create AI provider
    let ai_provider = match AiProviderFactory::create_provider(&config).await {
        Some(provider) => provider,
        None => return Err("No AI provider configured. Check your configuration.".into()),
    };
    
    println!("Asking AI: {}", question);
    println!();
    
    // Measure response time
    let start = Instant::now();
    
    // Query AI
    let response = ai_provider.query(question).await?;
    
    let duration = start.elapsed();
    
    // Output result
    if markdown {
        println!("{}", response);
    } else {
        // Simple markdown stripping for terminal output
        let response = response.replace("```", "");
        println!("{}", response);
    }
    
    println!("\nResponse time: {:.2?}", duration);
    
    Ok(())
}

/// Start interactive chat session
pub async fn cmd_chat(model: Option<&str>, conversation_id: Option<&str>) -> Result<(), Box<dyn Error>> {
    let config = config::loader::load_config()?;
    
    // Create AI provider
    let ai_provider = match AiProviderFactory::create_provider(&config).await {
        Some(provider) => provider,
        None => return Err("No AI provider configured. Check your configuration.".into()),
    };
    
    // Create conversation manager
    let config_dir = config::loader::get_config_dir()?;
    let conversation_dir = config_dir.join("conversations");
    
    let mut conversation_manager = crate::ai::conversation::ConversationManager::new(&conversation_dir)?;
    
    // Handle conversation ID if provided
    if let Some(id) = conversation_id {
        conversation_manager.set_active_conversation(id)?;
    }
    
    // Run interactive session
    conversation_manager.run_interactive_session(ai_provider.as_ref()).await?;
    
    Ok(())
}

/// Detect current shell
fn detect_current_shell() -> Result<String, Box<dyn Error>> {
    // Try to detect from SHELL environment variable
    if let Ok(shell) = std::env::var("SHELL") {
        let shell_path = PathBuf::from(shell);
        if let Some(file_name) = shell_path.file_name() {
            let shell_name = file_name.to_string_lossy().to_string();
            
            // Match known shells
            if shell_name == "bash" || shell_name == "zsh" || shell_name == "fish" {
                return Ok(shell_name);
            }
        }
    }
    
    // Try to detect from process name
    if let Ok(output) = Command::new("ps").args(["-p", &std::process::id().to_string(), "-o", "comm="]).output() {
        let output = String::from_utf8_lossy(&output.stdout).trim().to_string();
        
        if output.contains("bash") {
            return Ok("bash".to_string());
        } else if output.contains("zsh") {
            return Ok("zsh".to_string());
        } else if output.contains("fish") {
            return Ok("fish".to_string());
        }
    }
    
    // Default to bash
    Ok("bash".to_string())
}
