use super::{
    character::Character,
    inventory::Inventory,
    items::item::{CostValue, DamageValue},
    winner::Winner,
};

pub struct Simulation {
    player_health: u32,
    boss: Character,
    inventory: Inventory,
}

impl Simulation {
    pub fn new(player_health: u32, boss: Character) -> Self {
        Self {
            player_health,
            boss,
            inventory: Inventory::new(),
        }
    }

    pub fn find_minimal_cost_to_win_battle(&self) -> CostValue {
        let mut players = self
            .inventory
            .build_players_combinations(self.player_health);

        // Filter battles when Player wins and find the one with the minimum cost
        players
            .iter_mut()
            .filter_map(|p| {
                let mut boss = self.boss.clone();

                if Self::battle(p, &mut boss) == Winner::Player {
                    Some(p.calc_cost())
                } else {
                    None
                }
            })
            .min()
            .expect("Failed to find min cost")
    }

    pub fn find_maximal_cost_and_lose_battle(&self) -> CostValue {
        let mut players = self
            .inventory
            .build_players_combinations(self.player_health);

        // Filter battles when Player wins and find the one with the minimum cost
        players
            .iter_mut()
            .filter_map(|p| {
                let mut boss = self.boss.clone();

                if Self::battle(p, &mut boss) == Winner::Boss {
                    Some(p.calc_cost())
                } else {
                    None
                }
            })
            .max()
            .expect("Failed to find max cost")
    }

    fn battle(player: &mut Character, boss: &mut Character) -> Winner {
        // Battle until someone wins
        loop {
            // Player starts first
            let damage = Self::calculate_damage(player, boss);
            boss.take_damage(damage);

            if !boss.is_alive() {
                return Winner::Player;
            }

            // Boss turn
            let damage = Self::calculate_damage(boss, player);
            player.take_damage(damage);

            if !player.is_alive() {
                return Winner::Boss;
            }
        }
    }

    fn calculate_damage(attacker: &Character, defender: &Character) -> DamageValue {
        // Damage is at least one even if defender's armor > attacker's damage
        std::cmp::max(
            1,
            attacker.calc_damage() as i32 - defender.calc_armor() as i32,
        ) as u32
    }
}

#[cfg(test)]
mod tests {

    use crate::example::items::{armor::Armor, ring::Ring, weapon::Weapon};

    use super::*;

    fn create_player() -> Character {
        // Overall stats: damage: 5, armor: 5
        let weapon = Weapon::new("", 2, 2, 0);
        let armor = Armor::new("", 1, 1, 0);
        let rings = vec![Ring::new("", 1, 1, 0), Ring::new("", 1, 1, 0)];

        Character::new(8, weapon, Some(armor), Some(rings))
    }

    fn create_boss() -> Character {
        let weapon = Weapon::new("", 7, 2, 0);

        Character::new(12, weapon, None, None)
    }

    fn create_mega_boss() -> Character {
        let weapon = Weapon::new("", 100, 100, 0);

        Character::new(100, weapon, None, None)
    }

    #[test]
    fn test_battle() {
        let mut player = create_player();
        let mut boss = create_boss();

        assert_eq!(Simulation::battle(&mut player, &mut boss), Winner::Player);
    }

    #[test]
    fn test_calculate_damage() {
        let player = create_player();
        let boss = create_boss();

        assert_eq!(Simulation::calculate_damage(&player, &boss), 3);
        assert_eq!(Simulation::calculate_damage(&boss, &player), 2);

        let boss = create_mega_boss();

        assert_eq!(Simulation::calculate_damage(&player, &boss), 1);
        assert_eq!(Simulation::calculate_damage(&boss, &player), 95);
    }
}
