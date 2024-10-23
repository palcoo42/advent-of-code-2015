use std::collections::HashSet;

use once_cell::sync::Lazy;

use super::letter::{Letter, Wrap};

#[derive(Debug, PartialEq)]
pub struct Password {
    current: Vec<Letter>,
}

impl Password {
    pub fn new(pwd: &str) -> Self {
        let password = pwd
            .bytes()
            .map(|c| Letter::new(c as char))
            .collect::<Vec<_>>();

        Self { current: password }
    }

    pub fn is_valid(&self) -> bool {
        let pwd = self.get();

        Self::has_three_consecutive_letters(&pwd)
            && !Self::has_forbidden_letters(&pwd)
            && Self::has_at_least_two_non_overlapping_pairs(&pwd)
    }

    pub fn find_next_valid_password(&self) -> Password {
        // Find next password until we find a valid password
        let mut next_pwd = Self::next_password(self);

        loop {
            if next_pwd.is_valid() {
                return next_pwd;
            }

            next_pwd = Self::next_password(&next_pwd);
        }
    }

    fn next_password(pwd: &Password) -> Password {
        // Create new password as we do not want to alter existing as it may be invalid
        let mut next_pwd = pwd.current.clone();

        // Advance last letter
        let last_letter = next_pwd.iter_mut().last().unwrap_or_else(|| {
            panic!("Cannot access last letter in password");
        });

        let mut wrapped = last_letter.advance_next();

        // Update other letters if needed; skip last one as it is already updated
        for letter in next_pwd.iter_mut().rev().skip(1) {
            // If letter is not wrapped around, there is nothing to do
            if wrapped == Wrap::NotWrapped {
                return Password { current: next_pwd };
            }

            wrapped = letter.advance_next();
        }

        Password { current: next_pwd }
    }

    pub fn get(&self) -> String {
        self.current
            .iter()
            .map(|letter| letter.get())
            .collect::<String>()
    }

    fn has_three_consecutive_letters(pwd: &str) -> bool {
        // Format allowed triplets abc, bcd, ..., xyz
        static ALLOWED: Lazy<HashSet<String>> = Lazy::new(|| {
            // 3 consecutive -> skip last two so we have triplets
            ('a'..='x')
                .map(|c| {
                    let second = char::from_u32((c as u32) + 1)
                        .unwrap_or_else(|| panic!("Invalid second char '{}'", c));
                    let third = char::from_u32((c as u32) + 2)
                        .unwrap_or_else(|| panic!("Invalid third char '{}'", c));
                    format!("{}{}{}", c, second, third)
                })
                .collect::<HashSet<_>>()
        });

        // Triplets - skip last two items
        for idx in 0..pwd.len() - 2 {
            if ALLOWED.contains(&pwd[idx..idx + 3]) {
                return true;
            }
        }

        false
    }

    fn has_forbidden_letters(pwd: &str) -> bool {
        pwd.contains("i") || pwd.contains("o") || pwd.contains("l")
    }

    fn has_at_least_two_non_overlapping_pairs(pwd: &str) -> bool {
        let mut idx = 1;
        let mut pairs = 0;

        while idx < pwd.len() {
            if pwd[idx..=idx] == pwd[idx - 1..=idx - 1] {
                pairs += 1;
                if pairs == 2 {
                    return true;
                }
                idx += 2;
            } else {
                idx += 1;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_password() {
        let expected_passwords = ["xy", "xz", "ya", "yb"];

        let mut password = Password::new("xx");
        assert_eq!(&password.get(), "xx");

        for expected_pwd in expected_passwords {
            password = Password::next_password(&password);
            assert_eq!(password, Password::new(expected_pwd));
        }
    }

    #[test]
    fn test_has_three_consecutive_letters() {
        assert!(Password::has_three_consecutive_letters("abc"));
        assert!(Password::has_three_consecutive_letters("klm"));
        assert!(Password::has_three_consecutive_letters("xyz"));
        assert!(Password::has_three_consecutive_letters("pqrs"));
        assert!(Password::has_three_consecutive_letters("qrstv"));
        assert!(Password::has_three_consecutive_letters("pabcx"));

        assert!(!Password::has_three_consecutive_letters("abd"));
        assert!(!Password::has_three_consecutive_letters("xya"));
        assert!(!Password::has_three_consecutive_letters("pstz"));
    }

    #[test]
    fn test_has_forbidden_letters() {
        assert!(Password::has_forbidden_letters("i"));
        assert!(Password::has_forbidden_letters("o"));
        assert!(Password::has_forbidden_letters("l"));
        assert!(Password::has_forbidden_letters("abicd"));
        assert!(Password::has_forbidden_letters("xyzo"));
        assert!(Password::has_forbidden_letters("pslz"));

        assert!(!Password::has_forbidden_letters("abcdefghjkmnpqrstvuvwyz"));
    }

    #[test]
    fn test_has_at_least_two_non_overlapping_pairs() {
        assert!(Password::has_at_least_two_non_overlapping_pairs("aaaa"));
        assert!(Password::has_at_least_two_non_overlapping_pairs("aabb"));
        assert!(Password::has_at_least_two_non_overlapping_pairs("aaxbb"));
        assert!(Password::has_at_least_two_non_overlapping_pairs("aaaaaa"));
        assert!(Password::has_at_least_two_non_overlapping_pairs("aabbaa"));

        assert!(!Password::has_at_least_two_non_overlapping_pairs("aabcad"));
        assert!(!Password::has_at_least_two_non_overlapping_pairs(
            "prsttvyzm"
        ));
        assert!(!Password::has_at_least_two_non_overlapping_pairs(
            "klmoosdezdertwnlk"
        ));
    }

    #[test]
    fn test_is_valid() {
        assert!(!Password::new("hijklmmn").is_valid());
        assert!(!Password::new("abbceffg").is_valid());
        assert!(!Password::new("abbcegjk").is_valid());

        assert!(Password::new("abcdffaa").is_valid());
        assert!(Password::new("ghjaabcc").is_valid());
    }

    #[test]
    fn test_find_next_valid_password() {
        assert_eq!(
            Password::new("abcdefgh").find_next_valid_password(),
            Password::new("abcdffaa")
        );
    }

    #[test]
    #[ignore] // Long running test
    fn test_find_next_valid_password_long_running() {
        assert_eq!(
            Password::new("ghijklmn").find_next_valid_password(),
            Password::new("ghjaabcc")
        );
    }
}
