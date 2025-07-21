use std::error::Error;
use std::path::PathBuf;

use puzzler::env::project;
use puzzler::parsers::parser::Parser;
use puzzler::puzzler::puzzle::Puzzle;

pub struct Solution {
    row: usize,
    col: usize,
}

impl Puzzle for Solution {
    fn name(&self) -> &str {
        "--- Day 25: Let It Snow ---"
    }

    fn get_input_file_path(&self) -> Option<PathBuf> {
        Some(
            project::get_project_file("../input/day_25.txt")
                .unwrap_or_else(|err| panic!("Failed to fetch file ../input/day_25.txt [{err}]")),
        )
    }

    fn parse_content(&mut self, lines: Vec<String>) -> Result<(), Box<dyn Error>> {
        let info = Parser::parse_lines_with_regex(
            lines,
            r"Enter the code at row (\d+), column (\d+).$",
            |params| {
                // Check params
                if params.len() != 2 {
                    return Err(format!("Expected 2 parameters, found {}", params.len()).into());
                }

                // Extract values
                let rows = params[0]
                    .parse::<usize>()
                    .map_err(|_err| format!("Failed to parse 'row' to usize [{}]", params[0]))?;

                let cols = params[1]
                    .parse::<usize>()
                    .map_err(|_err| format!("Failed to parse 'col' to usize [{}]", params[1]))?;

                Ok((rows, cols))
            },
        )?;

        if info.len() != 1 {
            return Err(format!(
                "Expected exactly one parsed row/col in input file, found {}",
                info.len()
            )
            .into());
        }

        (self.row, self.col) = info[0];

        Ok(())
    }

    fn solve_part1(&mut self) -> Result<String, Box<dyn Error>> {
        let code = Solution::generate_code(self.row, self.col);
        Ok(code.to_string())
    }

    fn solve_part2(&mut self) -> Result<String, Box<dyn Error>> {
        Ok("Not solved".into())
    }
}

impl Solution {
    pub fn new() -> Self {
        Self { row: 0, col: 0 }
    }

    fn generate_code(row: usize, col: usize) -> usize {
        // Code machine constants
        const INITIAL_CODE: usize = 20151125;
        const MULTIPLIER: usize = 252533;
        const DIVIDER: usize = 33554393;

        let index = Self::calculate_index(row, col);

        let mut code = INITIAL_CODE;

        for _ in 1..index {
            code = (code * MULTIPLIER) % DIVIDER;
        }

        code
    }

    fn calculate_index(target_row: usize, target_col: usize) -> usize {
        let mut row = 1;
        let mut col = 1;
        let mut index = 1;

        while row != target_row || col != target_col {
            if row == 1 {
                row = col + 1;
                col = 1;
            } else {
                row -= 1;
                col += 1;
            }
            index += 1;
        }

        index
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
        assert_eq!(get_puzzle().solve_part1().unwrap(), "19980801");
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(get_puzzle().solve_part2().unwrap(), "Not solved");
    }

    #[test]
    fn test_generate_code() {
        assert_eq!(Solution::generate_code(1, 1), 20151125);
        assert_eq!(Solution::generate_code(1, 2), 18749137);
        assert_eq!(Solution::generate_code(1, 3), 17289845);
        assert_eq!(Solution::generate_code(2, 1), 31916031);
        assert_eq!(Solution::generate_code(2, 2), 21629792);
        assert_eq!(Solution::generate_code(3, 1), 16080970);
        assert_eq!(Solution::generate_code(5, 6), 31663883);
        assert_eq!(Solution::generate_code(6, 5), 1534922);
        assert_eq!(Solution::generate_code(6, 6), 27995004);
    }

    #[test]
    fn test_calculate_index() {
        assert_eq!(Solution::calculate_index(1, 1), 1);
        assert_eq!(Solution::calculate_index(1, 2), 3);
        assert_eq!(Solution::calculate_index(1, 3), 6);
        assert_eq!(Solution::calculate_index(1, 4), 10);
        assert_eq!(Solution::calculate_index(1, 5), 15);
        assert_eq!(Solution::calculate_index(1, 6), 21);
        assert_eq!(Solution::calculate_index(2, 1), 2);
        assert_eq!(Solution::calculate_index(2, 2), 5);
        assert_eq!(Solution::calculate_index(2, 3), 9);
        assert_eq!(Solution::calculate_index(2, 4), 14);
        assert_eq!(Solution::calculate_index(2, 5), 20);
        assert_eq!(Solution::calculate_index(3, 1), 4);
        assert_eq!(Solution::calculate_index(3, 2), 8);
        assert_eq!(Solution::calculate_index(3, 3), 13);
        assert_eq!(Solution::calculate_index(3, 4), 19);
        assert_eq!(Solution::calculate_index(4, 1), 7);
        assert_eq!(Solution::calculate_index(4, 2), 12);
        assert_eq!(Solution::calculate_index(4, 3), 18);
        assert_eq!(Solution::calculate_index(5, 1), 11);
        assert_eq!(Solution::calculate_index(5, 2), 17);
        assert_eq!(Solution::calculate_index(6, 1), 16);
    }
}
