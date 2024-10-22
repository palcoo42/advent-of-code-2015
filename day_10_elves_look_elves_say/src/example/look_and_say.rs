pub struct LookAndSay {}

impl LookAndSay {
    pub fn translate_multiple_times(number: &str, repetitions: u32) -> String {
        let mut translated = String::from(number);

        for _ in 0..repetitions {
            translated = Self::translate(&translated);
        }

        translated
    }

    pub fn translate(number: &str) -> String {
        let mut translated = String::with_capacity(number.len() * 2);
        let mut idx = 0;

        while idx < number.len() {
            // Fetch next number
            let c = number.as_bytes()[idx];

            // Count number of occurrences
            let count = &number[idx..]
                .as_bytes()
                .iter()
                .map_while(|next| if c == *next { Some(c) } else { None })
                .count();

            // Get translated number
            translated.push_str(&format!("{}{}", count, c as char));

            // Move to the next number
            idx += count;
        }

        translated
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate() {
        assert_eq!(LookAndSay::translate("1"), "11");
        assert_eq!(LookAndSay::translate("11"), "21");
        assert_eq!(LookAndSay::translate("21"), "1211");
        assert_eq!(LookAndSay::translate("1211"), "111221");
        assert_eq!(LookAndSay::translate("111221"), "312211");
    }
}
