use std::{fs::File, io::BufReader, path::Path, str::FromStr};

use common::reader::text_reader_error::TextReaderError;
use serde_json::{Map, Number, Value};

pub struct Abacus {
    value: Value,
}

impl FromStr for Abacus {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = serde_json::from_str(s)?;
        Ok(Abacus { value })
    }
}

impl Abacus {
    pub fn from_file(path: &Path) -> Result<Self, TextReaderError> {
        let file = File::open(path).map_err(|err| {
            TextReaderError::GenericError(format!(
                "Failed to open file '{:?}' with error '{}'",
                path, err
            ))
        })?;
        let reader = BufReader::new(file);

        let value = serde_json::from_reader(reader).map_err(|err| {
            TextReaderError::GenericError(format!(
                "Failed to parse JSON file '{:?}' with error '{}'",
                path, err
            ))
        })?;

        Ok(Self { value })
    }

    pub fn sum_of_all_numbers(&self) -> i64 {
        Self::count_json_value(&self.value)
    }

    fn count_json_value(value: &Value) -> i64 {
        match value {
            Value::Null => 0,
            Value::Bool(_) => 0,
            Value::Number(number) => Self::count_json_number(number),
            Value::String(_) => 0,
            Value::Array(values) => Self::count_json_array(values),
            Value::Object(objects) => Self::count_json_object(objects),
        }
    }

    fn count_json_number(number: &Number) -> i64 {
        number
            .as_i64()
            .unwrap_or_else(|| panic!("Failed to convert number '{}' to i64", number))
    }

    fn count_json_array(values: &[Value]) -> i64 {
        values.iter().map(Self::count_json_value).sum()
    }

    fn count_json_object(objects: &Map<String, Value>) -> i64 {
        objects
            .iter()
            .map(|(_, value)| Self::count_json_value(value))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_all_numbers() {
        assert_eq!(
            Abacus::from_str(r#"[1,2,3]"#).unwrap().sum_of_all_numbers(),
            6
        );
        assert_eq!(
            Abacus::from_str(r#"{"a":2,"b":4}"#)
                .unwrap()
                .sum_of_all_numbers(),
            6
        );

        assert_eq!(
            Abacus::from_str(r#"[[[3]]]"#).unwrap().sum_of_all_numbers(),
            3
        );
        assert_eq!(
            Abacus::from_str(r#"{"a":{"b":4},"c":-1}"#)
                .unwrap()
                .sum_of_all_numbers(),
            3
        );

        assert_eq!(
            Abacus::from_str(r#"{"a":[-1,1]}"#)
                .unwrap()
                .sum_of_all_numbers(),
            0
        );
        assert_eq!(
            Abacus::from_str(r#"[-1,{"a":1}]"#)
                .unwrap()
                .sum_of_all_numbers(),
            0
        );

        assert_eq!(Abacus::from_str(r#"[]"#).unwrap().sum_of_all_numbers(), 0);
        assert_eq!(Abacus::from_str(r#"{}"#).unwrap().sum_of_all_numbers(), 0);
    }
}
