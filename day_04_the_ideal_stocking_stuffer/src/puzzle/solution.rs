use std::error::Error;
use std::fmt::Write;

use md5::Digest;
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
        // Start with number 0
        let mut number = 0;

        // Reusable buffer to avoid format!() in every loop which allocates a new String
        // Note: Magic number 12 is an estimation of maximum number, i.e., 999_999_999_999
        let mut data = String::with_capacity(secret_key.len() + 12);

        // Loop until we find digest with leading zeros
        loop {
            // Fill in data for MD5
            data.clear();
            data.push_str(secret_key);
            write!(&mut data, "{number}").unwrap();

            // Compute digest
            let digest = md5::compute(&data);

            // Check for a solution
            if Self::all_zeros(&digest, zeros_count) {
                return number;
            }

            number += 1;
        }
    }

    // Compare raw bytes to avoid conversions to String
    fn all_zeros(digest: &Digest, zeros_count: usize) -> bool {
        let bytes = &digest.0;

        // MD5 digest is 16 byte hash, each digit is 4 bits. If zeros_count is 5, we need to
        // check 5 x 4 bits, i.e. 2 bytes and one half byte.
        let target_bytes = zeros_count / 2;
        let extra_half_byte = zeros_count % 2;

        // Check full bytes
        if bytes.iter().take(target_bytes).any(|b| *b != 0) {
            return false;
        }

        // Check possible last half byte
        if extra_half_byte > 0 && bytes[target_bytes] >> 4 != 0 {
            return false;
        }

        true
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
