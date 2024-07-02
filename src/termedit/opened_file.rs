use std::{
    borrow::Borrow, fs::File, io::{self, BufRead}
};

pub struct OpenedFile {
    pub lines: Vec<String>,
    pub file: File,
    pub current_line: u16,
    pub current_pos: u16,
}

impl OpenedFile {
    pub fn create(path: &str) -> io::Result<OpenedFile> {
        let mut line_vec: Vec<String> = Vec::new();

        let file = File::open(path)?;

        for line in io::BufReader::new(&file).lines().flatten() {
            line_vec.push(line);            
        }

        Ok(OpenedFile {
            lines: line_vec,
            file: file,
            current_line: 0,
            current_pos: 0,
        })
    }
}
