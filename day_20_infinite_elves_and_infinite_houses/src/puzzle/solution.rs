use std::error::Error;

use divisors_fixed::Divisors;
use puzzler::puzzler::puzzle::Puzzle;

const PRESENTS: usize = 36_000_000;

pub struct Solution {}

impl Puzzle for Solution {
    fn name(&self) -> &str {
        "--- Day 20: Infinite Elves and Infinite Houses ---"
    }

    fn solve_part1(&mut self) -> Result<String, Box<dyn Error>> {
        let house = Self::find_house_number_infinite(PRESENTS);
        Ok(house.to_string())
    }

    fn solve_part2(&mut self) -> Result<String, Box<dyn Error>> {
        let house = Self::find_house_number_finite(PRESENTS);
        Ok(house.to_string())
    }
}

impl Solution {
    pub fn new() -> Self {
        Self {}
    }

    // Generic function is used to distinguish part 1 and part 2 of the puzzle. It is
    // sufficient as only computation of presents per house differs, i.e., the loop and
    // summing algorithms are the same.
    fn find_house_number<F>(presents: usize, func: F) -> usize
    where
        F: Fn(usize) -> usize,
    {
        let mut house = 1;

        loop {
            let count = func(house);

            if count >= presents {
                break;
            }
            house += 1;
        }

        house
    }

    fn count_presents_infinite(house: usize) -> usize {
        // Get all divisors for house
        let elfs = house.divisors();

        // Sum presents from all elfes
        elfs.iter().map(|nr| nr * 10).sum()
    }

    fn find_house_number_infinite(presents: usize) -> usize {
        Self::find_house_number(presents, Self::count_presents_infinite)
    }

    fn count_presents_finite(house: usize) -> usize {
        // Get all divisors for house
        let elfs = house.divisors();

        // Filter only elfs that deliver max to 50 houses
        let elfs = elfs
            .into_iter()
            .filter(|elf| {
                let visits = house / elf;
                visits <= 50
            })
            .collect::<Vec<_>>();

        // Sum presents from all elfes
        elfs.iter().map(|nr| nr * 11).sum()
    }

    fn find_house_number_finite(presents: usize) -> usize {
        Self::find_house_number(presents, Self::count_presents_finite)
    }
}

#[cfg(test)]
mod tests {
    use puzzler::puzzler::puzzle::Puzzle;

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
        assert_eq!(get_puzzle().solve_part1().unwrap(), "831600");
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(get_puzzle().solve_part2().unwrap(), "884520");
    }

    #[test]
    fn test_count_presents_infinite() {
        assert_eq!(Solution::count_presents_infinite(1), 10);
        assert_eq!(Solution::count_presents_infinite(2), 30);
        assert_eq!(Solution::count_presents_infinite(3), 40);
        assert_eq!(Solution::count_presents_infinite(4), 70);
        assert_eq!(Solution::count_presents_infinite(5), 60);
        assert_eq!(Solution::count_presents_infinite(6), 120);
        assert_eq!(Solution::count_presents_infinite(7), 80);
        assert_eq!(Solution::count_presents_infinite(8), 150);
        assert_eq!(Solution::count_presents_infinite(9), 130);
    }

    #[test]
    fn test_find_house_number() {
        assert_eq!(Solution::find_house_number_infinite(120), 6);
        assert_eq!(Solution::find_house_number_infinite(80), 6);
        assert_eq!(Solution::find_house_number_infinite(150), 8);
        assert_eq!(Solution::find_house_number_infinite(130), 8);
    }
}
