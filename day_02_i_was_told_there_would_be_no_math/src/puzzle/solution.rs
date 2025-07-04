use core::panic;
use std::error::Error;

use puzzler::env::project;
use puzzler::parsers::parser::Parser;
use puzzler::puzzler::puzzle::Puzzle;

use crate::puzzle::dimension::Dimensions;

pub struct Solution {
    dimensions: Vec<Dimensions>,
}

impl Puzzle for Solution {
    fn name(&self) -> &str {
        "--- Day 02: I Was Told There Would Be No Math ---"
    }

    fn get_input_file_path(&self) -> Option<std::path::PathBuf> {
        Some(
            project::get_project_file("../input/day_02.txt")
                .unwrap_or_else(|err| panic!("Failed to fetch file ../input/day_02.txt [{}]", err)),
        )
    }

    fn parse_content(&mut self, lines: Vec<String>) -> Result<(), Box<dyn Error>> {
        self.dimensions = Parser::parse_lines_with_regex(lines, r"^(\d+)x(\d+)x(\d+)", |params| {
            if params.len() != 3 {
                return Err(
                    format!("Exactly 3 parameters expected [found {}]", params.len()).into(),
                );
            }

            let length = params[0]
                .parse::<i64>()
                .map_err(|e| format!("Parsing length error '{}' [{}]", params[0], e))?;

            let width = params[1]
                .parse::<i64>()
                .map_err(|e| format!("Parsing width error '{}' [{}]", params[1], e))?;

            let height = params[2]
                .parse::<i64>()
                .map_err(|e| format!("Parsing height error '{}' [{}]", params[2], e))?;

            Ok(Dimensions {
                length,
                width,
                height,
            })
        })?;
        Ok(())
    }

    fn solve_part1(&mut self) -> Result<String, Box<dyn Error>> {
        let total: i64 = self.dimensions.iter().map(Solution::calculate_area).sum();
        Ok(total.to_string())
    }

    fn solve_part2(&mut self) -> Result<String, Box<dyn Error>> {
        let total: i64 = self.dimensions.iter().map(Solution::calculate_ribbon).sum();
        Ok(total.to_string())
    }
}

impl Solution {
    pub fn new() -> Self {
        Self { dimensions: vec![] }
    }

    pub fn calculate_area(dimension: &Dimensions) -> i64 {
        let areas = [
            dimension.length * dimension.width,
            dimension.width * dimension.height,
            dimension.height * dimension.length,
        ];

        // Areas of all + smallest area
        2 * areas.iter().sum::<i64>()
            + areas
                .iter()
                .min()
                .unwrap_or_else(|| panic!("Failed to find minimum area in [{:?}]", areas))
    }

    pub fn calculate_ribbon(dimension: &Dimensions) -> i64 {
        let sides = [dimension.length, dimension.width, dimension.height];

        // sum of two shortest sides + bow
        2 * (sides.iter().sum::<i64>()
            - sides
                .iter()
                .max()
                .unwrap_or_else(|| panic!("Failed to find maximum side in [{:?}]", sides)))
            + sides.iter().product::<i64>()
    }
}

#[cfg(test)]
mod tests {
    use puzzler::puzzler::puzzle::Puzzle;

    use crate::puzzle::{dimension::Dimensions, solution::Solution};

    fn get_puzzle() -> Solution {
        let mut solution = Solution::new();

        solution
            .parse_input_file()
            .unwrap_or_else(|err| panic!("Failed to parse input file [{}]", err));

        solution
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(get_puzzle().solve_part1().unwrap(), "1598415");
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(get_puzzle().solve_part2().unwrap(), "3812909");
    }

    #[test]
    fn test_calculate_area() {
        assert_eq!(
            Solution::calculate_area(&Dimensions {
                length: 2,
                width: 3,
                height: 4
            }),
            58
        );
        assert_eq!(
            Solution::calculate_area(&Dimensions {
                length: 1,
                width: 1,
                height: 10
            }),
            43
        );
    }

    #[test]
    fn test_calculate_ribbon() {
        assert_eq!(
            Solution::calculate_ribbon(&Dimensions {
                length: 2,
                width: 3,
                height: 4
            }),
            34
        );
        assert_eq!(
            Solution::calculate_ribbon(&Dimensions {
                length: 1,
                width: 1,
                height: 10
            }),
            14
        );
    }
}
