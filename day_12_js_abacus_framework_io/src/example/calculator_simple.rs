use super::calculator::Calculator;

pub struct CalculatorSimple {}

impl Calculator for CalculatorSimple {
    fn count_json_object(objects: &serde_json::Map<String, serde_json::Value>) -> i64 {
        objects
            .iter()
            .map(|(_, value)| Self::count_json_value(value))
            .sum()
    }
}
