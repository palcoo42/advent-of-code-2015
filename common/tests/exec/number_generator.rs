use common::exec::generator::Generator;

// Create number generator
#[derive(Clone)]
pub struct NumberGenerator {
    next: u32,
    max: u32,
}

impl NumberGenerator {
    pub fn new(max: u32) -> Self {
        Self { next: 0, max }
    }
}

impl Generator for NumberGenerator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next + 1;
        if next < self.max {
            self.next = next;
            return Some(next);
        }

        None
    }
}
