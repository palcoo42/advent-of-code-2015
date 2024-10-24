use core::panic;
use std::collections::HashMap;

use iter_tools::Itertools;

use super::person::Person;

pub type Happiness = i64;

#[derive(Default)]
pub struct Table {
    mappings: HashMap<Person, HashMap<Person, Happiness>>,
}

impl Table {
    pub fn new() -> Self {
        Self {
            mappings: HashMap::new(),
        }
    }

    pub fn insert(&mut self, person: Person, neighbor: Person, happiness: Happiness) {
        // Insert person into mapping
        let record = self.mappings.entry(person).or_default();

        // Insert neighbor happiness value
        record.entry(neighbor).or_insert(happiness);
    }

    pub fn seat_persons_with_maximum_happiness(&self) -> Happiness {
        // Find all permutations and find highest happiness.
        // This is usable only to small number of persons though.
        let permutations = self
            .mappings
            .keys()
            .permutations(self.mappings.keys().len())
            .collect::<Vec<_>>();

        permutations
            .iter()
            .map(|seats| self.calculate_happiness(seats))
            .max()
            .unwrap()
    }

    fn calculate_happiness(&self, seats: &[&Person]) -> Happiness {
        // Create pairs of neighbors
        let mut pairs = Vec::new();

        for idx in 0..seats.len() {
            let left = if idx == 0 { seats.len() - 1 } else { idx - 1 };
            let right = if idx == seats.len() - 1 { 0 } else { idx + 1 };

            pairs.push(vec![seats[idx], seats[left]]);
            pairs.push(vec![seats[idx], seats[right]]);
        }

        let mut happiness = 0;

        // Calculate happiness
        for pair in pairs {
            // Find person
            let person = self
                .mappings
                .get(pair[0])
                .unwrap_or_else(|| panic!("Failed to find '{:?}' in mapping", pair[0]));

            // Find related neighbor's happiness
            let neighbor_happiness = person
                .get(pair[1])
                .unwrap_or_else(|| panic!("Failed to find '{:?}' in mapping", pair[1]));

            // Update overall happiness
            happiness += neighbor_happiness;
        }

        happiness
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seat_persons_with_maximum_happiness() {
        let mut table = Table::new();
        table.insert(Person::new("Alice"), Person::new("Bob"), 54);
        table.insert(Person::new("Alice"), Person::new("Carol"), -72);
        table.insert(Person::new("Alice"), Person::new("David"), -2);
        table.insert(Person::new("Bob"), Person::new("Alice"), 83);
        table.insert(Person::new("Bob"), Person::new("Carol"), -7);
        table.insert(Person::new("Bob"), Person::new("David"), -63);
        table.insert(Person::new("Carol"), Person::new("Alice"), -62);
        table.insert(Person::new("Carol"), Person::new("Bob"), 60);
        table.insert(Person::new("Carol"), Person::new("David"), 55);
        table.insert(Person::new("David"), Person::new("Alice"), 46);
        table.insert(Person::new("David"), Person::new("Bob"), -7);
        table.insert(Person::new("David"), Person::new("Carol"), 41);

        assert_eq!(table.seat_persons_with_maximum_happiness(), 330);
    }
}
