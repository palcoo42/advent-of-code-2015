use std::error::Error;
use std::path::PathBuf;

use puzzler::env::project;
use puzzler::parsers::parser::Parser;
use puzzler::puzzler::puzzle::Puzzle;

pub struct Solution {
    numbers: Vec<usize>,
}

impl Puzzle for Solution {
    fn name(&self) -> &str {
        "--- Day 17: No Such Thing as Too Much ---"
    }

    fn get_input_file_path(&self) -> Option<PathBuf> {
        Some(
            project::get_project_file("../input/day_17.txt")
                .unwrap_or_else(|err| panic!("Failed to fetch file ../input/day_17.txt [{err}]")),
        )
    }

    fn parse_content(&mut self, lines: Vec<String>) -> Result<(), Box<dyn Error>> {
        self.numbers = Parser::parse_lines_to_unsigned_integer(lines)?;
        Ok(())
    }

    fn solve_part1(&mut self) -> Result<String, Box<dyn Error>> {
        let count = Solution::count_combinations(&self.numbers, 150);
        Ok(count.to_string())
    }

    fn solve_part2(&mut self) -> Result<String, Box<dyn Error>> {
        let count = Solution::count_minimum_different_ways(&self.numbers, 150);
        Ok(count.to_string())
    }
}

impl Solution {
    pub fn new() -> Self {
        Self {
            numbers: Vec::new(),
        }
    }

    fn count_combinations(containers: &[usize], total: usize) -> usize {
        let mut solutions = Vec::new();

        // Sort containers from smallest to highest. This should speed up discarding dead ends.
        let mut containers = containers.to_vec();
        containers.sort();

        Self::dfs(&containers, total, &mut solutions);
        solutions.len()
    }

    fn count_minimum_different_ways(containers: &[usize], total: usize) -> usize {
        let mut solutions = Vec::new();

        // Sort containers from smallest to highest. This should speed up discarding dead ends.
        let mut containers = containers.to_vec();
        containers.sort();

        Self::dfs(&containers, total, &mut solutions);

        // Find minimum number of containers
        let min = solutions.iter().map(|c| c.len()).min().unwrap();

        let filtered = solutions
            .iter()
            .filter(|c| c.len() == min)
            .collect::<Vec<_>>();

        filtered.len()
    }

    fn dfs(containers: &[usize], total: usize, solutions: &mut Vec<Vec<usize>>) {
        // Create stack of pending investigations
        let mut stack = Vec::new();
        stack.push((0, 0, vec![])); // index, current_sum, path

        while let Some((index, current_sum, path)) = stack.pop() {
            // Check end conditions
            if current_sum == total {
                // Solution
                solutions.push(path.clone());
                continue;
            } else if current_sum > total || index >= containers.len() {
                // Dead end
                continue;
            }

            // Investigate next solutions

            // Exclude current item
            stack.push((index + 1, current_sum, path.clone()));

            // Include current item
            let mut path = path.clone();
            path.push(containers[index]);

            stack.push((index + 1, current_sum + containers[index], path));
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
        assert_eq!(get_puzzle().solve_part1().unwrap(), "1638");
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(get_puzzle().solve_part2().unwrap(), "17");
    }

    #[test]
    fn test_dfs() {
        let mut solutions = Vec::new();
        let mut expected = vec![vec![10, 15], vec![20, 5], vec![20, 5], vec![15, 5, 5]];

        Solution::dfs(&[20, 15, 10, 5, 5], 25, &mut solutions);

        assert_eq!(solutions.len(), 4);

        // To compare vectors in vectors we will sort inner and outer vector
        for vec in &mut expected {
            vec.sort();
        }
        for vec in &mut solutions {
            vec.sort();
        }

        // Sort the outer vectors
        expected.sort();
        solutions.sort();

        assert_eq!(expected, solutions);
    }

    #[test]
    fn test_count_combinations() {
        assert_eq!(Solution::count_combinations(&[20, 15, 10, 5, 5], 25), 4);
    }

    #[test]
    fn test_count_minimum_different_ways() {
        assert_eq!(
            Solution::count_minimum_different_ways(&[20, 15, 10, 5, 5], 25),
            3
        );
    }
}
