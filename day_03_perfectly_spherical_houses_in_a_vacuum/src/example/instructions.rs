use super::direction::Direction;

#[derive(Debug)]
pub struct Instructions {
    steps: Vec<Direction>,
}

pub struct InstructionsIterator<'a> {
    instructions: &'a Instructions,
    index: usize,
}

impl Instructions {
    pub fn new(steps: Vec<Direction>) -> Self {
        Self { steps }
    }

    pub fn iter(&self) -> InstructionsIterator {
        InstructionsIterator {
            instructions: self,
            index: 0,
        }
    }
}

impl<'a> Iterator for InstructionsIterator<'a> {
    type Item = &'a Direction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.instructions.steps.len() {
            let step = &self.instructions.steps[self.index];
            self.index += 1;
            Some(step)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator() {
        let instr = Instructions {
            steps: vec![
                Direction::Left,
                Direction::Right,
                Direction::Up,
                Direction::Down,
            ],
        };

        let mut iter = instr.iter();

        assert_eq!(iter.next(), Some(&Direction::Left));
        assert_eq!(iter.next(), Some(&Direction::Right));
        assert_eq!(iter.next(), Some(&Direction::Up));
        assert_eq!(iter.next(), Some(&Direction::Down));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
}
