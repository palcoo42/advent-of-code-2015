use std::path::Path;

use common::reader::{text_reader::TextReader, text_reader_error::TextReaderError};
use once_cell::sync::Lazy;
use regex::Regex;

use crate::example::items::item::{ArmorValue, DamageValue};

use super::{
    character::{Character, HitPoints},
    items::weapon::Weapon,
};

pub struct Reader {}

impl Reader {
    pub fn read_boss(path: &Path) -> Result<Character, TextReaderError> {
        let reader = TextReader::new(path.to_path_buf());
        let lines = reader.read_lines(5)?;

        Self::read_from_lines(&lines)
    }

    #[inline]
    fn read_from_lines(lines: &[String]) -> Result<Character, TextReaderError> {
        // Double check for sufficient lines
        if lines.len() != 3 {
            return Err(TextReaderError::GenericError(format!(
                "Invalid number of lines '{}'",
                lines.len(),
            )));
        }

        let hit_points = Self::parse_hit_points(&lines[0])?;
        let damage = Self::parse_damage(&lines[1])?;
        let armor = Self::parse_armor(&lines[2])?;

        Ok(Character::new(
            hit_points,
            Weapon::new("Butcher", damage, armor, 0),
            None,
            None,
        ))
    }

    fn parse_hit_points(line: &str) -> Result<HitPoints, TextReaderError> {
        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r#"^Hit Points:\s+(\d+)"#).expect("Failed to create 'Hit Points' regex")
        });

        if let Some(captures) = RE.captures(line) {
            let hit_points = &captures[1].parse::<HitPoints>().map_err(|err| {
                TextReaderError::GenericError(format!(
                    "Failed to parse hit points '{}' with error {}",
                    &captures[1], err
                ))
            })?;
            return Ok(*hit_points);
        }

        Err(TextReaderError::GenericError(format!(
            "Failed to parse hit points '{}'",
            line
        )))
    }

    fn parse_damage(line: &str) -> Result<HitPoints, TextReaderError> {
        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r#"^Damage:\s+(\d+)"#).expect("Failed to create 'Damage' regex")
        });

        if let Some(captures) = RE.captures(line) {
            let damage = &captures[1].parse::<DamageValue>().map_err(|err| {
                TextReaderError::GenericError(format!(
                    "Failed to parse damage '{}' with error {}",
                    &captures[1], err
                ))
            })?;
            return Ok(*damage);
        }

        Err(TextReaderError::GenericError(format!(
            "Failed to parse damage '{}'",
            line
        )))
    }

    fn parse_armor(line: &str) -> Result<HitPoints, TextReaderError> {
        static RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r#"^Armor:\s+(\d+)"#).expect("Failed to create 'Armor' regex"));

        if let Some(captures) = RE.captures(line) {
            let armor = &captures[1].parse::<ArmorValue>().map_err(|err| {
                TextReaderError::GenericError(format!(
                    "Failed to parse armor '{}' with error {}",
                    &captures[1], err
                ))
            })?;
            return Ok(*armor);
        }

        Err(TextReaderError::GenericError(format!(
            "Failed to armor damage '{}'",
            line
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_from_lines() {
        let character = Reader::read_from_lines(&[
            "Hit Points: 123".to_string(),
            "Damage: 456".to_string(),
            "Armor: 789".to_string(),
        ])
        .unwrap();

        assert_eq!(character.get_hit_points(), 123);
        assert_eq!(character.calc_damage(), 456);
        assert_eq!(character.calc_armor(), 789);
    }
}
