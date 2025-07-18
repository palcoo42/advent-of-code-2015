use std::error::Error;
use std::ops::Range;
use std::path::PathBuf;

use itertools::Itertools;
use puzzler::env::project;
use puzzler::parsers::parser::Parser;
use puzzler::puzzler::puzzle::Puzzle;

use crate::puzzle::character::Character;
use crate::puzzle::equipment::Equipment;
use crate::puzzle::item::Item;
use crate::puzzle::shop::Shop;

const PLAYER_HIT_POINT: usize = 100;

pub struct Solution {
    shop: Shop,
    boss: Character,
}

impl Puzzle for Solution {
    fn name(&self) -> &str {
        "--- Day 21: RPG Simulator 20XX ---"
    }

    fn get_input_file_path(&self) -> Option<PathBuf> {
        Some(
            project::get_project_file("../input/day_21.txt")
                .unwrap_or_else(|err| panic!("Failed to fetch file ../input/day_21.txt [{err}]")),
        )
    }

    fn parse_content(&mut self, lines: Vec<String>) -> Result<(), Box<dyn Error>> {
        // Parse Boss stats form the file
        if lines.len() != 3 {
            return Err(format!("Expected 3 lines, found '{}'", lines.len()).into());
        }

        let hit_points = Parser::decode_line_to_unsigned_integer(&lines[0], "Hit Points:")?;
        let damage = Parser::decode_line_to_unsigned_integer(&lines[1], "Damage:")?;
        let armor = Parser::decode_line_to_unsigned_integer(&lines[2], "Armor:")?;

        self.boss = Character {
            hit_points,
            damage,
            armor,
        };

        Ok(())
    }

    fn solve_part1(&mut self) -> Result<String, Box<dyn Error>> {
        let min_gold = self.fight_win_with_minimum_gold();
        Ok(min_gold.to_string())
    }

    fn solve_part2(&mut self) -> Result<String, Box<dyn Error>> {
        let max_gold = self.fight_loose_with_maximum_gold();
        Ok(max_gold.to_string())
    }
}

impl Solution {
    pub fn new() -> Self {
        let weapons = vec![
            Item {
                _name: String::from("Dagger"),
                cost: 8,
                damage: 4,
                armor: 0,
            },
            Item {
                _name: String::from("Shortsword"),
                cost: 10,
                damage: 5,
                armor: 0,
            },
            Item {
                _name: String::from("Warhammer"),
                cost: 25,
                damage: 6,
                armor: 0,
            },
            Item {
                _name: String::from("Longsword"),
                cost: 40,
                damage: 7,
                armor: 0,
            },
            Item {
                _name: String::from("Greataxe"),
                cost: 74,
                damage: 8,
                armor: 0,
            },
        ];

        let armors = vec![
            Item {
                _name: String::from("Leather"),
                cost: 13,
                damage: 0,
                armor: 1,
            },
            Item {
                _name: String::from("Chainmail"),
                cost: 31,
                damage: 0,
                armor: 2,
            },
            Item {
                _name: String::from("Splintmail"),
                cost: 53,
                damage: 0,
                armor: 3,
            },
            Item {
                _name: String::from("Bandedmail"),
                cost: 75,
                damage: 0,
                armor: 4,
            },
            Item {
                _name: String::from("Platemail"),
                cost: 102,
                damage: 0,
                armor: 5,
            },
        ];

        let rings = vec![
            Item {
                _name: String::from("Damage +1"),
                cost: 25,
                damage: 1,
                armor: 0,
            },
            Item {
                _name: String::from("Damage +2"),
                cost: 50,
                damage: 2,
                armor: 0,
            },
            Item {
                _name: String::from("Damage +3"),
                cost: 100,
                damage: 3,
                armor: 0,
            },
            Item {
                _name: String::from("Defense +1"),
                cost: 20,
                damage: 0,
                armor: 1,
            },
            Item {
                _name: String::from("Defense +2"),
                cost: 40,
                damage: 0,
                armor: 2,
            },
            Item {
                _name: String::from("Defense +3"),
                cost: 80,
                damage: 0,
                armor: 3,
            },
        ];

        Self {
            shop: Shop {
                weapons,
                armors,
                rings,
            },
            boss: Character::default(),
        }
    }

    fn player_wins_fight(player: &Character, boss: &Character) -> bool {
        // Repeat until someone is alive
        let mut player = player.clone();
        let mut boss = boss.clone();

        while player.is_alive() && boss.is_alive() {
            // Player's turn
            let damage = std::cmp::max(player.damage.saturating_sub(boss.armor), 1);

            boss.receive_damage(damage);
            if !boss.is_alive() {
                break;
            }

            // Boss's turn
            let damage = std::cmp::max(boss.damage.saturating_sub(player.armor), 1);

            player.receive_damage(damage);
            if !player.is_alive() {
                break;
            }
        }

        player.is_alive()
    }

