use std::iter::once;

use iter_tools::Itertools;

use super::{
    character::{Character, HitPoints},
    items::{armor::Armor, ring::Ring, weapon::Weapon},
};

#[derive(Default)]
pub struct Inventory {
    weapons: Vec<Weapon>,
    armors: Vec<Armor>,
    rings: Vec<Ring>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            weapons: Self::create_weapons(),
            armors: Self::create_armors(),
            rings: Self::create_rings(),
        }
    }

    fn create_weapons() -> Vec<Weapon> {
        let dagger = Weapon::new("Dagger", 4, 0, 8);
        let short_sword = Weapon::new("Shortsword", 5, 0, 10);
        let war_hammer = Weapon::new("Warhammer", 6, 0, 25);
        let long_sword = Weapon::new("Longsword", 7, 0, 40);
        let great_axe = Weapon::new("Greataxe", 8, 0, 74);

        vec![dagger, short_sword, war_hammer, long_sword, great_axe]
    }

    fn create_armors() -> Vec<Armor> {
        let leather = Armor::new("Leather", 0, 1, 13);
        let chain_mail = Armor::new("Chainmail", 0, 2, 31);
        let splint_mail = Armor::new("Splitmail", 0, 3, 53);
        let banded_mail = Armor::new("Bandedmail", 0, 4, 75);
        let plate_mail = Armor::new("Platemail", 0, 5, 102);

        vec![leather, chain_mail, splint_mail, banded_mail, plate_mail]
    }

    fn create_rings() -> Vec<Ring> {
        let damage_1 = Ring::new("Damage +1", 1, 0, 25);
        let damage_2 = Ring::new("Damage +2", 2, 0, 50);
        let damage_3 = Ring::new("Damage +3", 3, 0, 100);
        let defense_1 = Ring::new("Defense +1", 0, 1, 20);
        let defense_2 = Ring::new("Defense +2", 0, 2, 40);
        let defense_3 = Ring::new("Defense +3", 0, 3, 80);

        vec![
            damage_1, damage_2, damage_3, defense_1, defense_2, defense_3,
        ]
    }

    pub fn build_players_combinations(&self, hit_points: HitPoints) -> Vec<Character> {
        // Create all permutations
        // Note: There is 1 weapon, 0-1 armor, 0-2 rings
        let mut players = Vec::new();

        for weapon in &self.weapons {
            for armor in self.armors.iter().cloned().map(Some).chain(once(None)) {
                for ring in (0..=2).flat_map(|n| {
                    self.rings
                        .iter()
                        .cloned()
                        .combinations(n)
                        .map(Some)
                        .chain(None)
                }) {
                    let player =
                        Character::new(hit_points, weapon.clone(), armor.clone(), ring.clone());
                    players.push(player);
                }
            }
        }

        players
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_players_combinations() {
        let inv = Inventory::new();

        assert_eq!(inv.build_players_combinations(100).len(), 660);
    }
}
