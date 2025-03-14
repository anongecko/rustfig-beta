// Re-export from the plugin module
pub mod api;

// Basic plugin system trait
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}

// Plugin manager
pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }
    
    pub fn register_plugin(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }
    
    pub fn initialize_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for plugin in &mut self.plugins {
            plugin.initialize()?;
        }
        Ok(())
    }
}
