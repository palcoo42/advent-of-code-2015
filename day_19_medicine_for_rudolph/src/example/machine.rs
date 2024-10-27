use std::collections::HashSet;

use super::replacement::Replacement;

pub struct Machine {
    replacements: Vec<Replacement>,
}

impl Machine {
    pub fn new(replacements: Vec<Replacement>) -> Self {
        Self { replacements }
    }

    pub fn get_number_of_distinct_molecules(&self, molecule: &str) -> usize {
        self.replacements
            .iter()
            .flat_map(|repl| repl.replace(molecule))
            .collect::<HashSet<_>>()
            .len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_number_of_distinct_molecules() {
        let machine = Machine::new(vec![
            Replacement::new("H".to_string(), "HO".to_string()),
            Replacement::new("H".to_string(), "OH".to_string()),
            Replacement::new("O".to_string(), "HH".to_string()),
        ]);

        assert_eq!(machine.get_number_of_distinct_molecules("HOH"), 4);
        assert_eq!(machine.get_number_of_distinct_molecules("HOHOHO"), 7);
    }
}
