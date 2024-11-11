use std::path::Path;

use common::reader::{text_reader::TextReader, text_reader_error::TextReaderError};
use once_cell::sync::Lazy;
use regex::Regex;

use super::coordinates::Coordinates;

pub struct Reader {}

impl Reader {
    pub fn read_coordinates(path: &Path) -> Result<Coordinates, TextReaderError> {
        let reader = TextReader::new(path.to_path_buf());
        let line = reader.read_lines(1)?;

        if line.is_empty() {
            return Err(TextReaderError::GenericError(String::from(
                "Input file is empty",
            )));
        }

        Self::parse_from_line(line.first().unwrap())
    }

    fn parse_from_line(line: &str) -> Result<Coordinates, TextReaderError> {
        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r#"^.*\s+row\s+(\w+),\s+column\s+(\w+)."#)
                .expect("Failed to create 'Coordinates' regex")
        });

        if let Some(captures) = RE.captures(line) {
            let row = captures[1].parse::<u32>().map_err(|err| {
                TextReaderError::GenericError(format!(
                    "Failed to convert row '{}' to u32 with error '{}'",
                    &captures[1], err
                ))
            })?;

            let col = captures[2].parse::<u32>().map_err(|err| {
                TextReaderError::GenericError(format!(
                    "Failed to convert col '{}' to u32 with error '{}'",
                    &captures[2], err
                ))
            })?;

            return Ok(Coordinates { row, col });
        }

        Err(TextReaderError::GenericError(format!(
            "Failed to parse line '{}'",
            line
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_from_line() {
        assert_eq!(Reader::parse_from_line("To continue, please consult the code grid in the manual.  Enter the code at row 123, column 456.
").unwrap(), Coordinates{ row: 123, col: 456 });
    }
}
