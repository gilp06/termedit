use std::{
    fs::File,
    io::{stdout, Cursor, Write},
};

use crossterm::{
    cursor,
    style::{Color, Colors, Print, PrintStyledContent, SetColors},
    terminal::size,
    ExecutableCommand, QueueableCommand,
};

use super::{opened_file::OpenedFile, TermEditApp};

pub trait Render {
    fn render(&mut self);
}

pub struct RenderProperties {
    pub x_origin: u16,
    pub y_origin: u16,
    pub x_size: u16,
    pub y_size: u16,
}

pub struct FileRenderer<'fl> {
    props: RenderProperties,
    file: &'fl OpenedFile,
    start_line: u16,
}

impl<'fl> FileRenderer<'fl> {
    pub fn create(
        props: RenderProperties,
        file: &'fl OpenedFile,
        start_line: u16,
    ) -> FileRenderer<'fl> {
        FileRenderer {
            props: props,
            file: file,
            start_line: start_line,
        }
    }
}

impl<'fl> Render for FileRenderer<'fl> {
    fn render(&mut self) {
        let size_offset: u16 = ((self.start_line + self.props.y_size) as f32).log10() as u16;
        for i in 0..self.props.y_size {
            stdout()
                .queue(cursor::MoveTo(
                    self.props.x_origin,
                    self.props.y_origin + (i) as u16,
                ))
                .unwrap();
            stdout().queue(Print(format!("{}", i))).unwrap();

            if self.start_line as usize + i as usize >= self.file.lines.len() {
                continue;
            }

            stdout()
                .queue(cursor::MoveRight(size_offset))
                .unwrap()
                .queue(Print(
                    self.file.lines[i as usize + self.start_line as usize].as_str(),
                ))
                .unwrap();
        }
        stdout()
            .queue(cursor::MoveTo(self.props.x_origin + size_offset + 1, self.props.y_origin))
            .unwrap();
        stdout().flush().unwrap();
    }
}
