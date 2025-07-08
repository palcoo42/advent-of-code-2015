use std::error::Error;

use puzzler::puzzler::puzzle::Puzzle;

pub struct Solution {}

impl Puzzle for Solution {
    fn name(&self) -> &str {
        "--- Day 10: Elves Look, Elves Say ---"
    }

    // Solve first part of the puzzle
    fn solve_part1(&mut self) -> Result<String, Box<dyn Error>> {
        let length = Self::repeat_look_and_say("1321131112", 40);
        Ok(length.to_string())
    }

    // Solve second part of the puzzle
    fn solve_part2(&mut self) -> Result<String, Box<dyn Error>> {
        let length = Self::repeat_look_and_say("1321131112", 50);
        Ok(length.to_string())
    }
}

impl Solution {
    pub fn new() -> Self {
        Self {}
    }

    fn repeat_look_and_say(word: &str, repetitions: usize) -> usize {
        let mut word = word.to_string();

        for _ in 0..repetitions {
            word = Self::look_and_say(&word);
        }

        word.len()
    }

    fn look_and_say(word: &str) -> String {
        let bytes = word.as_bytes();
        let mut new_word = String::with_capacity(word.len() * 2);
        let mut start = 0;

        while start < bytes.len() {
            let current = bytes[start];
            let mut end = start + 1;

            while end < bytes.len() {
                if bytes[end] == current {
                    end += 1;
                } else {
                    break;
                }
            }

            new_word.push_str(&format!("{}{}", end - start, current as char));
            start = end;
        }

        new_word
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
        assert_eq!(get_puzzle().solve_part1().unwrap(), "492982");
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(get_puzzle().solve_part2().unwrap(), "6989950");
    }

    #[test]
    fn test_look_and_say() {
        assert_eq!(Solution::look_and_say("1"), String::from("11"));
        assert_eq!(Solution::look_and_say("11"), String::from("21"));
        assert_eq!(Solution::look_and_say("21"), String::from("1211"));
        assert_eq!(Solution::look_and_say("1211"), String::from("111221"));
        assert_eq!(Solution::look_and_say("111221"), String::from("312211"));
    }
}
