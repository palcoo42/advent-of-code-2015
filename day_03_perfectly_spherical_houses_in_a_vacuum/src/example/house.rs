use std::hash::Hash;

#[derive(Clone, PartialOrd)]
pub struct House {
    x: i32,
    y: i32,
}

impl PartialEq for House {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for House {}

impl Hash for House {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl House {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
