use std::io;

use crossterm::event::{KeyCode, KeyEvent, MouseEvent};

use super::TermEditApp;

impl TermEditApp
{
    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> io::Result<()>
    {
        match key_event.code {
            KeyCode::Char('q') => self.should_close = true,
            KeyCode::Char(':') => panic!("RAHHH"), 
            _ => ()
        }
        Ok(())
    }

    pub fn handle_mouse_event(&mut self, mouse_event: MouseEvent) -> io::Result<()>
    {
        match mouse_event.kind {
            _ => ()
        }
        Ok(())
    }
}