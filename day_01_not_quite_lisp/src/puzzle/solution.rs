use std::{error::Error, path::PathBuf};

use puzzler::{env::project, puzzler::puzzle::Puzzle};

pub struct Solution {
    instructions: String,
}

impl Puzzle for Solution {
    fn name(&self) -> &str {
        "--- Day 01: Not Quite Lisp ---"
    }

    fn get_input_file_path(&self) -> Option<PathBuf> {
        Some(
            project::get_project_file("../input/day_01.txt")
                .unwrap_or_else(|err| panic!("Failed to fetch file ../input/day_01.txt [{}]", err)),
        )
    }

    fn parse_content(&mut self, lines: Vec<String>) -> Result<(), Box<dyn Error>> {
        if lines.len() != 1 {
            return Err(format!(
                "Exactly one line is expected in input file, but '{}' found",
                lines.len()
            )
            .into());
        }

        self.instructions = lines.into_iter().next().unwrap();
        Ok(())
    }

    fn solve_part1(&mut self) -> Result<String, Box<dyn Error>> {
        let floors = Self::calculate_floor(&self.instructions);

        Ok(floors.to_string())
    }

    fn solve_part2(&mut self) -> Result<String, Box<dyn Error>> {
        let floor_number = Self::find_floor_to_enter_basement(&self.instructions)
            .ok_or_else(|| -> Box<dyn Error> { "Floor number not found".into() })?;

        Ok(floor_number.to_string())
    }
}

impl Solution {
    pub fn new() -> Self {
        Self {
            instructions: String::new(),
        }
    }

    fn calculate_floor(instructions: &str) -> i64 {
        // Loop only once
        let mut up = 0_i64;
        let mut down = 0_i64;

        for stair in instructions.bytes() {
            match stair {
                b'(' => up += 1,
                b')' => down += 1,
                x => panic!("Unxpected instruction '{}'", x),
            }
        }

        up - down
    }

    fn find_floor_to_enter_basement(instructions: &str) -> Option<i64> {
        // Loop only once
        let mut delta = 0_i64;

        for (i, stair) in instructions.bytes().enumerate() {
            match stair {
                b'(' => delta += 1,
                b')' => delta -= 1,
                x => panic!("Unxpected instruction '{}'", x),
            }

            if delta < 0 {
                return Some(i as i64 + 1);
            }
        }

        None
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
            .unwrap_or_else(|err| panic!("Failed to parse input file [{}]", err));

        solution
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(get_puzzle().solve_part1().unwrap(), "280");
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(get_puzzle().solve_part2().unwrap(), "1797");
    }

    #[test]
    fn test_calculate_floor() {
        assert_eq!(Solution::calculate_floor("(())"), 0);
        assert_eq!(Solution::calculate_floor("()()"), 0);
        assert_eq!(Solution::calculate_floor("((("), 3);
        assert_eq!(Solution::calculate_floor("(()(()("), 3);
        assert_eq!(Solution::calculate_floor("))((((("), 3);
        assert_eq!(Solution::calculate_floor("())"), -1);
        assert_eq!(Solution::calculate_floor("))("), -1);
        assert_eq!(Solution::calculate_floor(")))"), -3);
        assert_eq!(Solution::calculate_floor(")())())"), -3);
    }

    #[test]
    fn find_floor_to_enter_basement() {
        assert_eq!(Solution::find_floor_to_enter_basement(")"), Some(1));
        assert_eq!(Solution::find_floor_to_enter_basement("()())"), Some(5));
    }
}
