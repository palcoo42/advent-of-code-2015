use std::collections::HashMap;

pub struct Tape {
    tapes: HashMap<String, u32>,
}

impl Tape {
    pub fn new(tapes: Vec<(String, u32)>) -> Self {
        Self {
            tapes: tapes.into_iter().collect::<HashMap<_, _>>(),
        }
    }

    pub fn get(&self, name: &str) -> Option<u32> {
        self.tapes.get(name).copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &u32)> {
        self.tapes.iter()
    }
}
