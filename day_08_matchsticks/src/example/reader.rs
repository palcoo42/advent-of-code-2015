use std::path::Path;

use common::reader::{text_reader::TextReader, text_reader_error::TextReaderError};

use super::{literal::Literal, literals::Literals};

pub struct Reader {}

impl Reader {
    pub fn read_literals(path: &Path) -> Result<Literals, TextReaderError> {
        let reader = TextReader::new(path.to_path_buf());
        let lines = reader.read_lines(300)?;
        let literals = lines.into_iter().map(|line| Literal::new(&line)).collect();
        Ok(Literals::new(literals))
    }
}
