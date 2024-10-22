use std::path::Path;

use common::reader::{text_reader::TextReader, text_reader_error::TextReaderError};
use once_cell::sync::Lazy;
use regex::Regex;

use super::locations::Locations;

/// Holds information of parsed location
#[derive(Debug, PartialEq)]
struct Location {
    source: String,
    destination: String,
    distance: u32,
}

pub struct Reader {}

impl Reader {
    pub fn read_locations(path: &Path) -> Result<Locations, TextReaderError> {
        let reader = TextReader::new(path.to_path_buf());
        let lines = reader.read_lines(30)?;

        let mut locations = Locations::new();

        for line in lines {
            let location = Self::parse_line(&line)?;
            locations.insert(&location.source, &location.destination, location.distance);
        }

        Ok(locations)
    }

    fn parse_line(line: &str) -> Result<Location, TextReaderError> {
        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r#"^(\w+)\s+to\s+(\w+)\s+=\s+(\d+)"#)
                .expect("Failed to create regex to parse line")
        });

        if let Some(captures) = RE.captures(line) {
            let source = &captures[1];
            let destination = &captures[2];
            let distance = &captures[3].parse::<u32>().unwrap_or_else(|err| {
                panic!(
                    "Failed to convert distance '{}' to u32 with error '{}', line '{}' ",
                    &captures[2], err, line
                )
            });

            return Ok(Location {
                source: source.to_string(),
                destination: destination.to_string(),
                distance: *distance,
            });
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
    fn test_parse_line() {
        assert_eq!(
            Reader::parse_line("Amsterdam to Berlin = 42").expect("Failed to parse line"),
            Location {
                source: String::from("Amsterdam"),
                destination: String::from("Berlin"),
                distance: 42
            }
        );
    }
}
