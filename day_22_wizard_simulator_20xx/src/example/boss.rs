#[derive(Debug, Clone, PartialEq)]
pub struct Boss {
    hit_points: u32,
    damage: u32,
}

impl Boss {
    pub fn new(hit_points: u32, damage: u32) -> Self {
        Self { hit_points, damage }
    }

    #[inline]
    pub fn get_hit_points(&self) -> u32 {
        self.hit_points
    }

    #[inline]
    pub fn get_damage(&self) -> u32 {
        self.damage
    }

    pub fn decrease_hit_points(&mut self, amount: u32) {
        self.hit_points = if amount < self.hit_points {
            self.hit_points - amount
        } else {
            0
        }
    }
}
