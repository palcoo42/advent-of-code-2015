use std::path::Path;

use common::reader::{text_reader::TextReader, text_reader_error::TextReaderError};
use once_cell::sync::Lazy;
use regex::Regex;

use super::{
    person::Person,
    table::{Happiness, Table},
};

/// Holds a single record as extracted from the file
#[derive(Debug, PartialEq)]
struct Record {
    person: Person,
    neighbor: Person,
    happiness: Happiness,
}

pub struct Reader {}

impl Reader {
    pub fn read_table(path: &Path) -> Result<Table, TextReaderError> {
        let reader = TextReader::new(path.to_path_buf());
        let lines = reader.read_lines(100)?;

        let mut table = Table::new();

        for line in lines {
            let record = Self::parse_record(&line)?;
            table.insert(record.person, record.neighbor, record.happiness);
        }

        Ok(table)
    }

    fn parse_record(line: &str) -> Result<Record, TextReaderError> {
        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r#"(\w+)\s+would\s+(gain|lose)\s+(\d+)\s+happiness\s+units\s+by\s+sitting\s+next\s+to\s+(\w+)"#).expect("Failed to create 'Record' regex")
        });

        if let Some(captures) = RE.captures(line) {
            let person = Person::new(&captures[1]);
            let action = &captures[2];
            let happiness = captures[3].parse::<i64>().map_err(|err| {
                TextReaderError::GenericError(format!(
                    "Failed to convert happiness '{}' to i64 with error '{}'",
                    &captures[3], err
                ))
            })?;
            let neighbor = Person::new(&captures[4]);

            let record = Record {
                person,
                neighbor,
                happiness: if action == "gain" {
                    happiness
                } else {
                    -happiness
                },
            };

            return Ok(record);
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
    fn test_parse_record() {
        assert_eq!(
            Reader::parse_record("Alice would gain 54 happiness units by sitting next to Bob.")
                .unwrap(),
            Record {
                person: Person::new("Alice"),
                neighbor: Person::new("Bob"),
                happiness: 54
            }
        );

        assert_eq!(
            Reader::parse_record("David would lose 7 happiness units by sitting next to Bob.")
                .unwrap(),
            Record {
                person: Person::new("David"),
                neighbor: Person::new("Bob"),
                happiness: -7
            }
        );
    }
}
