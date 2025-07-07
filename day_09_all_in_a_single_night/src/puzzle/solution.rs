use std::collections::{HashMap, HashSet};
use std::error::Error;

use itertools::Itertools;
use puzzler::env::project;
use puzzler::parsers::parser::Parser;
use puzzler::puzzler::puzzle::Puzzle;

pub struct Solution {
    routes: HashMap<(String, String), usize>,
}

impl Puzzle for Solution {
    fn name(&self) -> &str {
        "--- Day 09: All in a Single Night ---"
    }

    fn get_input_file_path(&self) -> Option<std::path::PathBuf> {
        Some(
            project::get_project_file("../input/day_09.txt")
                .unwrap_or_else(|err| panic!("Failed to fetch file ../input/day_09.txt [{err}]")),
        )
    }

    // Parse the file content for the puzzle
    fn parse_content(&mut self, lines: Vec<String>) -> Result<(), Box<dyn Error>> {
        let routes =
            Parser::parse_lines_with_regex(lines, r"^(\w+)\s+to\s+(\w+)\s+=\s+(\d+)", |params| {
                if params.len() != 3 {
                    return Err(
                        format!("Invalid number of params '{}', expected 3", params.len()).into(),
                    );
                }

                let distance = params[2]
                    .parse::<usize>()
                    .map_err(|err| format!("Failed to parse distance '{}' [{err}]", params[2]))?;

                Ok(((params[0].to_string(), params[1].to_string()), distance))
            })?;

        // Store both key combination
        for ((a, b), distance) in routes {
            self.routes.insert((a.clone(), b.clone()), distance);
            self.routes.insert((b, a), distance);
        }
        Ok(())
    }

    // Solve first part of the puzzle
    fn solve_part1(&mut self) -> Result<String, Box<dyn Error>> {
        let min = self.calculate_path_min();
        Ok(min.to_string())
    }

    // Solve second part of the puzzle
    fn solve_part2(&mut self) -> Result<String, Box<dyn Error>> {
        let max = self.calculate_path_max();
        Ok(max.to_string())
    }
}

impl Solution {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    fn calculate_path_min(&self) -> usize {
        let cities = self.get_all_cities();
        let paths = self.calculate_all_paths(&cities);
        *paths.iter().min().unwrap()
    }

    fn calculate_path_max(&self) -> usize {
        let cities = self.get_all_cities();
        let paths = self.calculate_all_paths(&cities);
        *paths.iter().max().unwrap()
    }

    fn get_all_cities(&self) -> Vec<&String> {
        let cities = self
            .routes
            .keys()
            .map(|(orig, _dest)| orig)
            .collect::<Vec<_>>();

        let unique: HashSet<&String> = cities.into_iter().collect();
        unique.into_iter().collect()
    }

    fn calculate_all_paths(&self, cities: &[&String]) -> Vec<usize> {
        // Generate all permutataions
        let paths = cities.iter().permutations(cities.len()).collect::<Vec<_>>();

        // Calculate distance for a given paths
        paths
            .iter()
            .map(|path| self.calculate_distance(path))
            .collect::<Vec<_>>()
    }

    fn calculate_distance(&self, path: &[&&String]) -> usize {
        path.windows(2)
            .map(|w| {
                let dist = self
                    .routes
                    .get(&(w[0].to_string(), w[1].to_string()))
                    .unwrap_or_else(|| panic!("Key ({},{}) not found in routes", w[0], w[1]));
                *dist
            })
            .sum::<usize>()
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
        assert_eq!(get_puzzle().solve_part1().unwrap(), "207");
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(get_puzzle().solve_part2().unwrap(), "804");
    }

    fn build_routes() -> Vec<((String, String), usize)> {
        vec![
            ((String::from("London"), String::from("Dublin")), 464_usize),
            ((String::from("Dublin"), String::from("London")), 464),
            ((String::from("London"), String::from("Belfast")), 518),
            ((String::from("Belfast"), String::from("London")), 518),
            ((String::from("Dublin"), String::from("Belfast")), 141),
            ((String::from("Belfast"), String::from("Dublin")), 141),
        ]
    }

    #[test]
    fn test_calculate_path_min() {
        let routes = build_routes();
        let mut solution = Solution::new();
        solution.routes = routes.into_iter().collect();

        assert_eq!(solution.calculate_path_min(), 605);
    }

    #[test]
    fn test_calculate_path_max() {
        let routes = build_routes();
        let mut solution = Solution::new();
        solution.routes = routes.into_iter().collect();

        assert_eq!(solution.calculate_path_max(), 982);
    }
}
