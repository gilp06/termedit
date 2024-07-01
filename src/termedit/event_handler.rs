use std::io::Result;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, MouseEvent};

use super::TermEditApp;

impl TermEditApp {
    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        if key_event.kind == KeyEventKind::Release {
            return Ok(());
        }
        match key_event.code {
            KeyCode::Char('q') => self.should_close = true,
            KeyCode::Char('j') => {
                self.opened_file.move_down();
            }
            KeyCode::Char('k') => {
                self.opened_file.move_up();
            }
            KeyCode::Char('l') => {
                self.opened_file.move_right();
            }
            KeyCode::Char('h') => {
                self.opened_file.move_left();
            }
            _ => (),
        }
        self.update_cursor_position();
        Ok(())
    }

    pub fn handle_mouse_event(&mut self, mouse_event: MouseEvent) -> Result<()> {
        match mouse_event.kind {
            _ => (),
        }
        Ok(())
    }
}
