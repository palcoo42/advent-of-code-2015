use std::error::Error;
use std::path::PathBuf;

use puzzler::env::project;
use puzzler::parsers::parser::Parser;
use puzzler::puzzler::puzzle::Puzzle;

use crate::puzzle::boss::Boss;
use crate::puzzle::game::Game;
use crate::puzzle::spell::Spell;
use crate::puzzle::winner::Winner;
use crate::puzzle::wizard::Wizard;

pub struct Solution {
    boss: Boss,
}

impl Puzzle for Solution {
    fn name(&self) -> &str {
        "--- Day 22: Wizard Simulator 20XX ---"
    }

    fn get_input_file_path(&self) -> Option<PathBuf> {
        Some(
            project::get_project_file("../input/day_22.txt")
                .unwrap_or_else(|err| panic!("Failed to fetch file ../input/day_22.txt [{err}]")),
        )
    }

    fn parse_content(&mut self, lines: Vec<String>) -> Result<(), Box<dyn Error>> {
        // Parse Boss stats form the file
        if lines.len() != 2 {
            return Err(format!("Expected 2 lines, found '{}'", lines.len()).into());
        }

        let hit_points = Parser::decode_line_to_unsigned_integer(&lines[0], "Hit Points:")?;
        let damage = Parser::decode_line_to_unsigned_integer(&lines[1], "Damage:")?;

        self.boss = Boss::new(hit_points, damage);
        Ok(())
    }

    fn solve_part1(&mut self) -> Result<String, Box<dyn Error>> {
        let min_mana = self.find_minimum_mana_to_win(false);
        Ok(min_mana.to_string())
    }

    fn solve_part2(&mut self) -> Result<String, Box<dyn Error>> {
        let min_mana = self.find_minimum_mana_to_win(true);
        Ok(min_mana.to_string())
    }
}

impl Solution {
    pub fn new() -> Self {
        Self {
            boss: Boss::default(),
        }
    }

    fn find_minimum_mana_to_win(&self, hard_mode: bool) -> usize {
        let wizard = Wizard::new(50, 500);
        let game = Game::new(wizard, self.boss.clone(), hard_mode);

        Self::bfs(&game)
    }

    fn bfs(game: &Game) -> usize {
        let mut min_mana = usize::MAX;
        // let mut min_history = Vec::new();

        let mut queue = Vec::new();
        queue.push(game.clone());

        while let Some(game) = queue.pop() {
            // Check for end of game
            if let Some(winner) = game.get_winner() {
                if winner == &Winner::Wizard {
                    let spent_mana = game.get_wizard().spent_mana;

                    if spent_mana < min_mana {
                        min_mana = spent_mana;
                        // min_history = game.get_history().clone();
                    }
                }
                continue;
            }

            // Check for non-min solution
            if game.get_wizard().spent_mana >= min_mana {
                continue;
            }

            // Spawn next games
            let spells = game.get_castable_spells();

            for spell in spells {
                let mut new_game = game.clone();
                new_game.one_round(Spell::new(spell));

                queue.push(new_game);
            }
        }

        // println!("{min_history:?}");
        min_mana
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

    // #[test]
    // fn test_solve_part1() {
    //     assert_eq!(get_puzzle().solve_part1().unwrap(), "953");
    // }

    #[test]
    fn test_solve_part2() {
        assert_eq!(get_puzzle().solve_part2().unwrap(), "1289");
    }
}
