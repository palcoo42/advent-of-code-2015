use super::light::Light;

pub struct Grid {
    x: usize,
    y: usize,
    lights: Vec<Vec<Light>>,
}

impl Grid {
    pub fn new_from_puzzle(puzzle: Vec<String>) -> Self {
        let mut lights: Vec<Vec<Light>> = vec![];

        for line in puzzle {
            let row = line
                .chars()
                .map(|c| {
                    let light = match c {
                        '.' => Light::Off,
                        '#' => Light::On,
                        _ => panic!("Invalid character '{}'", c),
                    };
                    light
                })
                .collect::<Vec<_>>();

            lights.push(row);
        }

        Self {
            x: lights.len(),
            y: lights.first().unwrap().len(),
            lights,
        }
    }

    pub fn lights_on_count(&self) -> usize {
        self.lights
            .iter()
            .map(|row| row.iter().filter(|&light| light == &Light::On).count())
            .sum()
    }

    pub fn steps(&mut self, count: usize) {
        for _ in 0..count {
            self.step();
        }
    }

    fn step(&mut self) {
        // Calculate off/on count in the current grid. Then update all at once.
        let mut counts = Vec::with_capacity(self.x);
        for _ in 0..self.x {
            let mut row = Vec::with_capacity(self.y);
            row.resize(self.y, (0, 0));

            counts.push(row);
        }

        for i in 0..self.x {
            for j in 0..self.y {
                let count = self.count_off_on(i, j);
                *counts.get_mut(i).unwrap().get_mut(j).unwrap() = count;
            }
        }

        // Now apply changes to the grid
        for i in 0..self.x {
            for j in 0..self.y {
                let light = self.lights.get_mut(i).unwrap().get_mut(j).unwrap();
                let (_count_off, count_on) = *counts.get(i).unwrap().get(j).unwrap();

                let new_state = match light {
                    Light::On => {
                        if count_on == 2 || count_on == 3 {
                            Light::On
                        } else {
                            Light::Off
                        }
                    }
                    Light::Off => {
                        if count_on == 3 {
                            Light::On
                        } else {
                            Light::Off
                        }
                    }
                };

                *light = new_state;
            }
        }
    }

    fn count_off_on(&self, x: usize, y: usize) -> (usize, usize) {
        let mut off = 0;
        let mut on = 0;

        let x_from = if x > 0 { x - 1 } else { 0 };
        let x_to = if x < self.x - 1 { x + 1 } else { self.x - 1 };

        let y_from = if y > 0 { y - 1 } else { 0 };
        let y_to = if y < self.y - 1 { y + 1 } else { self.y - 1 };

        for i in x_from..=x_to {
            for j in y_from..=y_to {
                // Skip ourself
                if i == x && j == y {
                    continue;
                }

                let light = self.lights.get(i).unwrap().get(j).unwrap();
                match light {
                    Light::On => {
                        on += 1;
                    }
                    Light::Off => {
                        off += 1;
                    }
                }
            }
        }

        (off, on)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn check_lights(grid: &Grid, on: &[(usize, usize)]) {
        for i in 0..grid.x {
            for j in 0..grid.y {
                // If there is an item in 'on' list expect this index as On, otherwise expect Off
                let expected = on
                    .iter()
                    .find(|(x, y)| i == *x && j == *y)
                    .map(|_| Light::On)
                    .unwrap_or(Light::Off);

                let light = grid.lights.get(i).unwrap().get(j).unwrap();
                assert_eq!(light, &expected, "[{},{}]", i, j);
            }
        }
    }

    fn create_grid() -> Grid {
        Grid::new_from_puzzle(vec![
            ".#.#.#".to_string(),
            "...##.".to_string(),
            "#....#".to_string(),
            "..#...".to_string(),
            "#.#..#".to_string(),
            "####..".to_string(),
        ])
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
    fn test_steps() {
        let mut grid = create_grid();
        grid.steps(4);

        assert_eq!(grid.lights_on_count(), 4);
    }

    #[test]
    fn test_count_off_on() {
        let grid = create_grid();

        assert_eq!(grid.count_off_on(0, 0), (2, 1));
        assert_eq!(grid.count_off_on(0, 1), (5, 0));
        assert_eq!(grid.count_off_on(0, 5), (2, 1));
        assert_eq!(grid.count_off_on(1, 0), (3, 2));
        assert_eq!(grid.count_off_on(1, 1), (6, 2));
        assert_eq!(grid.count_off_on(1, 5), (2, 3));
        assert_eq!(grid.count_off_on(5, 0), (1, 2));
        assert_eq!(grid.count_off_on(5, 1), (1, 4));
        assert_eq!(grid.count_off_on(5, 5), (2, 1));
    }
}
