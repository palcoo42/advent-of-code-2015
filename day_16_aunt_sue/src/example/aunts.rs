use super::sue::Sue;

#[derive(Debug)]
pub struct Aunts {
    sues: Vec<Sue>,
}

impl Aunts {
    pub fn new(sues: Vec<Sue>) -> Self {
        Self { sues }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Sue> {
        self.sues.iter()
    }
}
