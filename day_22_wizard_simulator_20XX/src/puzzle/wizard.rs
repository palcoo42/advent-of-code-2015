#[derive(Debug, Clone)]
pub struct Wizard {
    pub hit_points: usize,
    pub mana: usize,
    pub spent_mana: usize,
    pub armor: usize,
}

impl Wizard {
    pub fn new(hit_points: usize, mana: usize) -> Self {
        Self {
            hit_points,
            mana,
            spent_mana: 0,
            armor: 0,
        }
    }

    pub fn heal(&mut self, health: usize) {
        self.hit_points += health;
    }

    pub fn take_damage(&mut self, damage: usize) {
        let damage_taken = std::cmp::max(damage.saturating_sub(self.armor), 1);
        self.hit_points = self.hit_points.saturating_sub(damage_taken)
    }

    pub fn take_damage_no_armor(&mut self, damage: usize) {
        self.hit_points = self.hit_points.saturating_sub(damage)
    }

    pub fn drain_mana(&mut self, mana: usize) {
        self.mana = self.mana.saturating_sub(mana);
        self.spent_mana += mana;
    }

    pub fn receive_armor(&mut self, armor: usize) {
        self.armor += armor;
    }

    pub fn receive_mana(&mut self, mana: usize) {
        self.mana += mana;
    }

    pub fn reset_spell_effects(&mut self) {
        self.armor = 0;
    }
}
