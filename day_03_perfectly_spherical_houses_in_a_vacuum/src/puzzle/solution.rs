use std::collections::HashSet;
use std::error::Error;

use puzzler::env::project;
use puzzler::grids::direction::Direction;
use puzzler::grids::point::Point;
use puzzler::puzzler::puzzle::Puzzle;

pub struct Solution {
    directions: Vec<Direction>,
}

impl Puzzle for Solution {
    fn name(&self) -> &str {
        "--- Day 03: Perfectly Spherical Houses in a Vacuum ---"
    }

    fn get_input_file_path(&self) -> Option<std::path::PathBuf> {
        Some(
            project::get_project_file("../input/day_03.txt")
                .unwrap_or_else(|err| panic!("Failed to fetch file ../input/day_03.txt [{}]", err)),
        )
    }

    fn parse_content(&mut self, lines: Vec<String>) -> Result<(), Box<dyn Error>> {
        // All content is on a single line
        if lines.len() != 1 {
            return Err(format!(
                "Exactly one line expected in input file, found {}",
                lines.len()
            )
            .into());
        }

        // Convert to direction
        self.directions = lines[0]
            .bytes()
            .map(|b| Direction::try_from(b).map_err(|_| format!("Invalid direction '{}'", b)))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(())
    }

    // Solve first part of the puzzle
    fn solve_part1(&mut self) -> Result<String, Box<dyn Error>> {
        let visited_houses = Self::deliver_presents_santa(&self.directions);
        Ok(visited_houses.to_string())
    }

    // Solve second part of the puzzle
    fn solve_part2(&mut self) -> Result<String, Box<dyn Error>> {
        let visited_houses = Self::deliver_presents_santa_and_robot(&self.directions);
        Ok(visited_houses.to_string())
    }
}

impl Solution {
    pub fn new() -> Self {
        Self { directions: vec![] }
    }

    fn deliver_presents_santa(directions: &[Direction]) -> usize {
        // As a starting point use (0, 0)
        let mut point = Point::new(0, 0);

        // Use hashset to keep track of visited houses
        let mut houses: HashSet<Point> = HashSet::new();
        houses.insert(point);

        for dir in directions {
            point = point.neighbor(dir);
            houses.insert(point);
        }

        houses.len()
    }

    fn deliver_presents_santa_and_robot(directions: &[Direction]) -> usize {
        // As a starting point use (0, 0)
        let mut point = Point::new(0, 0);
        let mut point_robot = Point::new(0, 0);

        // Use hashset to keep track of visited houses
        let mut houses: HashSet<Point> = HashSet::new();
        houses.insert(point);
        houses.insert(point_robot);

        for dir in directions.chunks(2) {
            point = point.neighbor(&dir[0]);
            houses.insert(point);

            point_robot = point_robot.neighbor(&dir[1]);
            houses.insert(point_robot);
        }

        houses.len()
    }
}

#[cfg(test)]
mod tests {
    use puzzler::{grids::direction::Direction, puzzler::puzzle::Puzzle};

    use crate::puzzle::solution::Solution;

    fn get_puzzle() -> Solution {
        let mut solution = Solution::new();

        solution
            .parse_input_file()
            .unwrap_or_else(|err| panic!("Failed to parse input file [{}]", err));

        solution
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(get_puzzle().solve_part1().unwrap(), "2572");
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(get_puzzle().solve_part2().unwrap(), "2631");
    }

    #[test]
    fn test_deliver_presents_santa() {
        assert_eq!(Solution::deliver_presents_santa(&[Direction::East]), 2);
        assert_eq!(
            Solution::deliver_presents_santa(&[
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West
            ]),
            4
        );
        assert_eq!(
            Solution::deliver_presents_santa(&[
                Direction::North,
                Direction::South,
                Direction::North,
                Direction::South,
                Direction::North,
                Direction::South,
                Direction::North,
                Direction::South,
                Direction::North,
                Direction::South,
            ]),
            2
        );
    }

    #[test]
    fn test_deliver_presents_santa_and_robot() {
        assert_eq!(
            Solution::deliver_presents_santa_and_robot(&[Direction::North, Direction::South]),
            3
        );
        assert_eq!(
            Solution::deliver_presents_santa_and_robot(&[
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West
            ]),
            3
        );
        assert_eq!(
            Solution::deliver_presents_santa_and_robot(&[
                Direction::North,
                Direction::South,
                Direction::North,
                Direction::South,
                Direction::North,
                Direction::South,
                Direction::North,
                Direction::South,
                Direction::North,
                Direction::South,
            ]),
            11
        );
    }
}
