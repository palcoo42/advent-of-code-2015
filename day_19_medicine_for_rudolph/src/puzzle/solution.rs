use std::collections::HashSet;
use std::error::Error;
use std::path::PathBuf;

use puzzler::env::project;
use puzzler::parsers::parser::Parser;
use puzzler::puzzler::puzzle::Puzzle;

use crate::puzzle::recipes::Recipes;

pub struct Solution {
    recipes: Recipes,
    molecule: String,
}

impl Puzzle for Solution {
    fn name(&self) -> &str {
        "--- Day 19: Medicine for Rudolph ---"
    }

    fn get_input_file_path(&self) -> Option<PathBuf> {
        Some(
            project::get_project_file("../input/day_19.txt")
                .unwrap_or_else(|err| panic!("Failed to fetch file ../input/day_19.txt [{err}]")),
        )
    }

    fn parse_content(&mut self, lines: Vec<String>) -> Result<(), Box<dyn Error>> {
        // Split input file by empty line
        let mut groups = Parser::group_lines(lines);

        if groups.len() != 2 {
            return Err(
                format!("Exactly two groups are expected, found '{}'", groups.len()).into(),
            );
        }

        // First part are recipes
        let recipes = groups.remove(0);
        let recipes = Parser::parse_lines_with_regex(recipes, r"^(\w+) => (\w+)", |mut params| {
            if params.len() != 2 {
                return Err(
                    format!("Expected exactly 2 parameters, found '{}'", params.len()).into(),
                );
            }

            let key = params.remove(0);
            let replacement = params.remove(0);

            Ok((key, replacement))
        })?;

        for (key, replacement) in recipes {
            self.recipes.insert(&key, &[replacement.as_str()]);
        }

        // Second part is molecule
        self.molecule = groups.remove(0).remove(0);

        Ok(())
    }

    fn solve_part1(&mut self) -> Result<String, Box<dyn Error>> {
        let count = Solution::count_generated_molecules(&self.recipes, &self.molecule);
        Ok(count.to_string())
    }

    fn solve_part2(&mut self) -> Result<String, Box<dyn Error>> {
        let count = Solution::min_steps_fabricate_molecule(&self.recipes, &self.molecule);
        Ok(count.to_string())
    }
}

impl Solution {
    pub fn new() -> Self {
        Self {
            recipes: Recipes::new(),
            molecule: String::new(),
        }
    }

    fn count_generated_molecules(recipes: &Recipes, molecule: &str) -> usize {
        let mut unique_molecules = HashSet::new();

        for (key, replacements) in recipes.iter() {
            // Locate all positions of the key in molecule
            for (pos, _) in molecule.match_indices(key.as_str()) {
                // And generate new molecules
                for repl in replacements {
                    // New molecule: prefix part + replacement + postfix part
                    let new_molecule = format!(
                        "{}{}{}",
                        &molecule[0..pos],
                        repl,
                        &molecule[pos + key.len()..]
                    );
                    unique_molecules.insert(new_molecule);
                }
            }
        }

        unique_molecules.len()
    }

    fn min_steps_fabricate_molecule(recipes: &Recipes, molecule: &str) -> usize {
        // Let's use a trick. Reverse the order from the recipes (key and replacements) and
        // replace backwards. Count each replacement until we will find e (electron).
        let recipes = recipes.reverse();
        let keys = recipes.get_keys_descending_len();
        let mut new_molecule = molecule.to_string();
        let mut count = 0;

        // Repeat until all molecule elements are electorns
        while !new_molecule.as_str().chars().all(|b| b == 'e') {
            let issue_detector = count;

            // Go through all keys
            for key in &keys {
                let value = recipes
                    .get(key)
                    .expect("Failed to find key '{key}' in recipes");

                // Replace only one mapping at a time
                if let Some(pos) = new_molecule.find(key.as_str()) {
                    new_molecule.replace_range(pos..pos + key.len(), &value[0]);
                    count += 1;

                    // Start while again as molecule may be complete
                    break;
                }
            }

            // Check for infinite loop
            if issue_detector == count {
                panic!(
                    "Molecule was not changed during this round. This would lead to infinite loop :-("
                )
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(get_puzzle().solve_part1().unwrap(), "518");
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(get_puzzle().solve_part2().unwrap(), "200");
    }

    #[test]
    fn test_count_generated_molecules() {
        let mut recipes = Recipes::default();
        recipes.insert("H", &["HO", "OH"]);
        recipes.insert("O", &["HH"]);

        assert_eq!(Solution::count_generated_molecules(&recipes, "HOH"), 4);
        assert_eq!(Solution::count_generated_molecules(&recipes, "HOHOHO"), 7);
    }

    #[test]
    fn test_min_stepsfabricate_molecule() {
        let mut recipes = Recipes::default();
        recipes.insert("e", &["H", "O"]);
        recipes.insert("H", &["HO", "OH"]);
        recipes.insert("O", &["HH"]);

        assert_eq!(Solution::min_steps_fabricate_molecule(&recipes, "HOH"), 3);
        assert_eq!(
            Solution::min_steps_fabricate_molecule(&recipes, "HOHOHO"),
            6
        );
    }
}
