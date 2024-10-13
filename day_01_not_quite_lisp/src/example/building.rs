pub struct Building {
    floors: String,
}

impl Building {
    pub fn new(floors: &str) -> Self {
        Self {
            floors: floors.to_owned(),
        }
    }

    pub fn count(&self) -> i32 {
        let mut count: i32 = 0;

        self.floors.bytes().for_each(|c| {
            if c == b'(' {
                count += 1;
            } else if c == b')' {
                count -= 1;
            }
        });

        count
    }

    pub fn find_entry(&self, floor: i32) -> Option<usize> {
        let mut position: i32 = 0;

        for (idx, c) in self.floors.bytes().enumerate() {
            if c == b'(' {
                position += 1;
            } else if c == b')' {
                position -= 1;
            }

            if position == floor {
                return Some(idx + 1);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_count() {
        assert_eq!(Building::new("(())").count(), 0);
        assert_eq!(Building::new("()()").count(), 0);

        assert_eq!(Building::new("(((").count(), 3);
        assert_eq!(Building::new("(()(()(").count(), 3);
        assert_eq!(Building::new("))(((((").count(), 3);

        assert_eq!(Building::new("())").count(), -1);
        assert_eq!(Building::new("))(").count(), -1);

        assert_eq!(Building::new(")))").count(), -3);
        assert_eq!(Building::new(")())())").count(), -3);
    }

    #[test]
    fn test_find_entry() {
        assert_eq!(Building::new(")").find_entry(-1), Some(1));
        assert_eq!(Building::new("()())").find_entry(-1), Some(5));
    }
}
