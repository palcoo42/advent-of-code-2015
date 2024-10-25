use std::path::Path;

use common::reader::{text_reader::TextReader, text_reader_error::TextReaderError};
use once_cell::sync::Lazy;
use regex::Regex;

use super::{herd::Herd, reindeer::Reindeer};

pub struct Reader {}

impl Reader {
    pub fn read_herd(path: &Path) -> Result<Herd, TextReaderError> {
        let reader = TextReader::new(path.to_path_buf());
        let lines = reader.read_lines(10)?;

        let mut herd = Vec::new();

        for line in &lines {
            let reindeer = Self::parse_reindeer(line)?;
            herd.push(reindeer);
        }

        Ok(Herd::new(herd))
    }

    fn parse_reindeer(line: &str) -> Result<Reindeer, TextReaderError> {
        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r#"(\w+)\s+can\s+fly\s+(\d+)\s+km\/s\s+for\s+(\d+)\s+seconds,\s+but\s+then\s+must\s+rest\s+for\s+(\d+)\s+seconds."#).expect("Failed to create 'reindeer' regex")
        });

        if let Some(captures) = RE.captures(line) {
            let name = &captures[1];
            let fly_speed = captures[2].parse::<u32>().map_err(|err| {
                TextReaderError::GenericError(format!(
                    "Failed to convert fly speed '{}' to u32 wit error '{}'",
                    &captures[2], err
                ))
            })?;
            let fly_duration = captures[3].parse::<u32>().map_err(|err| {
                TextReaderError::GenericError(format!(
                    "Failed to convert fly duration '{}' to u32 wit error '{}'",
                    &captures[2], err
                ))
            })?;
            let rest_duration = captures[4].parse::<u32>().map_err(|err| {
                TextReaderError::GenericError(format!(
                    "Failed to convert rest duration '{}' to u32 wit error '{}'",
                    &captures[2], err
                ))
            })?;
            return Ok(Reindeer::new(name, fly_speed, fly_duration, rest_duration));
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
    fn test_parse_reindeer() {
        assert_eq!(
            Reader::parse_reindeer(
                "Rudolph can fly 3 km/s for 15 seconds, but then must rest for 28 seconds."
            )
            .unwrap(),
            Reindeer::new("Rudolph", 3, 15, 28)
        );
    }
}
