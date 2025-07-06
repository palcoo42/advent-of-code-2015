use std::error::Error;

use puzzler::env::project;
use puzzler::grids::point::Point;
use puzzler::parsers::parser::Parser;
use puzzler::puzzler::puzzle::Puzzle;

use crate::puzzle::action::Action;
use crate::puzzle::instruction::Instruction;

pub struct Solution {
    instructions: Vec<Instruction>,
}

impl Puzzle for Solution {
    fn name(&self) -> &str {
        "--- Day 06: Probably a Fire Hazard ---"
    }

    fn get_input_file_path(&self) -> Option<std::path::PathBuf> {
        Some(
            project::get_project_file("../input/day_06.txt")
                .unwrap_or_else(|err| panic!("Failed to fetch file ../input/day_06.txt [{err}]")),
        )
    }

    // Parse the file content for the puzzle. It is typically used in solve_partX() methods
    fn parse_content(&mut self, lines: Vec<String>) -> Result<(), Box<dyn Error>> {
        self.instructions = Parser::parse_lines_with_regex(
            lines,
            r"^(turn on|turn off|toggle)\s(\d+),(\d+)\sthrough\s(\w+),(\w+)",
            |params| {
                if params.len() != 5 {
                    return Err(format!(
                        "Invalid number of parts '{}', expected '5'",
                        params.len()
                    )
                    .into());
                }

                let action = match params[0].as_str() {
                    "turn on" => Action::TurnOn,
                    "turn off" => Action::TurnOff,
                    "toggle" => Action::Toggle,
                    a => return Err(format!("Invalid action [{a}]").into()),
                };

                let from_start = params[1]
                    .parse::<isize>()
                    .map_err(|e| format!("Failed to parse 'from start' [{e}]"))?;

                let from_end = params[2]
                    .parse::<isize>()
                    .map_err(|e| format!("Failed to parse 'from end' [{e}]"))?;

                let to_start = params[3]
                    .parse::<isize>()
                    .map_err(|e| format!("Failed to parse 'to start' [{e}]"))?;

                let to_end = params[4]
                    .parse::<isize>()
                    .map_err(|e| format!("Failed to parse 'to end' [{e}]"))?;

                Ok(Instruction {
                    action,
                    from: Point {
                        x: from_start,
                        y: from_end,
                    },
                    to: Point {
                        x: to_start,
                        y: to_end,
                    },
                })
            },
        )?;
        Ok(())
    }

    // Solve first part of the puzzle
    fn solve_part1(&mut self) -> Result<String, Box<dyn Error>> {
        // Lights are in a square 1000x1000
        let mut lights = vec![vec![false; 1000]; 1000];

        // Go through all instructions
        for instr in &self.instructions {
            // For every area mark the lights
            for row in instr.from.x..=instr.to.x {
                for col in instr.from.y..=instr.to.y {
                    match instr.action {
                        Action::TurnOn => lights[row as usize][col as usize] = true,
                        Action::TurnOff => lights[row as usize][col as usize] = false,
                        Action::Toggle => {
                            lights[row as usize][col as usize] = !lights[row as usize][col as usize]
                        }
                    }
                }
            }
        }

        Ok(lights
            .iter()
            .flatten()
            .filter(|light| **light)
            .count()
            .to_string())
    }

    // Solve second part of the puzzle
    fn solve_part2(&mut self) -> Result<String, Box<dyn Error>> {
        // Lights are in a square 1000x1000
        let mut lights = vec![vec![0_usize; 1000]; 1000];

        // Go through all instructions
        for instr in &self.instructions {
            // For every area mark the lights
            for row in instr.from.x..=instr.to.x {
                for col in instr.from.y..=instr.to.y {
                    match instr.action {
                        Action::TurnOn => lights[row as usize][col as usize] += 1,
                        Action::TurnOff => {
                            lights[row as usize][col as usize] =
                                lights[row as usize][col as usize].saturating_sub(1)
                        }
                        Action::Toggle => lights[row as usize][col as usize] += 2,
                    }
                }
            }
        }

        Ok(lights.iter().flatten().sum::<usize>().to_string())
    }
}

impl Solution {
    pub fn new() -> Self {
        Self {
            instructions: vec![],
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
        assert_eq!(get_puzzle().solve_part1().unwrap(), "377891");
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(get_puzzle().solve_part2().unwrap(), "14110788");
    }
}
