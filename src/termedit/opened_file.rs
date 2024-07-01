use std::{
    fs::File,
    io::{self, BufRead},
    vec,
};

pub struct OpenedFile {
    pub lines: Vec<String>,
    pub file: File,
}

impl OpenedFile {
    pub fn create(path: &str) -> io::Result<OpenedFile> {
        let mut line_vec: Vec<String> = Vec::new();

        let file = File::open(path)?;

        for line in io::BufReader::new(&file).lines().flatten() {
            line_vec.push(line);
        }

        Ok(OpenedFile { lines: line_vec , file: file})
    }
}