    fn fight_win_with_minimum_gold(&self) -> usize {
        // Generate all equipments
        let equipment = Self::combine_shop(&self.shop);

        // Keep track of minimum gold spent
        let mut min_gold = usize::MAX;

        // Construct all possible equipments and find minimum gold to still win
        for weapon in &equipment.weapons {
            for armor in &equipment.armors {
                for ring in &equipment.rings {
                    let (player, gold) = Self::create_player(PLAYER_HIT_POINT, weapon, armor, ring);

                    if Self::player_wins_fight(&player, &self.boss) {
                        min_gold = std::cmp::min(min_gold, gold);
                    }
                }
            }
        }

        min_gold
    }

    fn fight_loose_with_maximum_gold(&self) -> usize {
        // Generate all equipments
        let equipment = Self::combine_shop(&self.shop);

        // Keep track of maximum gold spent
        let mut max_gold = usize::MIN;

        // Construct all possible equipments and find maximum gold to still loose
        for weapon in &equipment.weapons {
            for armor in &equipment.armors {
                for ring in &equipment.rings {
                    let (player, gold) = Self::create_player(PLAYER_HIT_POINT, weapon, armor, ring);

                    if !Self::player_wins_fight(&player, &self.boss) {
                        max_gold = std::cmp::max(max_gold, gold);
                    }
                }
            }
        }

        max_gold
    }

    fn combine_shop(shop: &Shop) -> Equipment {
        // 1 weapon
        let weapons = Self::combine_items(&shop.weapons, 1..2);

        // 0 or 1 armor
        let armors = Self::combine_items(&shop.armors, 0..2);

        // 0, 1 or 2 rings
        let rings = Self::combine_items(&shop.rings, 0..3);

        Equipment {
            weapons,
            armors,
            rings,
        }
    }

    fn combine_items(items: &[Item], range: Range<usize>) -> Vec<Vec<&Item>> {
        // Check allowed range
        if range.end > 3 {
            panic!(
                "Range shall be within 0..3, found '{}..={}'",
                range.start, range.end
            );
        }

        let mut combined = Vec::new();

        for r in range {
            let mut combinations = items.iter().combinations(r).collect::<Vec<_>>();
            combined.append(&mut combinations);
        }

        combined
    }

    fn create_player(
        hit_points: usize,
        weapon: &Vec<&Item>,
        armor: &Vec<&Item>,
        ring: &Vec<&Item>,
    ) -> (Character, usize) {
        let player_damage = weapon.iter().map(|item| item.damage).sum::<usize>()
            + armor.iter().map(|item| item.damage).sum::<usize>()
            + ring.iter().map(|item| item.damage).sum::<usize>();

        let player_armor = weapon.iter().map(|item| item.armor).sum::<usize>()
            + armor.iter().map(|item| item.armor).sum::<usize>()
            + ring.iter().map(|item| item.armor).sum::<usize>();

        let gold = weapon.iter().map(|item| item.cost).sum::<usize>()
            + armor.iter().map(|item| item.cost).sum::<usize>()
            + ring.iter().map(|item| item.cost).sum::<usize>();

        (
            Character {
                hit_points,
                damage: player_damage,
                armor: player_armor,
            },
            gold,
        )
    }
}

#[cfg(test)]
mod tests {
    use puzzler::puzzler::puzzle::Puzzle;

    use crate::puzzle::{character::Character, solution::Solution};

    fn get_puzzle() -> Solution {
        let mut solution = Solution::new();

        solution
            .parse_input_file()
            .unwrap_or_else(|err| panic!("Failed to parse input file [{err}]"));

        solution
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(get_puzzle().solve_part1().unwrap(), "121");
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(get_puzzle().solve_part2().unwrap(), "201");
    }

    #[test]
    fn test_player_wins_fight() {
        let player = Character {
            hit_points: 8,
            damage: 5,
            armor: 5,
        };
        let boss = Character {
            hit_points: 12,
            damage: 7,
            armor: 2,
        };

        assert!(Solution::player_wins_fight(&player, &boss));
    }

    #[test]
    fn test_combine_items() {
        let solution = Solution::new();

        assert_eq!(
            Solution::combine_items(&solution.shop.armors, 1..2).len(),
            5
        );

        assert_eq!(
            Solution::combine_items(&solution.shop.weapons, 0..2).len(),
            6
        );

        assert_eq!(
            Solution::combine_items(&solution.shop.rings, 0..3).len(),
            22
        );
    }
}
