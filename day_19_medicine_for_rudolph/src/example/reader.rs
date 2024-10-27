use std::path::Path;

use common::reader::{text_reader::TextReader, text_reader_error::TextReaderError};
use once_cell::sync::Lazy;
use regex::Regex;

use super::{machine::Machine, replacement::Replacement};

pub struct Reader {}

impl Reader {
    pub fn read_machine_and_molecule(path: &Path) -> Result<(Machine, String), TextReaderError> {
        let reader = TextReader::new(path.to_path_buf());
        let lines = reader.read_lines(50)?;

        let mut replacements = Vec::new();
        let mut molecule_line = false;
        let mut molecule = String::new();

        for line in lines {
            // Empty line indicates end of the replacements, after it is molecule as a last line
            if line.is_empty() {
                molecule_line = true;
                continue;
            }

            // If we are at the molecule line stop parsing
            if !molecule_line {
                let replacement = Self::parse_replacement(&line)?;
                replacements.push(replacement);
            } else {
                // Last line in the file
                molecule = line;
                break;
            }
        }

        Ok((Machine::new(replacements), molecule))
    }

    fn parse_replacement(line: &str) -> Result<Replacement, TextReaderError> {
        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r#"^(\w+)\s+=>\s+(\w+)"#).expect("Failed to create 'replacement' regex")
        });

        if let Some(captures) = RE.captures(line) {
            let from = &captures[1];
            let to = &captures[2];

            return Ok(Replacement::new(from.to_string(), to.to_string()));
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
    fn test_parse_replacement() {
        assert_eq!(
            Reader::parse_replacement("Al => ThF").unwrap(),
            Replacement::new("Al".to_string(), "ThF".to_string())
        );
    }
}
