#[derive(Debug, Clone, Default)]
pub struct Character {
    pub hit_points: usize,
    pub damage: usize,
    pub armor: usize,
}

impl Character {
    pub fn is_alive(&self) -> bool {
        self.hit_points > 0
    }

    pub fn receive_damage(&mut self, damage: usize) {
        self.hit_points = self.hit_points.saturating_sub(damage);
    }
}
