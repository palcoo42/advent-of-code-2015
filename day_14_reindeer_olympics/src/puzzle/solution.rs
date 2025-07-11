use std::error::Error;

use puzzler::env::project;
use puzzler::parsers::parser::Parser;
use puzzler::puzzler::puzzle::Puzzle;

use crate::puzzle::reindeer::Reindeer;

const COMPETITION_SECONDS: usize = 2503;

pub struct Solution {
    reindeers: Vec<Reindeer>,
}

impl Puzzle for Solution {
    fn name(&self) -> &str {
        "--- Day 14: Reindeer Olympics ---"
    }

    fn get_input_file_path(&self) -> Option<std::path::PathBuf> {
        Some(
            project::get_project_file("../input/day_14.txt")
                .unwrap_or_else(|err| panic!("Failed to fetch file ../input/day_14.txt [{err}]")),
        )
    }

    fn parse_content(&mut self, lines: Vec<String>) -> Result<(), Box<dyn Error>> {
        self.reindeers = Parser::parse_lines_with_regex(
            lines,
            r"^(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.",
            |params| {
                if params.len() != 4 {
                    return Err(format!(
                        "Expected exactly 4 parameters, found '{}' [{:?}]",
                        params.len(),
                        params
                    )
                    .into());
                }

                // let name = params[0].clone();

                let speed = params[1]
                    .parse::<usize>()
                    .map_err(|err| format!("Failed to convert 'speed' to usize [{err}]"))?;

                let active = params[2]
                    .parse::<usize>()
                    .map_err(|err| format!("Failed to convert 'active' to usize [{err}]"))?;

                let resting = params[3]
                    .parse::<usize>()
                    .map_err(|err| format!("Failed to convert 'resting' to usize [{err}]"))?;

                Ok(Reindeer {
                    speed,
                    active,
                    resting,
                })
            },
        )?;
        Ok(())
    }

    fn solve_part1(&mut self) -> Result<String, Box<dyn Error>> {
        let max = self
            .reindeers
            .iter()
            .map(|r| r.distance(COMPETITION_SECONDS))
            .max()
            .unwrap();

        Ok(max.to_string())
    }

    fn solve_part2(&mut self) -> Result<String, Box<dyn Error>> {
        let max = Solution::run_race(&self.reindeers, COMPETITION_SECONDS);
        Ok(max.to_string())
    }
}

impl Solution {
    pub fn new() -> Self {
        Self { reindeers: vec![] }
    }

    fn run_race(reindeers: &[Reindeer], seconds: usize) -> usize {
        // Populate zero scores for everyone
        let mut scores = vec![0_usize; reindeers.len()];

        // Run rounds
        for seconds in 1..seconds {
            // Calculate distances for all reindeers
            let distances = reindeers
                .iter()
                .map(|r| r.distance(seconds))
                .collect::<Vec<_>>();

            // Find maximum distance
            let max = distances.iter().max().unwrap();

            for (index, dist) in distances.iter().enumerate() {
                if dist == max {
                    scores[index] += 1;
                }
            }
        }

        *scores.iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use puzzler::puzzler::puzzle::Puzzle;

    use crate::puzzle::{reindeer::Reindeer, solution::Solution};

    fn get_puzzle() -> Solution {
        let mut solution = Solution::new();

        solution
            .parse_input_file()
            .unwrap_or_else(|err| panic!("Failed to parse input file [{err}]"));

        solution
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(get_puzzle().solve_part1().unwrap(), "2660");
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(get_puzzle().solve_part2().unwrap(), "1256");
    }

    #[test]
    fn test_run_race() {
        let reindeers = [
            Reindeer {
                speed: 14,
                active: 10,
                resting: 127,
            },
            Reindeer {
                speed: 16,
                active: 11,
                resting: 162,
            },
        ];

        assert_eq!(Solution::run_race(&reindeers, 1000), 689);
    }
}
