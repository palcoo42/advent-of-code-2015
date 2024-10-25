use std::path::Path;

use common::reader::{text_reader::TextReader, text_reader_error::TextReaderError};
use once_cell::sync::Lazy;
use regex::Regex;

use crate::example::sue::Sue;

use super::aunts::Aunts;

pub struct Reader {}

impl Reader {
    pub fn read_aunts(path: &Path) -> Result<Aunts, TextReaderError> {
        let reader = TextReader::new(path.to_path_buf());
        let lines = reader.read_lines(500)?;

        let mut sues = Vec::new();

        for line in lines {
            sues.push(Self::parse_sue(&line)?);
        }

        Ok(Aunts::new(sues))
    }

    fn parse_sue(line: &str) -> Result<Sue, TextReaderError> {
        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r#"^Sue\s+(\d+): (\w+): (\d+), (\w+): (\d+), (\w+): (\d+)"#)
                .expect("Failed to create 'aunt' regex")
        });

        if let Some(captures) = RE.captures(line) {
            let id = captures[1].parse::<u32>().map_err(|err| {
                TextReaderError::GenericError(format!(
                    "Failed to convert id '{}' to u32 with error '{}'",
                    &captures[1], err
                ))
            })?;

            let item_1 = &captures[2];
            let count_1 = captures[3].parse::<u32>().map_err(|err| {
                TextReaderError::GenericError(format!(
                    "Failed to convert count_1 '{}' to u32 with error '{}'",
                    &captures[3], err
                ))
            })?;

            let item_2 = &captures[4];
            let count_2 = captures[5].parse::<u32>().map_err(|err| {
                TextReaderError::GenericError(format!(
                    "Failed to convert count_2 '{}' to u32 with error '{}'",
                    &captures[5], err
                ))
            })?;

            let item_3 = &captures[6];
            let count_3 = captures[7].parse::<u32>().map_err(|err| {
                TextReaderError::GenericError(format!(
                    "Failed to convert count_3 '{}' to u32 with error '{}'",
                    &captures[7], err
                ))
            })?;

            return Ok(Sue::new(
                id,
                vec![
                    (item_1.to_string(), count_1),
                    (item_2.to_string(), count_2),
                    (item_3.to_string(), count_3),
                ],
            ));
        }

        Err(TextReaderError::GenericError(format!(
            "Failed to parse line '{}'",
            line
        )))
    }
}

#[cfg(test)]
mod tests {
    use crate::example::{reader::Reader, sue::Sue};

    #[test]
    fn test_parse_sue() {
        assert_eq!(
            Reader::parse_sue("Sue 4: goldfish: 5, children: 8, perfumes: 3").unwrap(),
            Sue::new(
                4,
                vec![
                    ("goldfish".to_string(), 5),
                    ("children".to_string(), 8),
                    ("perfumes".to_string(), 3)
                ]
            )
        );
    }
}
