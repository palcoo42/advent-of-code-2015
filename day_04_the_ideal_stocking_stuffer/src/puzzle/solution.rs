use std::error::Error;

use puzzler::puzzler::puzzle::Puzzle;

pub struct Solution {}

impl Puzzle for Solution {
    fn name(&self) -> &str {
        "--- Day 04: The Ideal Stocking Stuffer ---"
    }

    // Solve first part of the puzzle
    fn solve_part1(&mut self) -> Result<String, Box<dyn Error>> {
        let number = Solution::find_md5("yzbqklnj", 5);
        Ok(number.to_string())
    }

    // Solve second part of the puzzle
    fn solve_part2(&mut self) -> Result<String, Box<dyn Error>> {
        let number = Solution::find_md5("yzbqklnj", 6);
        Ok(number.to_string())
    }
}

impl Solution {
    pub fn new() -> Self {
        Self {}
    }

    fn find_md5(secret_key: &str, zeros_count: usize) -> usize {
        // Build up leading zeros
        let zeros = "0".repeat(zeros_count);

        // Start with number 0
        let mut number = 0;

        // Loop until we find digest with leading zeros
        loop {
            let data = format!("{}{}", secret_key, number);
            let digest = md5::compute(&data);

            if format!("{:x}", digest).starts_with(&zeros) {
                return number;
            }

            number += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use puzzler::puzzler::puzzle::Puzzle;

    use crate::puzzle::solution::Solution;

    fn get_puzzle() -> Solution {
        Solution::new()
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(get_puzzle().solve_part1().unwrap(), "282749");
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(get_puzzle().solve_part2().unwrap(), "9962624");
    }

    #[test]
    fn test_find_md5() {
        assert_eq!(Solution::find_md5("abcdef", 5), 609043);
        assert_eq!(Solution::find_md5("pqrstuv", 5), 1048970);
    }
}
