use std::error::Error;

use puzzler::env::project;
use puzzler::puzzler::puzzle::Puzzle;

const VOWELS: [u8; 5] = [b'a', b'e', b'i', b'o', b'u'];

pub struct Solution {
    words: Vec<String>,
}

impl Puzzle for Solution {
    fn name(&self) -> &str {
        "--- Day 5: Doesn't He Have Intern-Elves For This? ---"
    }

    fn get_input_file_path(&self) -> Option<std::path::PathBuf> {
        Some(
            project::get_project_file("../input/day_05.txt")
                .unwrap_or_else(|err| panic!("Failed to fetch file ../input/day_05.txt [{}]", err)),
        )
    }

    fn parse_content(&mut self, lines: Vec<String>) -> Result<(), Box<dyn Error>> {
        self.words = lines;
        Ok(())
    }

    fn solve_part1(&mut self) -> Result<String, Box<dyn Error>> {
        let nice_words_count = self.words.iter().filter(|&w| Solution::is_nice(w)).count();
        Ok(nice_words_count.to_string())
    }

    fn solve_part2(&mut self) -> Result<String, Box<dyn Error>> {
        let nice_words_count = self
            .words
            .iter()
            .filter(|&w| Solution::is_nice_advanced(w))
            .count();
        Ok(nice_words_count.to_string())
    }
}

impl Solution {
    pub fn new() -> Self {
        Self { words: vec![] }
    }

    fn is_nice(word: &str) -> bool {
        // At least 3 vowels (aeiou)
        if word.bytes().filter(|c| VOWELS.contains(c)).count() < 3 {
            return false;
        }

        // At least one letter that appears twice in a row
        let found = word.as_bytes().windows(2).filter(|w| w[0] == w[1]).count();
        if found == 0 {
            return false;
        }

        // Does not contain strings: ab, cd, pq, xy
        let forbiden = ["ab", "cd", "pq", "xy"];
        let found = forbiden.iter().filter_map(|pat| word.find(pat)).count();

        found == 0
    }

    fn is_nice_advanced(word: &str) -> bool {
        let mut valid = false;
        let bytes = word.as_bytes();

        // Contains a pair of any two letters that appears at least twice in the string without overlapping
        for i in 0..bytes.len() - 2 {
            let pair = &bytes[i..i + 2];

            if bytes[i + 2..].windows(2).any(|w| pair == w) {
                valid = true;
                break;
            }
        }

        if !valid {
            return false;
        }

        // Contains at least one letter which repeats with exactly one letter between them
        valid = false;

        for i in 0..bytes.len() - 2 {
            if bytes[i] == bytes[i + 2] {
                valid = true;
                break;
            }
        }

        valid
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
            .unwrap_or_else(|err| panic!("Failed to parse input file [{}]", err));

        solution
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(get_puzzle().solve_part1().unwrap(), "255");
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(get_puzzle().solve_part2().unwrap(), "55");
    }

    #[test]
    fn test_is_nice() {
        assert!(Solution::is_nice("ugknbfddgicrmopn"));
        assert!(Solution::is_nice("aaa"));
    }

    #[test]
    fn test_is_naughty() {
        assert!(!Solution::is_nice("jchzalrnumimnmhp"));
        assert!(!Solution::is_nice("haegwjzuvuyypxyu"));
        assert!(!Solution::is_nice("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_is_nice_advanced() {
        assert!(Solution::is_nice_advanced("qjhvhtzxzqqjkmpb"));
        assert!(Solution::is_nice_advanced("xxyxx"));
    }

    #[test]
    fn test_is_naughty_advanced() {
        assert!(!Solution::is_nice_advanced("uurcxstgmygtbstg"));
        assert!(!Solution::is_nice_advanced("ieodomkazucvgmuy"));
    }
}
