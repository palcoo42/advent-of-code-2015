use std::collections::{HashMap, hash_map::Iter};

#[derive(Default)]
pub struct Recipes {
    recipes: HashMap<String, Vec<String>>,
}

impl Recipes {
    pub fn new() -> Self {
        Self {
            recipes: HashMap::new(),
        }
    }

    pub fn insert(&mut self, molecule: &str, replacements: &[&str]) {
        // Convert replacements to String
        let mut replacements = replacements
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        let element = self.recipes.entry(molecule.to_string()).or_default();
        element.append(&mut replacements);
    }

    pub fn iter(&self) -> Iter<'_, String, Vec<String>> {
        self.recipes.iter()
    }

    pub fn reverse(&self) -> Self {
        let mut reversed = HashMap::new();

        for (key, values) in self.recipes.iter() {
            for value in values {
                // The same value needs to have only one key. We can safely overwrite with
                // only one value stored in the new values.
                reversed.insert(value.to_string(), vec![key.to_string()]);
            }
        }

        Self { recipes: reversed }
    }

    pub fn get_keys_descending_len(&self) -> Vec<&String> {
        let mut keys = self.recipes.keys().collect::<Vec<_>>();
        keys.sort_by_key(|key| key.len());
        keys.reverse();
        keys
    }

    pub fn get(&self, key: &String) -> Option<&Vec<String>> {
        self.recipes.get(key)
    }
}
