use std::io::{self, stdout};

mod eventhandler;

use crossterm::{
    cursor,
    event::{read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseEventKind},
    terminal::{self, disable_raw_mode, enable_raw_mode},
    ExecutableCommand,
};

pub struct TermEditApp {
    should_close: bool,
}

impl TermEditApp {
    pub fn create() -> TermEditApp {
        TermEditApp {
            should_close: false,
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        self.init_terminal_window()?;
        loop {
            self.handle_loop()?;
            if self.should_close {
                break;
            }
        }
        self.close_terminal_window()?;
        Ok(())
    }

    fn handle_loop(&mut self) -> io::Result<()> {
        match read()? {
            Event::Key(key_event) => self.handle_key_event(key_event)?,
            Event::Mouse(mouse_event) => self.handle_mouse_event(mouse_event)?,
            _ => (),
        }
        Ok(())
    }

    fn init_terminal_window(&mut self) -> io::Result<()> {
        stdout().execute(terminal::EnterAlternateScreen)?;
        enable_raw_mode()?;

        stdout().execute(EnableMouseCapture)?;
        stdout().execute(cursor::SetCursorStyle::BlinkingBlock)?;

        Ok(())
    }

    fn close_terminal_window(&mut self) -> io::Result<()> {
        stdout().execute(cursor::SetCursorStyle::DefaultUserShape)?;
        stdout().execute(DisableMouseCapture)?;

        disable_raw_mode()?;
        stdout().execute(terminal::LeaveAlternateScreen)?;
        Ok(())
    }
}

impl Drop for TermEditApp {
    fn drop(&mut self) {
        let _ = self.close_terminal_window();
    }
}
