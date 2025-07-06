use std::error::Error;

use puzzler::env::project;
use puzzler::puzzler::puzzle::Puzzle;

pub struct Solution {
    words: Vec<String>,
}

impl Puzzle for Solution {
    fn name(&self) -> &str {
        "--- Day 08: Matchsticks ---"
    }

    fn get_input_file_path(&self) -> Option<std::path::PathBuf> {
        Some(
            project::get_project_file("../input/day_08.txt")
                .unwrap_or_else(|err| panic!("Failed to fetch file ../input/day_08.txt [{err}]")),
        )
    }

    fn parse_content(&mut self, lines: Vec<String>) -> Result<(), Box<dyn Error>> {
        self.words = lines;
        Ok(())
    }

    fn solve_part1(&mut self) -> Result<String, Box<dyn Error>> {
        let sum = self
            .words
            .iter()
            .map(|word| Self::get_count(word))
            .sum::<usize>();

        Ok(sum.to_string())
    }

    fn solve_part2(&mut self) -> Result<String, Box<dyn Error>> {
        let sum = self
            .words
            .iter()
            .map(|word| Self::get_encoded_count(word))
            .sum::<usize>();

        Ok(sum.to_string())
    }
}

impl Solution {
    pub fn new() -> Self {
        Self { words: vec![] }
    }

    fn get_count(value: &str) -> usize {
        value.len() - Self::interpreted_len(value)
    }

    fn interpreted_len(value: &str) -> usize {
        let mut length = 0;
        let mut i = 0;
        let bytes = value.as_bytes();

        while i < bytes.len() {
            if bytes[i] == b'\\' {
                // Two options
                match bytes[i + 1] {
                    b'x' => {
                        // hex value
                        i += 4;
                    }
                    _ => {
                        // escaped char
                        i += 2;
                    }
                }
            } else {
                // regular char
                i += 1;
            }
            length += 1;
        }

        length - 2 // Do not count leading and trailing "
    }

    fn get_encoded_count(word: &str) -> usize {
        Self::encoded_len(word) - word.len()
    }

    fn encoded_len(word: &str) -> usize {
        let mut length = 0;
        let bytes = word.as_bytes();

        bytes.iter().for_each(|b| match b {
            b'"' => length += 2,
            b'\\' => length += 2,
            _ => length += 1,
        });

        length + 2 // Count leading and trailing "
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
        assert_eq!(get_puzzle().solve_part1().unwrap(), "1371");
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(get_puzzle().solve_part2().unwrap(), "2117");
    }

    #[test]
    fn test_get_count() {
        assert_eq!(Solution::get_count(r#""""#), 2);
        assert_eq!(Solution::get_count(r#""abc""#), 2);
        assert_eq!(Solution::get_count(r#""aaa\"aaa""#), 3);
        assert_eq!(Solution::get_count(r#""\x27""#), 5);
        assert_eq!(Solution::get_count(r#""qludrkkvljljd\\xvdeum\x4e""#), 6);
        assert_eq!(
            Solution::get_count(r#""\"ihjqlhtwbuy\"hdkiv\"mtiqacnf\\""#),
            6
        );
    }

    #[test]
    fn test_get_encoded_count() {
        assert_eq!(Solution::get_encoded_count(r#""""#), 4);
        assert_eq!(Solution::get_encoded_count(r#""abc""#), 4);
        assert_eq!(Solution::get_encoded_count(r#""aaa\"aaa""#), 6);
        assert_eq!(Solution::get_encoded_count(r#""\x27""#), 5);
    }
}
