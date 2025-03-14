// Re-export from the terminal module
pub mod input;
pub mod render;
pub mod dropdown;

pub use self::input::InputHandler;
pub use self::render::Renderer;
pub use self::dropdown::Dropdown;

use std::error::Error;
use std::io;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use crate::{
    config::Config,
    shell::ShellIntegration,
    suggestion::engine::SuggestionEngine,
};

pub struct Terminal {
    input_handler: InputHandler,
    renderer: Renderer,
}

impl Terminal {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen)?;
        
        Ok(Self {
            input_handler: InputHandler::new(),
            renderer: Renderer::new()?,
        })
    }
    
    pub async fn run(
        &mut self,
        mut suggestion_engine: SuggestionEngine,
        shell_integration: Box<dyn ShellIntegration>,
        config: &Config,
    ) -> Result<(), Box<dyn Error>> {
        let mut current_input = String::new();
        let mut dropdown_visible = false;
        
        loop {
            // Process input
            if let Some(event) = self.input_handler.next_event(config.general.input_timeout_ms)? {
                match event {
                    Event::Key(KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL, .. }) => {
                        break;
                    },
                    Event::Key(KeyEvent { code: KeyCode::Tab, .. }) if !dropdown_visible => {
                        // Get current command line from shell
                        let cmd_line = shell_integration.get_current_command_line()?;
                        
                        // Generate suggestions (non-blocking)
                        let suggestions = suggestion_engine.get_suggestions(&cmd_line, 10).await;
                        
                        if !suggestions.is_empty() {
                            dropdown_visible = true;
                            self.renderer.render_dropdown(&suggestions, 0)?;
                        }
                    },
                    // Handle other key events...
                    _ => {
                        // Update current input
                        // This is simplified - actual implementation would integrate with shell
                        current_input = shell_integration.get_current_command_line()?;
                        
                        // If input changed, update suggestions
                        if dropdown_visible {
                            let suggestions = suggestion_engine.get_suggestions(&current_input, 10).await;
                            if suggestions.is_empty() {
                                dropdown_visible = false;
                                self.renderer.clear_dropdown()?;
                            } else {
                                self.renderer.render_dropdown(&suggestions, 0)?;
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
    }
}
