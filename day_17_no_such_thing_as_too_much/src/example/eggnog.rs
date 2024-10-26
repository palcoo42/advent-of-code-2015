use std::cell::RefCell;

pub struct Eggnog {
    buckets: Vec<u32>,
    current_combination: RefCell<Vec<u32>>,
    solutions: RefCell<Vec<Vec<u32>>>,
}

impl Eggnog {
    pub fn new(buckets: Vec<u32>) -> Self {
        Self {
            buckets,
            current_combination: RefCell::new(Vec::new()),
            solutions: RefCell::new(Vec::new()),
        }
    }

    pub fn count(&self, volume: u32) -> usize {
        // Reset members
        self.current_combination.borrow_mut().clear();
        self.solutions.borrow_mut().clear();

        // Investigate
        self.find_combination(volume, 0);

        // Return number of solutions
        self.solutions.borrow().len()
    }

    fn find_combination(&self, volume: u32, index: usize) {
        // If we have a solution add it to the list and return
        let sum = self.current_combination.borrow().iter().sum::<u32>();

        if sum == volume {
            self.solutions
                .borrow_mut()
                .push(self.current_combination.borrow().clone());
            return;
        }

        // Stop of solution is not possible anymore
        if sum > volume {
            return;
        }

        // Go deeper
        for idx in index..self.buckets.len() {
            // Add next item
            self.current_combination
                .borrow_mut()
                .push(self.buckets[idx]);

            // Investigate sub-tree from next index
            self.find_combination(volume, idx + 1);

            // Backtrack
            self.current_combination.borrow_mut().pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Eggnog;

    #[test]
    fn test_count() {
        let eggnog = Eggnog::new(vec![20, 15, 10, 5, 5]);

        assert_eq!(eggnog.count(25), 4);
    }
}
