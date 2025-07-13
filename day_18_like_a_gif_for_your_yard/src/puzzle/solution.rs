use std::error::Error;
use std::path::PathBuf;

use puzzler::env::project;
use puzzler::grids::direction::Direction;
use puzzler::grids::grid::Grid;
use puzzler::grids::point::Point;
use puzzler::parsers::parser::Parser;
use puzzler::puzzler::puzzle::Puzzle;

const LIGHT_ON: char = '#';
const LIGHT_OFF: char = '.';

pub struct Solution {
    grid: Grid,
}

impl Puzzle for Solution {
    fn name(&self) -> &str {
        "--- Day 18: Like a GIF For Your Yard ---"
    }

    fn get_input_file_path(&self) -> Option<PathBuf> {
        Some(
            project::get_project_file("../input/day_18.txt")
                .unwrap_or_else(|err| panic!("Failed to fetch file ../input/day_18.txt [{err}]")),
        )
    }

    fn parse_content(&mut self, lines: Vec<String>) -> Result<(), Box<dyn Error>> {
        self.grid = Parser::parse_lines_to_grid(lines)?;
        Ok(())
    }

    fn solve_part1(&mut self) -> Result<String, Box<dyn Error>> {
        let grid = Solution::run_steps(&self.grid, 100, false);
        let lights_on = grid.get_value(LIGHT_ON).len();
        Ok(lights_on.to_string())
    }

    fn solve_part2(&mut self) -> Result<String, Box<dyn Error>> {
        let grid = Solution::run_steps(&self.grid, 100, true);
        let lights_on = grid.get_value(LIGHT_ON).len();
        Ok(lights_on.to_string())
    }
}

impl Solution {
    pub fn new() -> Self {
        Self {
            grid: Grid::default(),
        }
    }

    fn run_steps(grid: &Grid, steps: usize, stuck: bool) -> Grid {
        // Next grid will hold new value. We need it because all point shall be investigates
        // simultaneuosly using previous grid.
        let mut next_grid = grid.clone();

        if stuck {
            Self::stuck_corners_light_on(&mut next_grid);
        }

        for _ in 0..steps {
            let current_grid = next_grid.clone();

            next_grid.print();

            for r in 0..current_grid.rows() {
                for c in 0..current_grid.cols() {
                    let point = Point {
                        x: r as isize,
                        y: c as isize,
                    };

                    let neighbors = current_grid.neighbors(&point, &Direction::ALL);

                    let neighbors_on = neighbors
                        .iter()
                        .filter(|(pt, _direction)| current_grid[*pt] == LIGHT_ON)
                        .count();

                    let new_light = match current_grid[point] {
                        '#' => match neighbors_on {
                            2 | 3 => LIGHT_ON,
                            _ => LIGHT_OFF,
                        },
                        '.' => match neighbors_on {
                            3 => LIGHT_ON,
                            _ => LIGHT_OFF,
                        },
                        c => panic!("Invalid character in grid '{c}'"),
                    };

                    next_grid[point] = new_light;
                }
            }

            if stuck {
                Self::stuck_corners_light_on(&mut next_grid);
            }
        }

        println!();
        next_grid.print();
        next_grid
    }

    // All four courners are stuck, i.e. always lighted on
    fn stuck_corners_light_on(grid: &mut Grid) {
        let positions = [
            Point { x: 0, y: 0 },
            Point {
                x: 0,
                y: (grid.cols() - 1) as isize,
            },
            Point {
                x: (grid.rows() - 1) as isize,
                y: 0,
            },
            Point {
                x: (grid.rows() - 1) as isize,
                y: (grid.cols() - 1) as isize,
            },
        ];

        for pos in positions {
            grid[pos] = LIGHT_ON;
        }
    }
}

#[cfg(test)]
mod tests {
    use puzzler::{grids::grid::Grid, parsers::parser::Parser, puzzler::puzzle::Puzzle};

    use crate::puzzle::solution::Solution;

    fn get_puzzle() -> Solution {
        let mut solution = Solution::new();

        solution
            .parse_input_file()
            .unwrap_or_else(|err| panic!("Failed to parse input file [{err}]"));

        solution
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(get_puzzle().solve_part1().unwrap(), "768");
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(get_puzzle().solve_part2().unwrap(), "781");
    }

    fn construct_grid() -> Grid {
        Parser::parse_lines_to_grid_str(&[
            ".#.#.#", "...##.", "#....#", "..#...", "#.#..#", "####..",
        ])
        .unwrap()
    }

    #[test]
    fn test_run_steps_not_stuck() {
        let grid = construct_grid();

        let grid = Solution::run_steps(&grid, 1, false);
        assert!(grid.equals(&["..##..", "..##.#", "...##.", "......", "#.....", "#.##..",]));

        let grid = Solution::run_steps(&grid, 3, false);
        assert!(grid.equals(&["......", "......", "..##..", "..##..", "......", "......",]));
    }

    #[test]
    fn test_run_steps_stuck() {
        let grid = construct_grid();

        let grid = Solution::run_steps(&grid, 1, true);
        assert!(grid.equals(&["#.##.#", "####.#", "...##.", "......", "#...#.", "#.####",]));

        // let grid = Solution::run_steps(&grid, 4, true);
        // assert!(grid.equals(&["##.###", ".##..#", ".##...", ".##...", "#.#...", "##...#",]));
    }
}
