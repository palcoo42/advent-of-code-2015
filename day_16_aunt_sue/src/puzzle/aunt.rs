use std::collections::HashMap;

#[derive(Debug)]
pub struct Aunt {
    id: usize,
    compounds: HashMap<String, usize>,
}

impl Aunt {
    pub fn new(id: usize, compounds: Vec<(String, usize)>) -> Self {
        Self {
            id,
            compounds: compounds.into_iter().collect(),
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_compound(&self, name: &str) -> Option<&usize> {
        self.compounds.get(name)
    }
}
