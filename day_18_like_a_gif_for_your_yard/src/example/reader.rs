use std::path::Path;

use common::reader::{text_reader::TextReader, text_reader_error::TextReaderError};

use super::{bulb::Bulb, grid::Grid, light::Light};

pub struct Reader {}

impl Reader {
    pub fn read_grid(path: &Path) -> Result<Grid, TextReaderError> {
        let reader = TextReader::new(path.to_path_buf());
        let lines = reader.read_lines(100)?;

        Self::read_grid_from_text(&lines)
    }

    pub fn read_grid_from_text(lines: &Vec<String>) -> Result<Grid, TextReaderError> {
        let mut bulbs = Vec::new();

        for line in lines {
            bulbs.extend(Self::parse_line(line)?);
        }

        Ok(Grid::new_from_puzzle(
            lines.len(),
            bulbs.len() / lines.len(),
            bulbs,
        ))
    }

    fn parse_line(line: &str) -> Result<Vec<Bulb>, TextReaderError> {
        let mut bulbs = Vec::with_capacity(line.len());

        for c in line.chars() {
            match c {
                '.' => {
                    bulbs.push(Bulb::new(Light::Off));
                }
                '#' => {
                    bulbs.push(Bulb::new(Light::On));
                }
                _ => {
                    return Err(TextReaderError::GenericError(format!(
                        "Unsupported character '{}'",
                        c
                    )));
                }
            }
        }

        Ok(bulbs)
    }
}
