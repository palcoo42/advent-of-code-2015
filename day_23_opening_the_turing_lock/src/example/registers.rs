use std::collections::HashMap;

#[derive(Default)]
pub struct Registers {
    values: HashMap<String, u32>,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn get(&self, register: &str) -> Option<&u32> {
        self.values.get(register)
    }

    pub fn set(&mut self, register: &str, value: u32) {
        *self.values.entry(register.to_string()).or_insert(value) = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registers() {
        let mut reg = Registers::new();

        assert_eq!(reg.get("a"), None);
        assert_eq!(reg.get("b"), None);

        reg.set("a", 42);
        assert_eq!(reg.get("a"), Some(&42));
        assert_eq!(reg.get("b"), None);

        reg.set("a", 24);
        assert_eq!(reg.get("a"), Some(&24));
        assert_eq!(reg.get("b"), None);

        reg.set("b", 99);
        assert_eq!(reg.get("a"), Some(&24));
        assert_eq!(reg.get("b"), Some(&99));

        reg.set("a", 1);
        reg.set("b", 2);
        assert_eq!(reg.get("a"), Some(&1));
        assert_eq!(reg.get("b"), Some(&2));
    }
}
