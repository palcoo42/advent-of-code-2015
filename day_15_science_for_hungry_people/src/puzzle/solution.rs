use std::error::Error;

use puzzler::env::project;
use puzzler::parsers::parser::Parser;
use puzzler::puzzler::puzzle::Puzzle;

use crate::puzzle::ingredient::Ingredient;

pub struct Solution {
    ingredients: Vec<Ingredient>,
}

impl Puzzle for Solution {
    fn name(&self) -> &str {
        "--- Day 15: Science for Hungry People ---"
    }

    fn get_input_file_path(&self) -> Option<std::path::PathBuf> {
        Some(
            project::get_project_file("../input/day_15.txt")
                .unwrap_or_else(|err| panic!("Failed to fetch file ../input/day_15.txt [{err}]")),
        )
    }

    fn parse_content(&mut self, lines: Vec<String>) -> Result<(), Box<dyn Error>> {
        self.ingredients = Parser::parse_lines_with_regex(
            lines,
            r"^.*: capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (\d+)",
            |params| {
                if params.len() != 5 {
                    return Err(format!("Expected 5 parameters [{params:?}]").into());
                }

                let capacity = params[0]
                    .parse::<isize>()
                    .map_err(|err| format!("Failed to convert 'capacity' to isize [{err}]"))?;

                let durability = params[1]
                    .parse::<isize>()
                    .map_err(|err| format!("Failed to convert 'durability' to isize [{err}]"))?;

                let flavor = params[2]
                    .parse::<isize>()
                    .map_err(|err| format!("Failed to convert 'flavor' to isize [{err}]"))?;

                let texture = params[3]
                    .parse::<isize>()
                    .map_err(|err| format!("Failed to convert 'texture' to isize [{err}]"))?;

                let calories = params[4]
                    .parse::<usize>()
                    .map_err(|err| format!("Failed to convert 'calories' to usize [{err}]"))?;

                Ok(Ingredient {
                    capacity,
                    durability,
                    flavor,
                    texture,
                    calories,
                })
            },
        )?;
        Ok(())
    }

    fn solve_part1(&mut self) -> Result<String, Box<dyn Error>> {
        let best_score = Self::find_best_cookie(&self.ingredients, 100);
        Ok(best_score.to_string())
    }

    fn solve_part2(&mut self) -> Result<String, Box<dyn Error>> {
        let best_score = Self::find_best_cookie_calories(&self.ingredients, 100, 500);
        Ok(best_score.to_string())
    }
}

impl Solution {
    pub fn new() -> Self {
        Self {
            ingredients: Vec::new(),
        }
    }

    fn find_best_cookie(ingredients: &[Ingredient], total: usize) -> usize {
        // Use brute force
        let combinations = Self::generate_combinations(ingredients.len(), total);

        combinations
            .iter()
            .map(|counts| Self::calculate_score_and_calories(ingredients, counts).0)
            .max()
            .unwrap()
    }

    fn find_best_cookie_calories(
        ingredients: &[Ingredient],
        total: usize,
        calories: usize,
    ) -> usize {
        // Use brute force
        let combinations = Self::generate_combinations(ingredients.len(), total);

        combinations
            .iter()
            .filter_map(|counts| {
                let (score, calor) = Self::calculate_score_and_calories(ingredients, counts);
                if calor == calories { Some(score) } else { None }
            })
            .max()
            .unwrap()
    }

    // Return (score, calories)
    fn calculate_score_and_calories(
        ingredients: &[Ingredient],
        counts: &[usize],
    ) -> (usize, usize) {
        // Calculate parts, if part < 0 use 0
        let capacity = ingredients
            .iter()
            .zip(counts)
            .map(|(ingredient, size)| ingredient.capacity * (*size as isize))
            .sum::<isize>()
            .max(0);

        let durability = ingredients
            .iter()
            .zip(counts)
            .map(|(ingredient, size)| ingredient.durability * (*size as isize))
            .sum::<isize>()
            .max(0);

        let flavor = ingredients
            .iter()
            .zip(counts)
            .map(|(ingredient, size)| ingredient.flavor * (*size as isize))
            .sum::<isize>()
            .max(0);

        let texture = ingredients
            .iter()
            .zip(counts)
            .map(|(ingredient, size)| ingredient.texture * (*size as isize))
            .sum::<isize>()
            .max(0);

        let calories = ingredients
            .iter()
            .zip(counts)
            .map(|(ingredient, size)| ingredient.calories * (*size))
            .sum::<usize>()
            .max(0);

        (
            (capacity * durability * flavor * texture) as usize,
            calories,
        )
    }

    fn generate_combinations(len: usize, total: usize) -> Vec<Vec<usize>> {
        let mut current = Vec::new();
        let mut combinations = Vec::new();

        Self::generator(len, total, &mut current, &mut combinations);
        combinations
    }

    fn generator(len: usize, total: usize, current: &mut Vec<usize>, result: &mut Vec<Vec<usize>>) {
        if len == 1 {
            // Append last element to the current and push it to results
            current.push(total);

            // Add record to result
            result.push(current.clone());

            // Remove fromt the current so we can continue if applicable
            current.pop();
        } else {
            // Go through all options
            for i in 0..=total {
                // Add current value
                current.push(i);

                // Recursively add another values
                Self::generator(len - 1, total - i, current, result);

                // Remove current, so we can process next value
                current.pop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use puzzler::puzzler::puzzle::Puzzle;

    use crate::puzzle::{ingredient::Ingredient, solution::Solution};

    fn get_puzzle() -> Solution {
        let mut solution = Solution::new();

        solution
            .parse_input_file()
            .unwrap_or_else(|err| panic!("Failed to parse input file [{err}]"));

        solution
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(get_puzzle().solve_part1().unwrap(), "222870");
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(get_puzzle().solve_part2().unwrap(), "117936");
    }

    fn construct_cookies() -> [Ingredient; 2] {
        let butterscotch = Ingredient {
            capacity: -1,
            durability: -2,
            flavor: 6,
            texture: 3,
            calories: 8,
        };
        let cinnamon = Ingredient {
            capacity: 2,
            durability: 3,
            flavor: -2,
            texture: -1,
            calories: 3,
        };

        [butterscotch, cinnamon]
    }

    #[test]
    fn test_find_best_cookie() {
        let ingredients = construct_cookies();
        let score = Solution::find_best_cookie(&ingredients, 100);
        assert_eq!(score, 62842880);
    }

    #[test]
    fn test_find_best_cookie_calories() {
        let ingredients = construct_cookies();
        let score = Solution::find_best_cookie_calories(&ingredients, 100, 500);
        assert_eq!(score, 57600000);
    }

    #[test]
    fn test_generate_combinations() {
        let combinations = Solution::generate_combinations(2, 100);
        assert_eq!(combinations.len(), 101);

        let combinations = Solution::generate_combinations(3, 100);
        assert_eq!(combinations.len(), 5_151);

        let combinations = Solution::generate_combinations(4, 100);
        assert_eq!(combinations.len(), 176_851);
    }
}
