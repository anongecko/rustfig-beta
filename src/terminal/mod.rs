pub mod input;
pub mod render;
pub mod dropdown;
pub mod ghosting;

use std::error::Error;
use std::io;
use std::time::Instant;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    cursor::{position, MoveTo},
    execute,
};
use crate::{
    config::Config,
    shell::ShellIntegration,
    suggestion::engine::SuggestionEngine,
    prediction::PredictionEngine,
    utils::perf_metrics::PerformanceMetrics,
};

pub use self::input::InputHandler;
pub use self::render::Renderer;
pub use self::dropdown::Dropdown;
pub use self::ghosting::GhostTextRenderer;

pub struct Terminal {
    input_handler: InputHandler,
    renderer: Renderer,
    ghost_renderer: GhostTextRenderer,
    performance_metrics: PerformanceMetrics,
}

impl Terminal {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen)?;
        
        Ok(Self {
            input_handler: InputHandler::new(),
            renderer: Renderer::new()?,
            ghost_renderer: GhostTextRenderer::new(),
            performance_metrics: PerformanceMetrics::new("terminal"),
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
        
        // Create a prediction engine
        let prediction_engine = PredictionEngine::new(config);
        
        // Initialize ghost mode
        let ghost_enabled = config.general.enable_ghost_text.unwrap_or(true);
        self.ghost_renderer.set_enabled(ghost_enabled);
        
        loop {
            // Process input
            if let Some(event) = self.input_handler.next_event(config.general.input_timeout_ms)? {
                match event {
                    Event::Key(KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL, kind: KeyEventKind::Press, .. }) => {
                        break;
                    },
                    Event::Key(KeyEvent { code: KeyCode::Tab, kind: KeyEventKind::Press, .. }) => {
                        if dropdown_visible {
                            // Select current dropdown item
                            // Implementation depends on your dropdown selection system
                        } else if let Some(ghost) = self.ghost_renderer.accept_ghost() {
                            // Accept ghost text
                            shell_integration.apply_completion(&ghost)?;
                            current_input = shell_integration.get_current_command_line()?;
                        } else {
                            // No ghost text, show dropdown
                            let cmd_line = shell_integration.get_current_command_line()?;
                            
                            // Generate suggestions
                            let suggestions = suggestion_engine.get_suggestions(&cmd_line, 10).await;
                            
                            if !suggestions.is_empty() {
                                dropdown_visible = true;
                                self.renderer.render_dropdown(&suggestions, 0)?;
                            }
                        }
                    },
                    Event::Key(KeyEvent { code: KeyCode::Right, kind: KeyEventKind::Press, .. }) => {
                        // Accept ghost text on right arrow if at end of input
                        let cmd_line = shell_integration.get_current_command_line()?;
                        let (cur_x, _) = position()?;
                        
                        if cur_x as usize >= cmd_line.len() {
                            if let Some(ghost) = self.ghost_renderer.accept_ghost() {
                                shell_integration.apply_completion(&ghost)?;
                                current_input = shell_integration.get_current_command_line()?;
                            }
                        }
                    },
                    // Handle other key events...
                    _ => {
                        // Clear ghost text on any other key
                        self.ghost_renderer.clear_ghost_text()?;
                        
                        // Update current input
                        let new_input = shell_integration.get_current_command_line()?;
                        
                        // Only update predictions if input changed
                        if new_input != current_input {
                            current_input = new_input;
                            
                            // Get cursor position for ghost text
                            let (cur_x, cur_y) = position()?;
                            self.ghost_renderer.update_cursor_pos(cur_x, cur_y);
                            
                            // Update dropdown if visible
                            if dropdown_visible {
                                let suggestions = suggestion_engine.get_suggestions(&current_input, 10).await;
                                if suggestions.is_empty() {
                                    dropdown_visible = false;
                                    self.renderer.clear_dropdown()?;
                                } else {
                                    self.renderer.render_dropdown(&suggestions, 0)?;
                                }
                            }
                            
                            // Generate predictions for ghost text with performance timing
                            let timing_start = Instant::now();
                            let predictions = prediction_engine.predict(&current_input, 5).await;
                            let timing_elapsed = timing_start.elapsed();
                            
                            // Only show ghost text if predictions were fast enough (<5ms)
                            if timing_elapsed.as_millis() < 5 && !predictions.is_empty() {
                                let prediction = predictions.first();
                                self.ghost_renderer.render_ghost_text(&current_input, prediction)?;
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
