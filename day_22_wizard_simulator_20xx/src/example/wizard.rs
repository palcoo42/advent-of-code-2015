#[derive(Debug, Clone, PartialEq)]
pub struct Wizard {
    hit_points: u32,
    mana: u32,
    spent_mana: u32,
}

impl Wizard {
    pub fn new(hit_points: u32, mana: u32) -> Self {
        Self {
            hit_points,
            mana,
            spent_mana: 0,
        }
    }

    #[inline]
    pub fn get_hit_points(&self) -> u32 {
        self.hit_points
    }

    #[inline]
    pub fn get_mana(&self) -> u32 {
        self.mana
    }

    #[inline]
    pub fn get_spent_mana(&self) -> u32 {
        self.spent_mana
    }

    pub fn increase_hit_points(&mut self, hit_points: u32) {
        self.hit_points += hit_points
    }

    pub fn decrease_hit_points(&mut self, hit_points: u32) {
        self.hit_points = if hit_points < self.hit_points {
            self.hit_points - hit_points
        } else {
            0
        }
    }

    pub fn increase_mana(&mut self, mana: u32) {
        self.mana += mana;
    }

    pub fn decrease_mana(&mut self, mana: u32) {
        self.mana = if mana < self.mana {
            self.mana - mana
        } else {
            0
        };

        self.spent_mana += mana;
    }
}
