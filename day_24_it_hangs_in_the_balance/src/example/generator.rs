use std::{cmp::Ordering, collections::VecDeque};

use super::state::State;

const GROUPS_SIZE: u32 = 3;

#[derive(Default)]
pub struct Generator {
    packages: Vec<u32>,
}

impl Generator {
    pub fn new(packages: Vec<u32>) -> Self {
        let mut sorted = packages;
        sorted.sort_by(|a, b| b.cmp(a));

        Self { packages: sorted }
    }

    pub fn find_min_entanglement(&self) -> u128 {
        // Find group target sum
        let target = self.packages.iter().sum::<u32>() / GROUPS_SIZE;

        let mut min_quantum_entanglement = u128::MAX;
        let mut min_depth = u32::MAX;

        // BFS algorithm
        let mut queue = VecDeque::new();
        queue.push_back(State {
            sum: 0,
            quantum_entanglement: 1,
            depth: 0,
            remaining_weights: &self.packages[..],
        });

        while let Some(state) = queue.pop_front() {
            // Check for a solution
            if state.sum == target {
                match state.depth.cmp(&min_depth) {
                    Ordering::Less => {
                        min_depth = state.depth;
                        min_quantum_entanglement = state.quantum_entanglement;
                    }
                    Ordering::Equal => {
                        min_quantum_entanglement =
                            min_quantum_entanglement.min(state.quantum_entanglement);
                    }
                    Ordering::Greater => {
                        // Not valid solution
                    }
                }
                continue;
            }

            // Invalid solution
            if state.sum > target || state.depth > min_depth {
                continue;
            }

            // Add remaining weights
            for (i, weight) in state.remaining_weights.iter().enumerate() {
                let new_state = State {
                    sum: state.sum + weight,
                    quantum_entanglement: state.quantum_entanglement * *weight as u128,
                    depth: state.depth + 1,
                    remaining_weights: &state.remaining_weights[i + 1..],
                };

                queue.push_back(new_state);
            }
        }

        min_quantum_entanglement
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_triplets() {
        let gen = Generator::new(vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11]);

        assert_eq!(gen.find_min_entanglement(), 99);
    }
}
