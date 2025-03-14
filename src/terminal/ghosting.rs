use std::io::{self, Write};
use crossterm::{
    style::{Color, Print, SetForegroundColor, ResetColor},
    cursor::{SavePosition, RestorePosition, MoveTo},
    QueueableCommand,
    terminal::size,
};
use crate::prediction::models::Prediction;

/// Renders ghost text in the terminal
pub struct GhostTextRenderer {
    ghost_color: Color,
    enabled: bool,
    current_ghost: Option<String>,
    cursor_pos: (u16, u16),
}

impl GhostTextRenderer {
    pub fn new() -> Self {
        Self {
            ghost_color: Color::DarkGrey,
            enabled: true,
            current_ghost: None,
            cursor_pos: (0, 0),
        }
    }
    
    /// Set ghost text color
    pub fn set_color(&mut self, color: Color) {
        self.ghost_color = color;
    }
    
    /// Enable or disable ghost text
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    /// Update current cursor position
    pub fn update_cursor_pos(&mut self, x: u16, y: u16) {
        self.cursor_pos = (x, y);
    }
    
    /// Render ghost text at current cursor position
    pub fn render_ghost_text(&mut self, current_input: &str, prediction: Option<&Prediction>) -> io::Result<()> {
        if !self.enabled {
            return Ok(());
        }
        
        // Clear any existing ghost text
        self.clear_ghost_text()?;
        
        let ghost_text = match prediction {
            Some(pred) => pred.get_ghost_text(current_input),
            None => String::new(),
        };
        
        if ghost_text.is_empty() {
            self.current_ghost = None;
            return Ok(());
        }
        
        // Store current ghost text
        self.current_ghost = Some(ghost_text.clone());
        
        // Get terminal size to avoid drawing off-screen
        let (term_width, _) = size()?;
        
        // Calculate visible ghost text
        let visible_ghost = if self.cursor_pos.0 + ghost_text.len() as u16 > term_width {
            let visible_len = term_width.saturating_sub(self.cursor_pos.0) as usize;
            &ghost_text[..visible_len.min(ghost_text.len())]
        } else {
            &ghost_text
        };
        
        if visible_ghost.is_empty() {
            return Ok(());
        }
        
        // Render ghost text
        let mut stdout = io::stdout();
        stdout.queue(SavePosition)?
              .queue(SetForegroundColor(self.ghost_color))?
              .queue(Print(visible_ghost))?
              .queue(ResetColor)?
              .queue(RestorePosition)?;
        
        stdout.flush()?;
        
        Ok(())
    }
    
    /// Clear existing ghost text
    pub fn clear_ghost_text(&self) -> io::Result<()> {
        if !self.enabled || self.current_ghost.is_none() {
            return Ok(());
        }
        
        if let Some(ghost) = &self.current_ghost {
            let mut stdout = io::stdout();
            
            // Save current position
            stdout.queue(SavePosition)?;
            
            // Clear ghost text by overwriting with spaces
            let spaces = " ".repeat(ghost.len());
            stdout.queue(Print(&spaces))?;
            
            // Restore position
            stdout.queue(RestorePosition)?;
            stdout.flush()?;
        }
        
        Ok(())
    }
    
    /// Accept the current ghost text
    pub fn accept_ghost(&mut self) -> Option<String> {
        let ghost = self.current_ghost.take();
        if let Some(ghost) = &ghost {
            // Print the ghost text in normal color
            let mut stdout = io::stdout();
            let _ = stdout
                .queue(Print(ghost))
                .and_then(|_| stdout.flush());
        }
        ghost
    }
}
