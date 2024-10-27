use ndarray::Array2;

use super::{bulb::Bulb, light::Light};

pub struct Grid {
    bulbs: ndarray::Array2<Bulb>,
}

impl Grid {
    pub fn new_from_puzzle(rows: usize, cols: usize, bulbs: Vec<Bulb>) -> Self {
        let bulbs = Array2::from_shape_vec((rows, cols), bulbs).unwrap_or_else(|err| {
            panic!("Failed to create 2D array 'lights' with error '{}'", err)
        });

        Self { bulbs }
    }

    pub fn set_lights_stuck(&mut self) {
        // Set bulbs in fours corners as stuck
        let row_max = self.bulbs.nrows() - 1;
        let col_max = self.bulbs.ncols() - 1;

        self.bulbs[[0, 0]].set_stuck();
        self.bulbs[[0, col_max]].set_stuck();
        self.bulbs[[row_max, 0]].set_stuck();
        self.bulbs[[row_max, col_max]].set_stuck();
    }

    pub fn lights_on_count(&self) -> usize {
        self.bulbs
            .iter()
            .filter(|&bulb| bulb.light() == &Light::On)
            .count()
    }

    pub fn steps(&mut self, count: usize) {
        for _ in 0..count {
            self.step();
        }
    }

    fn step(&mut self) {
        // Count current lights for all neighbors
        self.count_neighbor_lights();

        // Apply changes to the grid
        for bulb in self.bulbs.iter_mut() {
            // Examine new state of the light
            let new_light = match bulb.light() {
                Light::Off => match bulb.neighbors_on() {
                    3 => Light::On,
                    _ => Light::Off,
                },
                Light::On => match bulb.neighbors_on() {
                    2 | 3 => Light::On,
                    _ => Light::Off,
                },
            };

            // Update the state
            bulb.set_light(new_light);
        }
    }

    fn count_neighbor_lights(&mut self) {
        // Calculate number of lights which are on on neighbors
        let row_max = self.bulbs.nrows() - 1;
        let col_max = self.bulbs.ncols() - 1;

        for row_idx in 0..=row_max {
            for col_idx in 0..=col_max {
                // Clamp indexes
                let from_x = (row_idx as isize - 1).clamp(0, row_max as isize) as usize;
                let to_x = (row_idx + 1).clamp(0, row_max);
                let from_y = (col_idx as isize - 1).clamp(0, col_max as isize - 1) as usize;
                let to_y = (col_idx + 1).clamp(0, col_max);

                // Get sub-matrix
                let slice = self
                    .bulbs
                    .slice_mut(ndarray::s![from_x..=to_x, from_y..=to_y]);

                // Count neighbor lights on - count all from the slice and decrement ourself if light is on
                let lights_on = slice
                    .iter()
                    .filter(|&bulb| bulb.light() == &Light::On)
                    .count()
                    - (self.bulbs[[row_idx, col_idx]].light() == &Light::On) as usize;

                self.bulbs[[row_idx, col_idx]].set_neighbors_on(lights_on as u32);
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::example::reader::Reader;

    use super::*;

    fn check_lights(grid: &Grid, on: &[(usize, usize)]) {
        for i in 0..grid.bulbs.nrows() {
            for j in 0..grid.bulbs.ncols() {
                // If there is an item in 'on' list expect this index as On, otherwise expect Off
                let expected = on
                    .iter()
                    .find(|(x, y)| i == *x && j == *y)
                    .map(|_| Light::On)
                    .unwrap_or(Light::Off);

                assert_eq!(grid.bulbs[[i, j]].light(), &expected, "[{},{}]", i, j);
            }
        }
    }

    fn create_grid() -> Grid {
        Reader::read_grid_from_text(&vec![
            ".#.#.#".to_string(),
            "...##.".to_string(),
            "#....#".to_string(),
            "..#...".to_string(),
            "#.#..#".to_string(),
            "####..".to_string(),
        ])
        .expect("Failed to create grid from text")
    }

    fn create_grid_stuck() -> Grid {
        Reader::read_grid_from_text(&vec![
            "##.#.#".to_string(),
            "...##.".to_string(),
            "#....#".to_string(),
            "..#...".to_string(),
            "#.#..#".to_string(),
            "####.#".to_string(),
        ])
        .expect("Failed to create grid from text")
    }

    #[test]
    fn test_step() {
        let mut grid = create_grid();

        check_lights(
            &grid,
            &[
                (0, 1),
                (0, 3),
                (0, 5),
                (1, 3),
                (1, 4),
                (2, 0),
                (2, 5),
                (3, 2),
                (4, 0),
                (4, 2),
                (4, 5),
                (5, 0),
                (5, 1),
                (5, 2),
                (5, 3),
            ],
        );

        grid.step();
        check_lights(
            &grid,
            &[
                (0, 2),
                (0, 3),
                (1, 2),
                (1, 3),
                (1, 5),
                (2, 3),
                (2, 4),
                (4, 0),
                (5, 0),
                (5, 2),
                (5, 3),
            ],
        );
    }

    #[test]
    fn test_count_neighbor_lights() {
        let mut grid = create_grid();

        grid.count_neighbor_lights();

        assert_eq!(grid.bulbs[[0, 0]].neighbors_on(), 1);
        assert_eq!(grid.bulbs[[0, 1]].neighbors_on(), 0);
        assert_eq!(grid.bulbs[[0, 5]].neighbors_on(), 1);
        assert_eq!(grid.bulbs[[1, 0]].neighbors_on(), 2);
        assert_eq!(grid.bulbs[[1, 1]].neighbors_on(), 2);
        assert_eq!(grid.bulbs[[1, 5]].neighbors_on(), 3);
        assert_eq!(grid.bulbs[[5, 0]].neighbors_on(), 2);
        assert_eq!(grid.bulbs[[5, 1]].neighbors_on(), 4);
        assert_eq!(grid.bulbs[[5, 5]].neighbors_on(), 1);
    }

    #[test]
    fn test_steps() {
        let mut grid = create_grid();
        grid.steps(4);

        assert_eq!(grid.lights_on_count(), 4);
    }

    #[test]
    fn test_steps_lights_stuck() {
        let mut grid = create_grid_stuck();

        grid.set_lights_stuck();
        grid.steps(5);

        assert_eq!(grid.lights_on_count(), 17);
    }
}
