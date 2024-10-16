use std::path::Path;

use common::reader::{text_reader::TextReader, text_reader_error::TextReaderError};

use super::words::Words;

pub struct Reader {}

impl Reader {
    pub fn read_words(path: &Path) -> Result<Words, TextReaderError> {
        let reader = TextReader::new(path.to_path_buf());
        let words = reader.read_lines(1000)?;

        Ok(Words::new(words))
    }
}
