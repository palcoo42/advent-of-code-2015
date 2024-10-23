#[derive(Clone, Debug, PartialEq)]
pub struct Letter {
    value: char,
}

#[derive(Debug, PartialEq)]
pub enum Wrap {
    Wrapped,
    NotWrapped,
}

impl Letter {
    pub fn new(c: char) -> Self {
        if !c.is_ascii_lowercase() {
            panic!("Invalid letter character '{}'", c);
        }

        Self { value: c }
    }

    pub fn get(&self) -> char {
        self.value
    }

    pub fn advance_next(&mut self) -> Wrap {
        // If we are at the end of the range we need to overflow
        if self.value == 'z' {
            self.value = 'a';
            Wrap::Wrapped
        } else {
            let numeric = self.value as u8 + 1;
            self.value = numeric as char;
            Wrap::NotWrapped
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advance_next() {
        let mut letter = Letter::new('a');

        assert_eq!(letter.advance_next(), Wrap::NotWrapped);
        assert_eq!(letter.value, 'b');

        assert_eq!(letter.advance_next(), Wrap::NotWrapped);
        assert_eq!(letter.value, 'c');

        let mut letter = Letter::new('x');

        assert_eq!(letter.advance_next(), Wrap::NotWrapped);
        assert_eq!(letter.value, 'y');

        assert_eq!(letter.advance_next(), Wrap::NotWrapped);
        assert_eq!(letter.value, 'z');

        assert_eq!(letter.advance_next(), Wrap::Wrapped);
        assert_eq!(letter.value, 'a');
    }
}
