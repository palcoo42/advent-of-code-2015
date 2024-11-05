use std::path::Path;

use common::reader::{text_reader::TextReader, text_reader_error::TextReaderError};
use once_cell::sync::Lazy;
use regex::Regex;

use super::boss::Boss;

pub struct Reader {}

impl Reader {
    pub fn read_boss(path: &Path) -> Result<Boss, TextReaderError> {
        let reader = TextReader::new(path.to_path_buf());
        let lines = reader.read_lines(5)?;

        Self::read_from_lines(&lines)
    }

    fn read_from_lines(lines: &[String]) -> Result<Boss, TextReaderError> {
        // Double check number of lines
        if lines.len() < 2 {
            return Err(TextReaderError::GenericError(format!(
                "Insufficient lines in input '{}'",
                lines.len()
            )));
        }

        let hit_points = Self::parse_hit_points(&lines[0])?;
        let damage = Self::parse_damage(&lines[1])?;

        Ok(Boss::new(hit_points, damage))
    }

    fn parse_hit_points(line: &str) -> Result<u32, TextReaderError> {
        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r#"^Hit Points:\s+(\w+)"#).expect("Failed to create 'Hit Points' regex")
        });

        if let Some(captures) = RE.captures(line) {
            let hit_points = captures[1].parse::<u32>().map_err(|err| {
                TextReaderError::GenericError(format!(
                    "Failed to convert hit points '{}' to u32 with error '{}'",
                    &captures[1], err
                ))
            })?;

            return Ok(hit_points);
        }

        Err(TextReaderError::GenericError(format!(
            "Failed to parse hit points from line '{}'",
            line
        )))
    }

    fn parse_damage(line: &str) -> Result<u32, TextReaderError> {
        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r#"^Damage:\s+(\w+)"#).expect("Failed to create 'Damage' regex")
        });

        if let Some(captures) = RE.captures(line) {
            let damage = captures[1].parse::<u32>().map_err(|err| {
                TextReaderError::GenericError(format!(
                    "Failed to convert damage '{}' to u32 with error '{}'",
                    &captures[1], err
                ))
            })?;

            return Ok(damage);
        }

        Err(TextReaderError::GenericError(format!(
            "Failed to parse damage from line '{}'",
            line
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_from_lines() {
        let lines = vec!["Hit Points: 42".to_string(), "Damage: 5".to_string()];
        assert_eq!(Reader::read_from_lines(&lines).unwrap(), Boss::new(42, 5));
    }

    #[test]
    fn test_read_from_lines_invalid() {
        let lines = vec!["Hit Points: 42".to_string()];
        assert!(Reader::read_from_lines(&lines).is_err());

        let lines = vec!["Damage: 99".to_string()];
        assert!(Reader::read_from_lines(&lines).is_err());

        let lines = vec![];
        assert!(Reader::read_from_lines(&lines).is_err());
    }
}
