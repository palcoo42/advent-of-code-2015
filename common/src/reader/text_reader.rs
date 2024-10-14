use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use super::text_reader_error::TextReaderError;

pub struct TextReader {
    path: PathBuf,
}

impl TextReader {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn read_lines(&self, hint: usize) -> Result<Vec<String>, TextReaderError> {
        let file = File::open(self.path.clone()).map_err(|e| {
            TextReaderError::FileOpenError(String::from(self.path.to_str().unwrap()), e)
        })?;

        let reader = BufReader::new(file);

        let mut content = Vec::with_capacity(hint);

        for line in reader.lines() {
            match line {
                Ok(line) => content.push(line),
                Err(e) => {
                    return Err(TextReaderError::GenericError(format!(
                        "Failed to read line with error '{}'",
                        e
                    )))
                }
            };
        }

        Ok(content)
    }
}

#[cfg(test)]
mod tests {

    use crate::env::environment::get_project_root;

    use super::*;

    #[test]
    fn test_read_lines() {
        let input_file = get_project_root().join("resources").join("input.txt");

        let expected = [
            String::from("1"),
            String::from("12"),
            String::from("123"),
            String::from("1234"),
            String::from("12345"),
        ];

        let reader = TextReader::new(input_file);
        let lines = reader.read_lines(5).expect("Failed to read lines");

        assert_eq!(lines, expected);
    }

    #[test]
    fn test_read_lines_not_existing_file() {
        let reader = TextReader::new(PathBuf::from("/tmp/not-existing.txt"));

        assert!(reader.read_lines(1).is_err());
    }
}
