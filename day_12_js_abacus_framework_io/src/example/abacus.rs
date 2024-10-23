use std::{fs::File, io::BufReader, path::Path, str::FromStr};

use common::reader::text_reader_error::TextReaderError;
use serde_json::Value;

use super::{
    calculator::Calculator, calculator_complex::CalculatorComplex,
    calculator_simple::CalculatorSimple,
};

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

    pub fn sum_simple(&self) -> i64 {
        CalculatorSimple::count(&self.value)
    }

    pub fn sum_complex(&self) -> i64 {
        CalculatorComplex::count(&self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_simple() {
        assert_eq!(Abacus::from_str(r#"[1,2,3]"#).unwrap().sum_simple(), 6);
        assert_eq!(
            Abacus::from_str(r#"{"a":2,"b":4}"#).unwrap().sum_simple(),
            6
        );

        assert_eq!(Abacus::from_str(r#"[[[3]]]"#).unwrap().sum_simple(), 3);
        assert_eq!(
            Abacus::from_str(r#"{"a":{"b":4},"c":-1}"#)
                .unwrap()
                .sum_simple(),
            3
        );

        assert_eq!(Abacus::from_str(r#"{"a":[-1,1]}"#).unwrap().sum_simple(), 0);
        assert_eq!(Abacus::from_str(r#"[-1,{"a":1}]"#).unwrap().sum_simple(), 0);

        assert_eq!(Abacus::from_str(r#"[]"#).unwrap().sum_simple(), 0);
        assert_eq!(Abacus::from_str(r#"{}"#).unwrap().sum_simple(), 0);
    }

    #[test]
    fn test_sum_complex() {
        assert_eq!(Abacus::from_str(r#"[1,2,3]"#).unwrap().sum_complex(), 6);
        assert_eq!(
            Abacus::from_str(r#"[1,{"c":"red","b":2},3]"#)
                .unwrap()
                .sum_complex(),
            4
        );
        assert_eq!(
            Abacus::from_str(r#"{"d":"red","e":[1,2,3,4],"f":5}"#)
                .unwrap()
                .sum_complex(),
            0
        );
        assert_eq!(Abacus::from_str(r#"[1,"red",5]"#).unwrap().sum_complex(), 6);
    }
}
