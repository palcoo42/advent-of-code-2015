use serde_json::{Map, Number, Value};

/// Default implementation of calculator
pub trait Calculator {
    fn count(value: &Value) -> i64 {
        Self::count_json_value(value)
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

    fn count_json_object(_objects: &Map<String, Value>) -> i64 {
        0
    }
}
