use std::collections::VecDeque;

use super::{boss::Boss, game::Game, winner::Winner, wizard::Wizard};

pub struct Simulation {
    wizard: Wizard,
    boss: Boss,
}

impl Simulation {
    pub fn new(wizard: Wizard, boss: Boss) -> Self {
        Self { wizard, boss }
    }

    pub fn find_lowest_mana_cost_to_win(&mut self) -> Option<Game> {
        // Start brand new game
        let game = Game::new(self.wizard.clone(), self.boss.clone());

        // Keep track of lowest mana found so far
        let mut min_mana_game: Option<Game> = None;

        // Run BFS algorithm to find lowest mana cost
        let mut queue = VecDeque::new();
        queue.push_back(game);

        while let Some(current_game) = queue.pop_front() {
            // Next possible game states
            let allowed_spells = current_game.get_allowed_spells(self.wizard.get_mana());

            for allowed_spell in allowed_spells {
                // Clone new game so we have new state for the spell
                let mut new_game = current_game.clone();
                let winner = new_game.next_round(allowed_spell);

                // Retrieve spent mana for the current game
                let game_spent_mana = new_game.get_spent_mana();

                match winner {
                    Some(winner) => {
                        if winner == Winner::Wizard {
                            // Update lowest value if applicable
                            match &min_mana_game {
                                Some(min_game) => {
                                    if min_game.get_spent_mana() > game_spent_mana {
                                        min_mana_game = Some(new_game.clone());
                                    }
                                }
                                None => {
                                    min_mana_game = Some(new_game.clone());
                                }
                            }
                        }

                        // We have a winner, continue with the next game
                        continue;
                    }
                    None => {
                        // If game spent mana >= lowest mana skip whole tree as we cannot reach minimum anymore
                        if let Some(ref lowest) = min_mana_game {
                            if game_spent_mana >= lowest.get_spent_mana() {
                                continue;
                            }
                        }

                        // Otherwise append to the queue
                        queue.push_back(new_game);
                    }
                }
            }
        }

        // Return the result
        min_mana_game
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_lowest_mana_cost_to_win() {
        let mut sim = Simulation::new(Wizard::new(10, 250), Boss::new(13, 8));

        assert_eq!(
            sim.find_lowest_mana_cost_to_win().unwrap().get_spent_mana(),
            226
        );
    }
}
