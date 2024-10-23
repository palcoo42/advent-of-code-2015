use super::calculator::Calculator;

use serde_json::Value::String;

pub struct CalculatorComplex {}

impl Calculator for CalculatorComplex {
    fn count_json_object(objects: &serde_json::Map<std::string::String, serde_json::Value>) -> i64 {
        // Ignore objects if value of any of objects is "red"
        if objects.values().any(|v| match v {
            String(text) => text == "red",
            _ => false,
        }) {
            return 0;
        }

        objects
            .iter()
            .map(|(_, value)| Self::count_json_value(value))
            .sum()
    }
}
