use std::path::Path;

use common::reader::{text_reader::TextReader, text_reader_error::TextReaderError};

pub struct Reader {}

impl Reader {
    pub fn read_packages(path: &Path) -> Result<Vec<u32>, TextReaderError> {
        let reader = TextReader::new(path.to_path_buf());
        let lines = reader.read_lines(30)?;

        let packages = Self::parse_from_lines(&lines)?;

        Ok(packages)
    }

    fn parse_from_lines(lines: &[String]) -> Result<Vec<u32>, TextReaderError> {
        let mut packages = Vec::new();

        for line in lines {
            let num = line.parse::<u32>().map_err(|err| {
                TextReaderError::GenericError(format!(
                    "Failed to convert '{}' to u32 with error '{}'",
                    line, err
                ))
            })?;

            packages.push(num);
        }

        Ok(packages)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_from_lines() {
        let lines = vec![String::from("1"), String::from("42")];
        assert_eq!(Reader::parse_from_lines(&lines).unwrap(), vec![1, 42]);
    }

    #[test]
    fn test_parse_from_lines_failed() {
        let lines = vec![String::from("1"), String::from("uups")];
        assert!(Reader::parse_from_lines(&lines).is_err());
    }
}
