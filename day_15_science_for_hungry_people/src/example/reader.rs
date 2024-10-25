use std::path::Path;

use common::reader::{text_reader::TextReader, text_reader_error::TextReaderError};
use once_cell::sync::Lazy;
use regex::Regex;

use super::{ingredient::Ingredient, recipe::Recipe};

pub struct Reader {}

impl Reader {
    pub fn read_recipe(path: &Path) -> Result<Recipe, TextReaderError> {
        let reader = TextReader::new(path.to_path_buf());
        let lines = reader.read_lines(10)?;

        let mut ingredients = Vec::new();

        for line in lines {
            ingredients.push(Self::parse_ingredient(&line)?);
        }

        Ok(Recipe::new(ingredients))
    }

    fn parse_ingredient(line: &str) -> Result<Ingredient, TextReaderError> {
        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r#"^(\w+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)"#).expect("Failed to create 'ingredient' regex")
        });

        if let Some(captures) = RE.captures(line) {
            let _name = &captures[1];
            let capacity = captures[2].parse::<i32>().map_err(|err| {
                TextReaderError::GenericError(format!(
                    "Failed to convert capacity '{}' to i32 with error '{}'",
                    &captures[2], err
                ))
            })?;
            let durability = captures[3].parse::<i32>().map_err(|err| {
                TextReaderError::GenericError(format!(
                    "Failed to convert durability '{}' to i32 with error '{}'",
                    &captures[3], err
                ))
            })?;
            let flavor = captures[4].parse::<i32>().map_err(|err| {
                TextReaderError::GenericError(format!(
                    "Failed to convert flavor '{}' to i32 with error '{}'",
                    &captures[4], err
                ))
            })?;
            let texture = captures[5].parse::<i32>().map_err(|err| {
                TextReaderError::GenericError(format!(
                    "Failed to convert texture '{}' to i32 with error '{}'",
                    &captures[5], err
                ))
            })?;
            let calories = captures[6].parse::<u32>().map_err(|err| {
                TextReaderError::GenericError(format!(
                    "Failed to convert texture '{}' to u32 with error '{}'",
                    &captures[6], err
                ))
            })?;

            return Ok(Ingredient {
                capacity,
                durability,
                flavor,
                texture,
                calories,
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
    fn test_parse_parse_ingredient() {
        assert_eq!(
            Reader::parse_ingredient(
                "Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"
            )
            .unwrap(),
            Ingredient {
                capacity: 2,
                durability: 3,
                flavor: -2,
                texture: -1,
                calories: 3
            }
        );
    }
}
