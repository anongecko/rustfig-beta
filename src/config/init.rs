use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::Write;

/// Initialize configuration files for RustFig
pub fn initialize_config_files() -> Result<(), Box<dyn Error>> {
    // Determine user config directory
    let config_dir = get_config_dir()?;
    
    // Create directories
    create_directory_structure(&config_dir)?;
    
    // Create main configuration file if it doesn't exist
    create_file_if_not_exists(&config_dir.join("config.yaml"), include_str!("../../resources/config/config.yaml"))?;
    
    // Create keybindings configuration file if it doesn't exist
    create_file_if_not_exists(&config_dir.join("keybindings.yaml"), include_str!("../../resources/config/keybindings.yaml"))?;
    
    // Create appearance configuration file if it doesn't exist
    create_file_if_not_exists(&config_dir.join("appearance.yaml"), include_str!("../../resources/config/appearance.yaml"))?;
    
    // Create AI models configuration file if it doesn't exist
    create_file_if_not_exists(&config_dir.join("ai_models.yaml"), include_str!("../../resources/config/ai_models.yaml"))?;
    
    // Create themes directory and default themes
    let themes_dir = config_dir.join("themes");
    fs::create_dir_all(&themes_dir)?;
    
    // Copy built-in themes to user themes directory
    for theme_name in &["dark", "light", "nord", "dracula", "monokai", "solarized"] {
        let theme_file = format!("{}.yaml", theme_name);
        let theme_path = themes_dir.join(&theme_file);
        
        if !theme_path.exists() {
            // In a real implementation, we would include all themes as resources
            // Here, we're just creating placeholders
            let theme_content = format!("# {} theme for RustFig\nname: {}\n", theme_name, theme_name);
            fs::write(theme_path, theme_content)?;
        }
    }
    
    // Create plugin directory
    fs::create_dir_all(config_dir.join("plugins"))?;
    
    // Create snippets directory and example snippet
    let snippets_dir = config_dir.join("snippets");
    fs::create_dir_all(&snippets_dir)?;
    create_file_if_not_exists(
        &snippets_dir.join("examples.yaml"),
        "# Example snippets\nfind_large_files: \"find . -type f -size +100M -exec ls -lh {} \\;\"\nbackup_dir: \"tar -czvf backup_$(date +%Y%m%d).tar.gz\"\n"
    )?;
    
    Ok(())
}

/// Get user configuration directory
fn get_config_dir() -> Result<PathBuf, Box<dyn Error>> {
    // Check XDG_CONFIG_HOME first
    if let Ok(xdg_config_home) = std::env::var("XDG_CONFIG_HOME") {
        let dir = PathBuf::from(xdg_config_home).join("rustfig");
        return Ok(dir);
    }
    
    // Then try ~/.config/rustfig
    if let Some(home_dir) = dirs::home_dir() {
        let dir = home_dir.join(".config").join("rustfig");
        return Ok(dir);
    }
    
    // If all else fails, use ~/.rustfig
    if let Some(home_dir) = dirs::home_dir() {
        let dir = home_dir.join(".rustfig");
        return Ok(dir);
    }
    
    Err("Could not determine configuration directory".into())
}

/// Create the directory structure for configuration
fn create_directory_structure(config_dir: &Path) -> Result<(), Box<dyn Error>> {
    // Create main config directory
    fs::create_dir_all(config_dir)?;
    
    // Create subdirectories
    fs::create_dir_all(config_dir.join("themes"))?;
    fs::create_dir_all(config_dir.join("plugins"))?;
    fs::create_dir_all(config_dir.join("snippets"))?;
    fs::create_dir_all(config_dir.join("cache"))?;
    fs::create_dir_all(config_dir.join("logs"))?;
    fs::create_dir_all(config_dir.join("data"))?;
    
    Ok(())
}

/// Create a file if it doesn't exist
fn create_file_if_not_exists(path: &Path, content: &str) -> Result<(), Box<dyn Error>> {
    if !path.exists() {
        let mut file = fs::File::create(path)?;
        file.write_all(content.as_bytes())?;
    }
    
    Ok(())
}

/// Generate a default configuration file at the specified path
pub fn generate_default_config(output_path: &Path) -> Result<(), Box<dyn Error>> {
    // Make sure parent directory exists
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    // Write default configuration
    let mut file = fs::File::create(output_path)?;
    file.write_all(include_str!("../../resources/config.yaml").as_bytes())?;
    
    Ok(())
}

/// Generate a default keybindings file at the specified path
pub fn generate_default_keybindings(output_path: &Path) -> Result<(), Box<dyn Error>> {
    // Make sure parent directory exists
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    // Write default keybindings
    let mut file = fs::File::create(output_path)?;
    file.write_all(include_str!("../../resources/keybindings.yaml").as_bytes())?;
    
    Ok(())
}

/// Generate a default appearance file at the specified path
pub fn generate_default_appearance(output_path: &Path) -> Result<(), Box<dyn Error>> {
    // Make sure parent directory exists
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    // Write default appearance
    let mut file = fs::File::create(output_path)?;
    file.write_all(include_str!("../../resources/appearance.yaml").as_bytes())?;
    
    Ok(())
}

/// Generate a default AI models file at the specified path
pub fn generate_default_ai_models(output_path: &Path) -> Result<(), Box<dyn Error>> {
    // Make sure parent directory exists
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    // Write default AI models
    let mut file = fs::File::create(output_path)?;
    file.write_all(include_str!("../../resources/ai_models.yaml").as_bytes())?;
    
    Ok(())
}
