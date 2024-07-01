use std::{borrow::Borrow, f32::consts::E, io::stdout};

use crossterm::{cursor, ExecutableCommand};

use super::{opened_file::OpenedFile, TermEditApp};

impl OpenedFile {
    pub fn move_up(&mut self) {
        if (self.current_line as usize) <= 0 {
            return;
        }
        if self.current_pos > self.lines[self.current_line as usize - 1].len() as u16 - 1 {
            self.current_pos = self.lines[self.current_line as usize - 1].len() as u16 - 1;
        }
        self.current_line -= 1;
    }

    pub fn move_down(&mut self) {
        if (self.current_line as usize) + 1 >= (self.lines.len()) {
            return;
        }

        if self.current_pos > self.lines[self.current_line as usize + 1].len() as u16 - 1 {
            self.current_pos = self.lines[self.current_line as usize + 1].len() as u16 - 1;
        }
        self.current_line += 1;
    }

    pub fn move_left(&mut self) {
        if self.current_pos as i16 - 1 < 0 {
            if (self.current_line as usize) <= 0 {
                return;
            }

            self.current_pos = self.lines[self.current_line as usize].len() as u16 - 1;
            self.current_line -= 1;
        } else {
            self.current_pos -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.current_pos + 1 >= self.lines[self.current_line as usize].len() as u16 {
            if (self.current_line as usize) + 1 >= (self.lines.len()) {
                return;
            }

            self.current_pos = 0;
            self.current_line += 1;
        } else {
            self.current_pos += 1;
        }
    }
}

impl TermEditApp {
    pub fn update_cursor_position(&mut self) {
        stdout()
            .execute(cursor::MoveTo(
                self.opened_file.current_pos,
                self.opened_file.current_line,
            ))
            .unwrap();
    }
}
