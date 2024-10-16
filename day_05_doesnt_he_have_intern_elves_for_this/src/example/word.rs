pub struct Word {}

impl Word {
    pub fn is_nice(word: &str) -> bool {
        Word::has_at_least_three_vowels(word)
            && Word::has_at_least_one_letter_twice_in_a_row(word)
            && !Word::has_forbidden_patterns(word)
    }

    fn has_at_least_three_vowels(word: &str) -> bool {
        word.bytes()
            .filter(|c| *c == b'a' || *c == b'e' || *c == b'i' || *c == b'o' || *c == b'u')
            .count()
            >= 3
    }

    fn has_at_least_one_letter_twice_in_a_row(word: &str) -> bool {
        let mut previous = None;

        for c in word.bytes() {
            if let Some(previous) = previous {
                if previous == c {
                    return true;
                }
            }

            previous = Some(c);
        }

        false
    }

    fn has_forbidden_patterns(word: &str) -> bool {
        let patterns = ["ab", "cd", "pq", "xy"];

        for pattern in patterns {
            if word.contains(pattern) {
                return true;
            }
        }

        false
    }

    pub fn is_nicer(word: &str) -> bool {
        Word::contains_a_pair_of_two_letters(word) && Word::contains_repeating_letter(word)
    }

    fn contains_a_pair_of_two_letters(word: &str) -> bool {
        for i in 0..word.len() {
            if i + 2 >= word.len() {
                return false;
            }

            let letters = &word[i..i + 2];
            let rest = &word[i + 2..];

            if rest.contains(letters) {
                return true;
            }
        }

        false
    }

    fn contains_repeating_letter(word: &str) -> bool {
        for i in 0..word.len() {
            if i + 2 >= word.len() {
                return false;
            }
            if word[i..i + 1] == word[i + 2..i + 3] {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_nice() {
        assert!(Word::is_nice("ugknbfddgicrmopn"));
        assert!(Word::is_nice("aaa"));

        assert!(!Word::is_nice("jchzalrnumimnmhp"));
        assert!(!Word::is_nice("haegwjzuvuyypxyu"));
        assert!(!Word::is_nice("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_has_at_least_three_vowels() {
        assert!(Word::has_at_least_three_vowels("aei"));
        assert!(Word::has_at_least_three_vowels("xazegov"));
        assert!(Word::has_at_least_three_vowels("aeiouaeiouaeiou"));

        assert!(!Word::has_at_least_three_vowels("dvszwmarrgswjxmb"));

        assert!(!Word::has_at_least_three_vowels("xyz"));
        assert!(!Word::has_at_least_three_vowels("axyz"));
        assert!(!Word::has_at_least_three_vowels("aexyz"));
    }

    #[test]
    fn test_has_at_least_one_letter_twice_in_a_row() {
        assert!(Word::has_at_least_one_letter_twice_in_a_row("xx"));
        assert!(Word::has_at_least_one_letter_twice_in_a_row("abcdde"));
        assert!(Word::has_at_least_one_letter_twice_in_a_row("aabbccdd"));
        assert!(Word::has_at_least_one_letter_twice_in_a_row("yyy"));

        assert!(!Word::has_at_least_one_letter_twice_in_a_row("a"));
        assert!(!Word::has_at_least_one_letter_twice_in_a_row("ab"));
        assert!(!Word::has_at_least_one_letter_twice_in_a_row("abc"));
        assert!(!Word::has_at_least_one_letter_twice_in_a_row("abcd"));
    }

    #[test]
    fn test_has_forbidden_patterns() {
        assert!(Word::has_forbidden_patterns("ab"));
        assert!(Word::has_forbidden_patterns("cd"));
        assert!(Word::has_forbidden_patterns("pq"));
        assert!(Word::has_forbidden_patterns("xy"));
        assert!(Word::has_forbidden_patterns("paxylc"));

        assert!(!Word::has_forbidden_patterns("a"));
        assert!(!Word::has_forbidden_patterns("aa"));
        assert!(!Word::has_forbidden_patterns("aaa"));
    }

    #[test]
    fn test_contains_a_pair_of_two_letters() {
        assert!(Word::contains_a_pair_of_two_letters("xyxy"));
        assert!(Word::contains_a_pair_of_two_letters("aabcdefgaa"));

        assert!(!Word::contains_a_pair_of_two_letters("aaa"));
        assert!(!Word::contains_a_pair_of_two_letters("baaa"));
    }

    #[test]
    fn test_contains_repeating_letter() {
        assert!(Word::contains_repeating_letter("xyx"));
        assert!(Word::contains_repeating_letter("abcdefeghi"));
        assert!(Word::contains_repeating_letter("aaa"));

        assert!(!Word::contains_repeating_letter("abc"));
        assert!(!Word::contains_repeating_letter("abba"));
    }

    #[test]
    fn test_is_nicer() {
        assert!(Word::is_nicer("qjhvhtzxzqqjkmpb"));
        assert!(Word::is_nicer("xxyxx"));

        assert!(!Word::is_nicer("uurcxstgmygtbstg"));
        assert!(!Word::is_nicer("ieodomkazucvgmuy"));
    }
}
