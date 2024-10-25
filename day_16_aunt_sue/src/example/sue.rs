use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Sue {
    id: u32,
    compounds: HashMap<String, u32>,
}

impl Sue {
    pub fn new(id: u32, compounds: Vec<(String, u32)>) -> Self {
        Self {
            id,
            compounds: compounds.into_iter().collect::<HashMap<_, _>>(),
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn get_compound_value(&self, name: &str) -> Option<u32> {
        self.compounds.get(name).copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &u32)> {
        self.compounds.iter()
    }
}
