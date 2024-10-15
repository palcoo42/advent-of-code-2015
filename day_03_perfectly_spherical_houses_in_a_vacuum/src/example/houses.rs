use std::collections::HashMap;

use super::{direction::Direction, house::House, instructions::Instructions};

type Visits = u32;

#[derive(Default)]
pub struct Houses {
    houses: HashMap<House, Visits>,
    current_x: i32,
    current_y: i32,
}

impl Houses {
    pub fn new() -> Self {
        Self {
            houses: HashMap::new(),
            current_x: 0,
            current_y: 0,
        }
    }

    pub fn deliver_presents(&mut self, instructions: &Instructions) {
        self.deliver_present_at_current_position();

        // Deliver to other locations
        for direction in instructions.iter() {
            match direction {
                Direction::Left => {
                    self.current_x -= 1;
                }
                Direction::Right => {
                    self.current_x += 1;
                }
                Direction::Down => {
                    self.current_y -= 1;
                }
                Direction::Up => {
                    self.current_y += 1;
                }
            }
            self.deliver_present_at_current_position();
        }
    }

    fn deliver_present_at_current_position(&mut self) {
        let house = House::new(self.current_x, self.current_y);
        let visits = self.houses.entry(house).or_insert(0);

        // Increment visit
        *visits += 1;
    }

    pub fn at_least_one_present(&self) -> usize {
        self.houses.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_at_least_one_present() {
        let houses_array = vec![
            House::new(0, 0),
            House::new(0, -1),
            House::new(1, 0),
            House::new(-1, 1),
        ];

        let mut houses = Houses::new();
        for house in houses_array {
            houses.houses.insert(house, 0);
        }

        // Simulate multiple presents
        *houses
            .houses
            .get_mut(&House::new(0, 0))
            .expect("House (0, 0) not found") += 2;

        *houses
            .houses
            .get_mut(&House::new(1, 0))
            .expect("House (1, 0) not found") += 3;

        *houses
            .houses
            .get_mut(&House::new(-1, 1))
            .expect("House (-1, 1) not found") += 4;

        assert_eq!(houses.at_least_one_present(), 4);
    }

    #[test]
    fn test_deliver_presents() {
        let instructions_array = [
            Instructions::new(vec![Direction::Right]),
            Instructions::new(vec![
                Direction::Up,
                Direction::Right,
                Direction::Down,
                Direction::Left,
            ]),
            Instructions::new(vec![
                Direction::Up,
                Direction::Down,
                Direction::Up,
                Direction::Down,
                Direction::Up,
                Direction::Down,
                Direction::Up,
                Direction::Down,
                Direction::Up,
                Direction::Down,
            ]),
        ];

        let expected_at_least_one_presents_array = [2, 4, 2];

        for (instructions, expected_at_least_one_presents) in instructions_array
            .iter()
            .zip(expected_at_least_one_presents_array)
        {
            let mut houses = Houses::new();
            houses.deliver_presents(instructions);

            assert_eq!(
                houses.at_least_one_present(),
                expected_at_least_one_presents,
                "instructions: {:?}",
                instructions
            );
        }
    }
}
