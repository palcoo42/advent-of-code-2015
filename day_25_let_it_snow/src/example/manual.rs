use super::coordinates::Coordinates;

pub struct Manual {}

impl Manual {
    pub fn get_code(coord: Coordinates) -> u64 {
        let mut previous = 20151125_u64;
        let mut row = 1;
        let mut col = 1;

        // Move diagonally, from left to right, from bottom to top
        while !(row == coord.row && col == coord.col) {
            if row == 1 {
                // If we are at the first row again move to the to the first column
                // and next row
                row = col + 1;
                col = 1;
            } else {
                // Move up one row and column to the right
                row -= 1;
                col += 1;
            }
            previous = Self::calc_code(previous);
        }

        previous
    }

    #[inline]
    fn calc_code(previous: u64) -> u64 {
        const MULTIPLIER: u64 = 252533;
        const DIVIDER: u64 = 33554393;

        previous * MULTIPLIER % DIVIDER
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_code() {
        assert_eq!(Manual::get_code(Coordinates { row: 1, col: 1 }), 20151125);
        assert_eq!(Manual::get_code(Coordinates { row: 2, col: 1 }), 31916031);
        assert_eq!(Manual::get_code(Coordinates { row: 1, col: 2 }), 18749137);
        assert_eq!(Manual::get_code(Coordinates { row: 6, col: 1 }), 33071741);
        assert_eq!(Manual::get_code(Coordinates { row: 5, col: 2 }), 17552253);
        assert_eq!(Manual::get_code(Coordinates { row: 4, col: 3 }), 21345942);
        assert_eq!(Manual::get_code(Coordinates { row: 3, col: 4 }), 7981243);
        assert_eq!(Manual::get_code(Coordinates { row: 2, col: 5 }), 15514188);
        assert_eq!(Manual::get_code(Coordinates { row: 1, col: 6 }), 33511524);
    }
}
