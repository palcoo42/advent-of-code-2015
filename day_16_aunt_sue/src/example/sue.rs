use std::collections::HashMap;

use super::tape::Tape;

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

    pub fn is_sue(&self, tape: &Tape) -> bool {
        self.compounds.iter().all(|(item, count)| {
            let tape_count = tape
                .get(item)
                .unwrap_or_else(|| panic!("Failed to find item '{}' in tape", item));

            *count == tape_count
        })
    }

    pub fn is_real_sue(&self, tape: &Tape) -> bool {
        self.compounds.iter().all(|(item, count)| {
            let tape_count = tape
                .get(item)
                .unwrap_or_else(|| panic!("Failed to find item '{}' in tape", item));

            match item.as_str() {
                "cats" | "trees" => *count > tape_count,
                "pomeranians" | "goldfish" => *count < tape_count,
                _ => *count == tape_count,
            }
        })
    }
}
