use std::error::Error;

use puzzler::env::project;
use puzzler::puzzler::puzzle::Puzzle;
use regex::Regex;
use serde_json::Value;

pub struct Solution {
    json: String,
}

impl Puzzle for Solution {
    fn name(&self) -> &str {
        "--- Day 12: JSAbacusFramework.io ---"
    }

    fn get_input_file_path(&self) -> Option<std::path::PathBuf> {
        Some(
            project::get_project_file("../input/day_12.txt")
                .unwrap_or_else(|err| panic!("Failed to fetch file ../input/day_12.txt [{err}]")),
        )
    }

    // Parse the file content for the puzzle
    fn parse_content(&mut self, lines: Vec<String>) -> Result<(), Box<dyn Error>> {
        if lines.len() != 1 {
            return Err(
                format!("Input file shall have only one line, {} found", lines.len()).into(),
            );
        }

        // Move huge string instead of clone
        let mut lines = lines;
        self.json = lines.remove(0);

        Ok(())
    }

    // Solve first part of the puzzle
    fn solve_part1(&mut self) -> Result<String, Box<dyn Error>> {
        let sum = Self::sum_numbers(&self.json);
        Ok(sum.to_string())
    }

    // Solve second part of the puzzle
    fn solve_part2(&mut self) -> Result<String, Box<dyn Error>> {
        let sum = Self::sum_numbers_red(&self.json);
        Ok(sum.to_string())
    }
}

impl Solution {
    pub fn new() -> Self {
        Self {
            json: String::new(),
        }
    }

    fn sum_numbers(json: &str) -> i64 {
        // Find all positive and negative numbers; they are not in strings ""
        let regex = Regex::new(r"(\-?\d+)").expect("Failed to build regex");

        // Sum all numbers
        regex
            .find_iter(json)
            .map(|value| {
                value
                    .as_str()
                    .parse::<i64>()
                    .unwrap_or_else(|_| panic!("Failed to parse '{}' to i64", value.as_str()))
            })
            .sum()
    }

    fn sum_numbers_red(json: &str) -> i64 {
        // Ignore red in objects
        let value: Value =
            serde_json::from_str(json).unwrap_or_else(|err| panic!("Failed to parse json [{err}]"));

        Self::sum_non_red_only(&value)
    }

    // Use recursion to solve JSON tree
    fn sum_non_red_only(value: &Value) -> i64 {
        match value {
            Value::Number(number) => number.as_i64().unwrap(),
            Value::Array(values) => {
                // Sum all value in array
                values.iter().map(Self::sum_non_red_only).sum()
            }
            Value::Object(objects) => {
                // Do not count objects with red
                if objects.values().any(|v| v.as_str() == Some("red")) {
                    return 0;
                }

                // Else count them
                objects.values().map(Self::sum_non_red_only).sum()
            }
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use puzzler::puzzler::puzzle::Puzzle;

    use crate::puzzle::solution::Solution;

    fn get_puzzle() -> Solution {
        let mut solution = Solution::new();

        solution
            .parse_input_file()
            .unwrap_or_else(|err| panic!("Failed to parse input file [{err}]"));

        solution
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(get_puzzle().solve_part1().unwrap(), "156366");
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(get_puzzle().solve_part2().unwrap(), "96852");
    }

    #[test]
    fn test_sum_numbers() {
        assert_eq!(Solution::sum_numbers(r#"[1,2,3]"#), 6);
        assert_eq!(Solution::sum_numbers(r#"{"a":2,"b":4}"#), 6);
        assert_eq!(Solution::sum_numbers(r#"[[[3]]]"#), 3);
        assert_eq!(Solution::sum_numbers(r#"{"a":{"b":4},"c":-1}"#), 3);
        assert_eq!(Solution::sum_numbers(r#"{"a":[-1,1]}"#), 0);
        assert_eq!(Solution::sum_numbers(r#"[-1,{"a":1}]"#), 0);
        assert_eq!(Solution::sum_numbers(r#"[]"#), 0);
        assert_eq!(Solution::sum_numbers(r#"{}"#), 0);
    }

    #[test]
    fn test_sum_numbers_red() {
        assert_eq!(Solution::sum_numbers_red(r#"[1,2,3]"#), 6);
        assert_eq!(Solution::sum_numbers_red(r#"[1,{"c":"red","b":2},3]"#), 4);
        assert_eq!(
            Solution::sum_numbers_red(r#"{"d":"red","e":[1,2,3,4],"f":5}"#),
            0
        );
        assert_eq!(Solution::sum_numbers_red(r#"[1,"red",5]"#), 6);
    }
}
