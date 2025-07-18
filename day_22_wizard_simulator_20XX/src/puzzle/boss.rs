#[derive(Debug, Clone, Default)]
pub struct Boss {
    pub hit_points: usize,
    pub damage: usize,
}

impl Boss {
    pub fn new(hit_points: usize, damage: usize) -> Self {
        Self { hit_points, damage }
    }

    pub fn take_damage(&mut self, damage: usize) {
        self.hit_points = self.hit_points.saturating_sub(damage)
    }
}
