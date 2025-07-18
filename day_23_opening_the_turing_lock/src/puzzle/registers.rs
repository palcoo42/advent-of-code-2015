use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Registers {
    a: usize,
    b: usize,
}

impl Index<&str> for Registers {
    type Output = usize;

    fn index(&self, index: &str) -> &Self::Output {
        match index {
            "a" => &self.a,
            "b" => &self.b,
            r => panic!("Unsupported register '{r}'"),
        }
    }
}

impl IndexMut<&str> for Registers {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        match index {
            "a" => &mut self.a,
            "b" => &mut self.b,
            r => panic!("Unsupported register '{r}'"),
        }
    }
}

impl Registers {
    pub fn new() -> Self {
        Self { a: 0, b: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registers() {
        let mut registers = Registers::new();

        assert_eq!(registers["a"], 0);
        assert_eq!(registers["b"], 0);

        registers["a"] = 42;
        registers["b"] = 24;

        assert_eq!(registers["a"], 42);
        assert_eq!(registers["b"], 24);
    }
}
