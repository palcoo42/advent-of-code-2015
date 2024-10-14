use std::path::PathBuf;

use common::reader::{text_reader::TextReader, text_reader_error::TextReaderError};

use super::{present::Present, presents::Presents};

pub struct Reader {}

impl Reader {
    pub fn read_presents(path: PathBuf, hint: usize) -> Result<Presents, TextReaderError> {
        let reader = TextReader::new(path);
        let lines = reader.read_lines(hint)?;
        let mut presents = Vec::new();

        for line in lines {
            if let Some(present) = Present::new(&line) {
                presents.push(present);
            } else {
                return Err(TextReaderError::GenericError(format!(
                    "Failed to create 'Present' from '{}'",
                    line
                )));
            }
        }

        Ok(Presents::new(presents))
    }
}
