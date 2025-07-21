use std::error::Error;
use std::path::PathBuf;

use itertools::Itertools;
use puzzler::env::project;
use puzzler::parsers::parser::Parser;
use puzzler::puzzler::puzzle::Puzzle;

pub struct Solution {
    packages: Vec<usize>,
}

impl Puzzle for Solution {
    fn name(&self) -> &str {
        "--- Day 24: It Hangs in the Balance ---"
    }

    fn get_input_file_path(&self) -> Option<PathBuf> {
        Some(
            project::get_project_file("../input/day_24.txt")
                .unwrap_or_else(|err| panic!("Failed to fetch file ../input/day_24.txt [{err}]")),
        )
    }

    fn parse_content(&mut self, lines: Vec<String>) -> Result<(), Box<dyn Error>> {
        self.packages = Parser::parse_lines_to_unsigned_integer(lines)?;
        Ok(())
    }

    fn solve_part1(&mut self) -> Result<String, Box<dyn Error>> {
        let min_entaglement = Solution::find_min_quantum_entaglement(&self.packages, 3);
        Ok(min_entaglement.to_string())
    }

    fn solve_part2(&mut self) -> Result<String, Box<dyn Error>> {
        let min_entaglement = Solution::find_min_quantum_entaglement(&self.packages, 4);
        Ok(min_entaglement.to_string())
    }
}

impl Solution {
    pub fn new() -> Self {
        Self {
            packages: Vec::new(),
        }
    }

    fn find_min_quantum_entaglement(packages: &[usize], nr_of_groups: usize) -> usize {
        // Sort in decreasing order to find solution faster
        let mut packages = packages.to_vec();
        packages.sort();

        // Find target sum
        let target = packages.iter().sum::<usize>() / nr_of_groups;

        let mut valid_groups = Vec::new();
        let mut min_length = usize::MAX;

        // Collect all valid combinations
        for i in 1..packages.len() + 1 {
            // If number of combinations is greater than found min value, we can skip this iteration
            if min_length < i {
                continue;
            }

            // Find all groups with given target sum & with less than minimum length
            let mut groups = packages
                .iter()
                .combinations(i)
                .map(|comb| comb.into_iter().copied().collect::<Vec<_>>())
                .filter(|group| group.iter().sum::<usize>() == target)
                .filter(|group| group.len() <= min_length)
                .map(|group| group.to_vec())
                .collect::<Vec<_>>();

            // Update minimal length
            if let Some(current_min_length) = groups.iter().map(|group| group.len()).min() {
                min_length = std::cmp::min(min_length, current_min_length);
            }

            // Append found groups
            valid_groups.append(&mut groups);
        }

        // Find minimum entaglement
        valid_groups
            .iter()
            .filter(|group| group.len() == min_length)
            .map(|group| group.iter().product::<usize>())
            .min()
            .unwrap()
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
        assert_eq!(get_puzzle().solve_part1().unwrap(), "11266889531");
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(get_puzzle().solve_part2().unwrap(), "77387711");
    }

    #[test]
    fn test_find_min_quantum_entaglement_3_groups() {
        assert_eq!(
            Solution::find_min_quantum_entaglement(&[1, 2, 3, 4, 5, 7, 8, 9, 10, 11], 3),
            99
        );
    }

    #[test]
    fn test_find_min_quantum_entaglement_4_groups() {
        assert_eq!(
            Solution::find_min_quantum_entaglement(&[1, 2, 3, 4, 5, 7, 8, 9, 10, 11], 4),
            44
        );
    }
}
