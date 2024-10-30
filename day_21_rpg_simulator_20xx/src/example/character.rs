use super::items::{
    armor::Armor,
    item::{ArmorValue, CostValue, DamageValue, Item},
    ring::Ring,
    weapon::Weapon,
};

pub type HitPoints = u32;

#[derive(Debug, Clone)]
pub struct Character {
    hit_points: HitPoints,
    weapon: Weapon,
    armor: Option<Armor>,
    rings: Option<Vec<Ring>>,
}

impl Character {
    pub fn new(
        hit_points: HitPoints,
        weapon: Weapon,
        armor: Option<Armor>,
        rings: Option<Vec<Ring>>,
    ) -> Self {
        Self {
            hit_points,
            weapon,
            armor,
            rings,
        }
    }

    pub fn get_hit_points(&self) -> HitPoints {
        self.hit_points
    }

    pub fn is_alive(&self) -> bool {
        self.hit_points > 0
    }

    pub fn take_damage(&mut self, damage: DamageValue) {
        self.hit_points = std::cmp::max(0, self.hit_points as i32 - damage as i32) as HitPoints;
    }

    pub fn calc_damage(&self) -> DamageValue {
        let mut damage = self.weapon.damage();

        if let Some(char_armor) = &self.armor {
            damage += char_armor.damage();
        }

        if let Some(char_rings) = &self.rings {
            for ring in char_rings {
                damage += ring.damage();
            }
        }

        damage
    }

    pub fn calc_armor(&self) -> ArmorValue {
        let mut armor = self.weapon.armor();

        if let Some(char_armor) = &self.armor {
            armor += char_armor.armor();
        }

        if let Some(char_rings) = &self.rings {
            for ring in char_rings {
                armor += ring.armor();
            }
        }

        armor
    }

    pub fn calc_cost(&self) -> CostValue {
        let mut cost = self.weapon.cost();

        if let Some(char_armor) = &self.armor {
            cost += char_armor.cost();
        }

        if let Some(char_rings) = &self.rings {
            for ring in char_rings {
                cost += ring.cost();
            }
        }

        cost
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_stats() {
        let player = Character::new(
            100,
            Weapon::new("", 1, 2, 3),
            Some(Armor::new("", 10, 20, 30)),
            Some(vec![
                Ring::new("", 100, 200, 300),
                Ring::new("", 1000, 2000, 3000),
            ]),
        );

        assert_eq!(player.get_hit_points(), 100);
        assert_eq!(player.calc_damage(), 1111);
        assert_eq!(player.calc_armor(), 2222);
        assert_eq!(player.calc_cost(), 3333);
    }
}
