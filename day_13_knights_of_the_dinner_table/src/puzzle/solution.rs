use std::collections::{HashMap, HashSet};
use std::error::Error;

use itertools::Itertools;
use puzzler::env::project;
use puzzler::parsers::parser::Parser;
use puzzler::puzzler::puzzle::Puzzle;

pub struct Solution {
    rules: HashMap<(String, String), isize>,
}

impl Puzzle for Solution {
    fn name(&self) -> &str {
        "--- Day 13: Knights of the Dinner Table ---"
    }

    fn get_input_file_path(&self) -> Option<std::path::PathBuf> {
        Some(
            project::get_project_file("../input/day_13.txt")
                .unwrap_or_else(|err| panic!("Failed to fetch file ../input/day_13.txt [{err}]")),
        )
    }

    // Parse the file content for the puzzle
    fn parse_content(&mut self, lines: Vec<String>) -> Result<(), Box<dyn Error>> {
        let mappings: Vec<((String, String), isize)> = Parser::parse_lines_with_regex(
            lines,
            r"^(\w+) would (\w+) (\d+) happiness units by sitting next to (\w+).",
            |params| {
                if params.len() != 4 {
                    return Err(
                        format!("Exactly 4 parameters expected [found {}]", params.len()).into(),
                    );
                }

                let first = params[0].clone();
                let sign = params[1].clone();
                let mut hapiness = params[2]
                    .parse::<isize>()
                    .map_err(|err| format!("Failed to parse 'hapiness' to isize [{err}]"))?;
                let neighbour = params[3].clone();

                match sign.as_str() {
                    "gain" => {}
                    "lose" => hapiness *= -1,
                    _ => {
                        return Err(format!("Unexpected sign marker '{sign}'").into());
                    }
                };

                Ok(((first, neighbour), hapiness))
            },
        )?;

        self.rules = mappings.into_iter().collect();
        Ok(())
    }

    fn solve_part1(&mut self) -> Result<String, Box<dyn Error>> {
        let max_happiness = Self::calculate_max_happiness(&self.rules);
        Ok(max_happiness.to_string())
    }

    fn solve_part2(&mut self) -> Result<String, Box<dyn Error>> {
        // Add mayself to rules
        let persons = Self::collect_persons(&self.rules);
        for person in persons {
            self.rules.insert((person.clone(), String::from("me")), 0);
            self.rules.insert((String::from("me"), person), 0);
        }

        let max_happiness = Self::calculate_max_happiness(&self.rules);
        Ok(max_happiness.to_string())
    }
}

impl Solution {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    fn calculate_max_happiness(rules: &HashMap<(String, String), isize>) -> isize {
        let persons = Self::collect_persons(rules);

        let permutations = persons
            .iter()
            .permutations(persons.len())
            .collect::<Vec<_>>();

        permutations
            .iter()
            .map(|order| Self::calculate_happiness(rules, order))
            .max()
            .unwrap()
    }

    fn collect_persons(rules: &HashMap<(String, String), isize>) -> Vec<String> {
        let unique: HashSet<String> = rules.keys().map(|(first, _)| first.clone()).collect();
        unique.into_iter().collect()
    }

    fn calculate_happiness(
        rules: &HashMap<(String, String), isize>,
        order: &Vec<&String>,
    ) -> isize {
        // Artifically hack order so that windows() generates triple also for first and last
        let mut order_new = Vec::new();
        order_new.push(order[order.len() - 1].clone());
        for o in order {
            order_new.push((**o).clone());
        }
        order_new.push(order[0].clone());

        // Middle in the tripple is the person for which we calculate happiness
        let triples: Vec<_> = order_new.windows(3).collect();

        triples
            .iter()
            .map(|t| {
                let left = rules.get(&(t[1].clone(), t[0].clone())).unwrap();
                let right = rules.get(&(t[1].clone(), t[2].clone())).unwrap();
                left + right
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

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
        assert_eq!(get_puzzle().solve_part1().unwrap(), "664");
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(get_puzzle().solve_part2().unwrap(), "640");
    }

    fn construct_persons() -> HashMap<(String, String), isize> {
        let mut persons = HashMap::new();
        persons.insert((String::from("Alice"), String::from("Bob")), 54);
        persons.insert((String::from("Alice"), String::from("Carol")), -79);
        persons.insert((String::from("Alice"), String::from("David")), -2);
        persons.insert((String::from("Bob"), String::from("Alice")), 83);
        persons.insert((String::from("Bob"), String::from("Carol")), -7);
        persons.insert((String::from("Bob"), String::from("David")), -63);
        persons.insert((String::from("Carol"), String::from("Alice")), -62);
        persons.insert((String::from("Carol"), String::from("Bob")), 60);
        persons.insert((String::from("Carol"), String::from("David")), 55);
        persons.insert((String::from("David"), String::from("Alice")), 46);
        persons.insert((String::from("David"), String::from("Bob")), -7);
        persons.insert((String::from("David"), String::from("Carol")), 41);
        persons
    }

    #[test]
    fn test_calucalate_happiness() {
        let persons = construct_persons();
        let order = [
            String::from("Carol"),
            String::from("David"),
            String::from("Alice"),
            String::from("Bob"),
        ];
        let order: Vec<&String> = order.iter().collect();

        assert_eq!(Solution::calculate_happiness(&persons, &order), 330);
    }

    #[test]
    fn test_calucalate_max_happiness() {
        let persons = construct_persons();
        assert_eq!(Solution::calculate_max_happiness(&persons), 330);
    }
}
