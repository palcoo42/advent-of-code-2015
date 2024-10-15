use std::collections::HashMap;

use super::{house::House, instructions::Instructions, santa::Santa};

type Visits = u32;

#[derive(Default)]
pub struct Houses {
    houses: HashMap<House, Visits>,
    santa: Santa,
    robo_santa: Santa,
}

impl Houses {
    pub fn new() -> Self {
        Self {
            houses: HashMap::new(),
            santa: Santa { x: 0, y: 0 },
            robo_santa: Santa { x: 0, y: 0 },
        }
    }

    pub fn deliver_presents(&mut self, instructions: &Instructions) {
        // Deliver to the start location
        self.deliver_present_at_position(House::new(self.santa.x, self.santa.y));

        // Deliver presents
        for direction in instructions.iter() {
            self.santa.move_to(direction);
            self.deliver_present_at_position(House::new(self.santa.x, self.santa.y));
        }
    }

    fn deliver_present_at_position(&mut self, house: House) {
        let visits = self.houses.entry(house).or_insert(0);

        // Increment visit
        *visits += 1;
    }

    pub fn at_least_one_present(&self) -> usize {
        self.houses.len()
    }

    pub fn deliver_presents_robo_santa(&mut self, instructions: &Instructions) {
        // Deliver present for two Santa's
        self.deliver_present_at_position(House::new(self.santa.x, self.santa.y));
        self.deliver_present_at_position(House::new(self.robo_santa.x, self.robo_santa.y));

        // Track splitting of instructions to Santa and Robo-Santa
        let mut count = 0;

        // Deliver to other locations
        for direction in instructions.iter() {
            count += 1;

            if count % 2 == 1 {
                self.santa.move_to(direction);
                self.deliver_present_at_position(House::new(self.santa.x, self.santa.y));
            } else {
                self.robo_santa.move_to(direction);
                self.deliver_present_at_position(House::new(self.robo_santa.x, self.robo_santa.y));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::example::direction::Direction;

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

    #[test]
    fn test_deliver_presents_robo_santa() {
        let instructions_array = [
            Instructions::new(vec![Direction::Up, Direction::Down]),
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

        let expected_at_least_one_presents_array = [3, 3, 11];

        for (instructions, expected_at_least_one_presents) in instructions_array
            .iter()
            .zip(expected_at_least_one_presents_array)
        {
            let mut houses = Houses::new();
            houses.deliver_presents_robo_santa(instructions);

            assert_eq!(
                houses.at_least_one_present(),
                expected_at_least_one_presents,
                "instructions: {:?}",
                instructions
            );
        }
    }
}
