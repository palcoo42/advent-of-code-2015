use std::collections::VecDeque;

use super::{
    boss::Boss, difficulty::Difficulty, game::Game, winner::Winner, winning_games::WinningGames,
    wizard::Wizard,
};

pub struct Simulation {
    wizard: Wizard,
    boss: Boss,
}

impl Simulation {
    pub fn new(wizard: Wizard, boss: Boss) -> Self {
        Self { wizard, boss }
    }

    pub fn find_lowest_mana_cost_to_win(&mut self, difficulty: Difficulty) -> WinningGames {
        // Start brand new game
        let game = Game::new(self.wizard.clone(), self.boss.clone(), difficulty);

        // Keep track of lowest mana found so far
        let mut winning_games = WinningGames::default();

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
                            // match &min_mana_game {
                            //     Some(min_game) => {
                            //         if min_game.get_spent_mana() > game_spent_mana {
                            //             min_mana_game = Some(new_game.clone());
                            //         }
                            //     }
                            //     None => {
                            //         min_mana_game = Some(new_game.clone());
                            //     }
                            // }

                            // Update status of winning games
                            match &winning_games.get_spent_mana() {
                                Some(current_min_mana) => {
                                    // Do we have a new minimum?
                                    match current_min_mana.cmp(&new_game.get_spent_mana()) {
                                        std::cmp::Ordering::Less => {
                                            // Current minimum is smaller than new game -> nothing to do
                                        }
                                        std::cmp::Ordering::Equal => {
                                            // Same minimum
                                            winning_games.add_game(new_game.clone());
                                        }
                                        std::cmp::Ordering::Greater => {
                                            // Current minimum is larger than new game -> update with new minimum
                                            winning_games.replace_games(new_game.clone());
                                        }
                                    }
                                }
                                None => {
                                    // First winner
                                    winning_games.replace_games(new_game.clone());
                                }
                            }
                        }

                        // We have a winner, continue with the next game
                        continue;
                    }
                    None => {
                        // If game spent mana >= lowest mana skip whole tree as we cannot reach minimum anymore
                        if let Some(ref lowest) = winning_games.get_spent_mana() {
                            if &game_spent_mana >= lowest {
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
        winning_games
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_lowest_mana_cost_to_win_normal_difficulty() {
        let mut sim = Simulation::new(Wizard::new(50, 500), Boss::new(55, 8));

        assert_eq!(
            sim.find_lowest_mana_cost_to_win(Difficulty::Normal)
                .get_spent_mana(),
            Some(953)
        );
    }

    #[test]
    fn test_find_lowest_mana_cost_to_win_hard_difficulty() {
        let mut sim = Simulation::new(Wizard::new(50, 500), Boss::new(55, 8));

        assert_eq!(
            sim.find_lowest_mana_cost_to_win(Difficulty::Hard)
                .get_spent_mana(),
            Some(1289)
        );
    }
}
