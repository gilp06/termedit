use std::{
    fs::File,
    io::{self, stdout},
};

mod cursor_movement;
mod editor_renderer;
mod event_handler;
mod opened_file;

use crossterm::{
    cursor::{self},
    event::{read, DisableMouseCapture, EnableMouseCapture, Event},
    style::Print,
    terminal::{self, disable_raw_mode, enable_raw_mode},
    ExecutableCommand,
};
use editor_renderer::{FileRenderer, Render, RenderProperties};
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

        let mut terminal_size = terminal::size().unwrap();
        let mut file_renderer: FileRenderer = FileRenderer::create(
            RenderProperties {
                x_origin: 0,
                y_origin: 0,
                x_size: terminal_size.0,
                y_size: terminal_size.1,
            },
            &self.opened_file,
            0,
        );

        loop {

            file_renderer.render();
            match read()? {
                // Event::Key(key_event) => self.handle_key_event(key_event)?,
                // Event::Mouse(mouse_event) => self.handle_mouse_event(mouse_event)?,
                _ => (),
            }
            if self.should_close {
                break;
            }
        }
        self.close_terminal_window()?;
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
