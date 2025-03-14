use std::error::Error;
use std::process;

mod config;
mod terminal;
mod shell;
mod suggestion;
mod ai;
mod plugin;
mod utils;
mod prediction;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize application with error handling
    if let Err(e) = run().await {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
    Ok(())
}

async fn run() -> Result<(), Box<dyn Error>> {
    // Load configuration
    let config = config::loader::load_config()?;
    
    // Initialize terminal
    let mut term = terminal::Terminal::new()?;
    
    // Initialize suggestion engine
    let suggestion_engine = suggestion::engine::SuggestionEngine::new(&config);
    
    // Initialize shell integration
    let shell_integration = shell::detect_and_initialize()?;
    
    // Main event loop
    term.run(suggestion_engine, shell_integration, &config).await?;
    
    Ok(())
}
