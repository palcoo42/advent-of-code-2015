use std::path::Path;

use common::reader::{text_reader::TextReader, text_reader_error::TextReaderError};

use super::{direction::Direction, instructions::Instructions};

pub struct Reader {}

impl Reader {
    pub fn read_instructions(path: &Path) -> Result<Instructions, TextReaderError> {
        let reader = TextReader::new(path.to_path_buf());
        let line = reader.read_lines(1)?;

        if line.len() != 1 {
            return Err(TextReaderError::GenericError(
                "Input file should contain only a single line".to_string(),
            ));
        }

        let mut directions = Vec::with_capacity(line[0].len());

        for c in line[0].bytes() {
            let direction = match c {
                b'<' => Some(Direction::Left),
                b'>' => Some(Direction::Right),
                b'v' => Some(Direction::Down),
                b'^' => Some(Direction::Up),
                _ => None,
            };

            match direction {
                Some(direction) => directions.push(direction),
                None => {
                    return Err(TextReaderError::GenericError(format!(
                        "Invalid instruction '{}'",
                        c
                    )))
                }
            }
        }

        Ok(Instructions::new(directions))
    }
}
