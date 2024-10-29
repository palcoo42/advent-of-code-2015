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

    pub fn fabricate_molecule_minimum_steps(&self, molecule: &str) -> Option<usize> {
        // To speedup we will go from final molecule to the basic elements
        let mut reversed_replacements = self
            .replacements
            .iter()
            .map(|r| Replacement::new(r.to().to_string(), r.from().to_string()))
            .collect::<Vec<_>>();

        // Sort so we have largest 'from' replacements first to reduce the molecule as much as possible
        reversed_replacements.sort_by_key(|b| std::cmp::Reverse(b.from().len()));

        // Use the greedy strategy - apply largest available transformation
        let mut steps = 0;
        let mut current_molecule = molecule.to_string();

        while current_molecule != "e" {
            let mut reduced = false;

            for replacement in &reversed_replacements {
                // Match the largest and continue
                if let Some(replaced_molecule) = replacement.replace(&current_molecule).first() {
                    current_molecule = replaced_molecule.clone();
                    steps += 1;
                    reduced = true; // Keep going
                    break; // Breaks for loop
                }
            }

            // If there was no transformation we are stuck
            if !reduced {
                eprintln!("Error reducing molecule size");
                return None;
            }
        }

        Some(steps)
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

    #[test]
    fn test_fabricate_molecule_minimum_steps() {
        let machine = Machine::new(vec![
            Replacement::new("e".to_string(), "H".to_string()),
            Replacement::new("e".to_string(), "O".to_string()),
            Replacement::new("H".to_string(), "HO".to_string()),
            Replacement::new("H".to_string(), "OH".to_string()),
            Replacement::new("O".to_string(), "HH".to_string()),
        ]);

        assert_eq!(machine.fabricate_molecule_minimum_steps("HOH"), Some(3));
        // assert_eq!(machine.fabricate_molecule_minimum_steps("HOHOHO"), 6);
    }
}
