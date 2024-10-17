use super::{bulb::Bulb, position::Position};

#[derive(Debug)]
pub struct Grid<T>
where
    T: Bulb,
{
    grid: Vec<Vec<T>>,
}

impl<T> Grid<T>
where
    T: Bulb,
{
    pub fn new(grid_x: usize, grid_y: usize) -> Self {
        // Create grid with all bulbs off
        let grid = (0..grid_x)
            .map(|_| (0..grid_y).map(|_| Default::default()).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self { grid }
    }

    pub fn turn_on(&mut self, from: &Position, to: &Position) {
        // from - to is an inclusive range
        for x in from.x..=to.x {
            for y in from.y..=to.y {
                self.grid[x][y].turn_on();
            }
        }
    }

    pub fn turn_off(&mut self, from: &Position, to: &Position) {
        // from - to is an inclusive range
        for x in from.x..=to.x {
            for y in from.y..=to.y {
                self.grid[x][y].turn_off();
            }
        }
    }

    pub fn toggle(&mut self, from: &Position, to: &Position) {
        // from - to is an inclusive range
        for x in from.x..=to.x {
            for y in from.y..=to.y {
                self.grid[x][y].toggle();
            }
        }
    }

    pub fn count_bulbs_on(&self) -> usize {
        self.grid
            .iter()
            .map(|row| row.iter().filter(|bulb| bulb.is_on()).count())
            .sum()
    }

    pub fn get_bulb_count(&self) -> usize {
        self.grid.len() * self.grid[0].len()
    }

    pub fn get_brightness(&self) -> usize {
        self.grid
            .iter()
            .map(|row| row.iter().map(|bulb| bulb.brightness()).sum::<usize>())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::example::{bulb_complex::BulbComplex, bulb_simple::BulbSimple};

    use super::*;

    #[test]
    fn test_turn_on() {
        let mut grid: Grid<BulbSimple> = Grid::new(3, 3);

        // All bulbs are off by default
        assert_eq!(grid.get_bulb_count(), 9);
        assert_eq!(grid.count_bulbs_on(), 0);

        grid.turn_on(&Position { x: 0, y: 0 }, &Position { x: 0, y: 2 });
        assert_eq!(grid.count_bulbs_on(), 3);

        grid.turn_on(&Position { x: 0, y: 0 }, &Position { x: 2, y: 2 });
        assert_eq!(grid.count_bulbs_on(), 9);
    }

    #[test]
    fn test_turn_off() {
        let mut grid: Grid<BulbSimple> = Grid::new(3, 3);

        // All bulbs are off by default
        assert_eq!(grid.get_bulb_count(), 9);
        assert_eq!(grid.count_bulbs_on(), 0);

        grid.turn_on(&Position { x: 0, y: 0 }, &Position { x: 2, y: 2 });
        assert_eq!(grid.count_bulbs_on(), 9);

        grid.turn_off(&Position { x: 2, y: 2 }, &Position { x: 2, y: 2 });
        assert_eq!(grid.count_bulbs_on(), 8);

        grid.turn_off(&Position { x: 2, y: 0 }, &Position { x: 2, y: 2 });
        assert_eq!(grid.count_bulbs_on(), 6);

        grid.turn_off(&Position { x: 1, y: 0 }, &Position { x: 2, y: 2 });
        assert_eq!(grid.count_bulbs_on(), 3);

        grid.turn_off(&Position { x: 0, y: 0 }, &Position { x: 2, y: 2 });
        assert_eq!(grid.count_bulbs_on(), 0);
    }

    #[test]
    fn test_toggle() {
        let mut grid: Grid<BulbSimple> = Grid::new(3, 3);

        // All bulbs are off by default
        assert_eq!(grid.get_bulb_count(), 9);
        assert_eq!(grid.count_bulbs_on(), 0);

        grid.toggle(&Position { x: 0, y: 0 }, &Position { x: 0, y: 0 });
        assert_eq!(grid.count_bulbs_on(), 1);

        grid.toggle(&Position { x: 0, y: 0 }, &Position { x: 0, y: 0 });
        assert_eq!(grid.count_bulbs_on(), 0);

        grid.toggle(&Position { x: 1, y: 0 }, &Position { x: 1, y: 2 });
        assert_eq!(grid.count_bulbs_on(), 3);

        grid.toggle(&Position { x: 0, y: 0 }, &Position { x: 2, y: 2 });
        assert_eq!(grid.count_bulbs_on(), 6);

        grid.toggle(&Position { x: 1, y: 1 }, &Position { x: 1, y: 1 });
        assert_eq!(grid.count_bulbs_on(), 7);
    }

    #[test]
    fn test_bulb_complex_brightness() {
        let mut grid: Grid<BulbComplex> = Grid::new(1000, 1000);

        grid.turn_on(&Position { x: 0, y: 0 }, &Position { x: 0, y: 0 });
        assert_eq!(grid.get_brightness(), 1);

        grid.turn_off(&Position { x: 0, y: 0 }, &Position { x: 0, y: 0 });
        assert_eq!(grid.get_brightness(), 0);

        grid.turn_off(&Position { x: 0, y: 0 }, &Position { x: 0, y: 0 });
        assert_eq!(grid.get_brightness(), 0);

        grid.toggle(&Position { x: 0, y: 0 }, &Position { x: 999, y: 999 });
        assert_eq!(grid.get_brightness(), 2000000);
    }
}
