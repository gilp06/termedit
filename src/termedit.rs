use std::io::{self, stdout};

mod event_handler;
mod opened_file;
mod cursor_movement;

use crossterm::{
    cursor::{self}, event::{read, DisableMouseCapture, EnableMouseCapture, Event}, style::Print, terminal::{self, disable_raw_mode, enable_raw_mode}, ExecutableCommand
};
use opened_file::OpenedFile;



pub struct TermEditApp {
    should_close: bool,
    opened_file: OpenedFile,
}

impl TermEditApp {
    pub fn create() -> TermEditApp {
        TermEditApp {
            should_close: false,
            opened_file: OpenedFile::create("test.txt").unwrap(),
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        self.init_terminal_window()?;
        
        stdout().execute(cursor::MoveTo(0,0))?;
        for line in &self.opened_file.lines {
            stdout().execute(Print(line.as_str()))?;
            stdout().execute(cursor::MoveToNextLine(1))?;
        }
        stdout().execute(cursor::MoveTo(0,0))?;

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
