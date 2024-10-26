use std::path::Path;

use common::reader::{text_reader::TextReader, text_reader_error::TextReaderError};

use super::grid::Grid;

pub struct Reader {}

impl Reader {
    pub fn read_grid(path: &Path) -> Result<Grid, TextReaderError> {
        let reader = TextReader::new(path.to_path_buf());
        let lines = reader.read_lines(100)?;

        Ok(Grid::new_from_puzzle(lines))
    }
}
