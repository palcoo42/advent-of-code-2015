use std::error::Error;

use puzzler::puzzler::puzzle::Puzzle;

pub struct Solution {}

impl Puzzle for Solution {
    fn name(&self) -> &str {
        "--- Day 11: Corporate Policy ---"
    }

    // Solve first part of the puzzle
    fn solve_part1(&mut self) -> Result<String, Box<dyn Error>> {
        let pwd = Solution::find_next_password("hepxcrrq");
        Ok(pwd)
    }

    // Solve second part of the puzzle
    fn solve_part2(&mut self) -> Result<String, Box<dyn Error>> {
        let pwd = Solution::find_next_password("hepxcrrq");
        let pwd = Solution::find_next_password(&pwd);
        Ok(pwd)
    }
}

impl Solution {
    pub fn new() -> Self {
        Self {}
    }

    fn find_next_password(pwd: &str) -> String {
        let mut next_password = pwd.to_string();

        loop {
            Self::increment_password(&mut next_password);

            if Self::is_password_secure(&next_password) {
                return next_password;
            }
        }
    }

    fn increment_password(pwd: &mut str) {
        let bytes: &mut [u8] = unsafe { pwd.as_bytes_mut() };
        let mut next_index = Some(bytes.len() - 1);

        while let Some(index) = next_index {
            // Increment current character
            let (incremented, overflow) = Self::increment_char(bytes[index]);

            // Update bytes
            bytes[index] = incremented;

            // If overflow is detected continue with previous byte
            next_index = if overflow { Some(index - 1) } else { None };
        }
    }

    // Increment single byte. Returns incremented byte and flag if overflow was detected
    #[inline]
    fn increment_char(b: u8) -> (u8, bool) {
        match b {
            b'z' => (b'a', true),
            b => (b + 1, false),
        }
    }

    fn is_password_secure(pwd: &str) -> bool {
        // Has to contain three consecutive letters
        if !Self::has_consecutive_letters(pwd) {
            return false;
        }

        // Can not contain i o l
        if ['i', 'o', 'l'].iter().filter(|c| pwd.contains(**c)).count() > 0 {
            return false;
        }

        // Has to contain two different non-overlapping pairs aa, bb, zz
        Self::count_non_overlapping_pairs(pwd) == 2
    }

    fn has_consecutive_letters(pwd: &str) -> bool {
        let bytes = pwd.as_bytes();

        for index in 0..bytes.len() - 3 {
            if bytes[index] + 1 == bytes[index + 1] && bytes[index + 1] + 1 == bytes[index + 2] {
                return true;
            }
        }

        false
    }

    fn count_non_overlapping_pairs(pwd: &str) -> usize {
        let bytes = pwd.as_bytes();
        let mut index = 0;
        let mut count = 0;

        while index < bytes.len() - 1 {
            if bytes[index] == bytes[index + 1] {
                count += 1;
                // Skip to next byte to avoid overlap
                index += 1;
            }

            index += 1;
        }

        count
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
        assert_eq!(get_puzzle().solve_part1().unwrap(), "hepxxyzz");
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(get_puzzle().solve_part2().unwrap(), "heqaabcc");
    }

    #[test]
    fn test_increment_password() {
        let mut pwd = String::from("xx");

        Solution::increment_password(&mut pwd);
        assert_eq!(&pwd, "xy");

        Solution::increment_password(&mut pwd);
        assert_eq!(&pwd, "xz");

        Solution::increment_password(&mut pwd);
        assert_eq!(&pwd, "ya");

        Solution::increment_password(&mut pwd);
        assert_eq!(&pwd, "yb");

        pwd = String::from("azzzzzy");

        Solution::increment_password(&mut pwd);
        assert_eq!(&pwd, "azzzzzz");

        Solution::increment_password(&mut pwd);
        assert_eq!(&pwd, "baaaaaa");

        Solution::increment_password(&mut pwd);
        assert_eq!(&pwd, "baaaaab");
    }

    #[test]
    fn test_is_password_secure() {
        assert!(!Solution::is_password_secure("hijklmmn"));
        assert!(!Solution::is_password_secure("abbceffg"));
        assert!(!Solution::is_password_secure("abbcegjk"));
    }

    #[test]
    fn test_has_consecutive_letters() {
        assert!(!Solution::has_consecutive_letters("xaby"));
        assert!(Solution::has_consecutive_letters("xabcy"));
        assert!(Solution::has_consecutive_letters("xabcdy"));
        assert!(Solution::has_consecutive_letters("xabcabcy"));
    }

    #[test]
    fn test_count_non_overlapping_pairs() {
        assert_eq!(Solution::count_non_overlapping_pairs("abcdef"), 0);
        assert_eq!(Solution::count_non_overlapping_pairs("aabcdef"), 1);
        assert_eq!(Solution::count_non_overlapping_pairs("aaabcdef"), 1);
        assert_eq!(Solution::count_non_overlapping_pairs("aabcdeef"), 2);
        assert_eq!(Solution::count_non_overlapping_pairs("aabaaeef"), 3);
        assert_eq!(Solution::count_non_overlapping_pairs("aaaaaeef"), 3);
    }

    #[test]
    fn test_find_next_password() {
        assert_eq!(
            Solution::find_next_password("abcdefgh"),
            String::from("abcdffaa")
        );
        assert_eq!(
            Solution::find_next_password("ghijklmn"),
            String::from("ghjaabcc")
        );
    }
}
