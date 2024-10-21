pub struct Literal {
    value: String,
}

impl Literal {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_owned(),
        }
    }

    pub fn diff(&self) -> usize {
        // Note: code_len > memory_len
        self.code_len() - self.memory_len()
    }

    pub fn diff_encoded(&self) -> usize {
        // Note: encoded_len > code_len
        self.encoded_len() - self.code_len()
    }

    fn code_len(&self) -> usize {
        self.value.len()
    }

    fn memory_len(&self) -> usize {
        // \\ -> represents a single backslash
        // \" -> represents a lone double-quote character
        // \x plus two hexadecimal characters -> represents a single character with that ASCII code
        // self.value
        let data = self.value.chars().collect::<Vec<_>>();
        if data.is_empty() {
            return 0;
        }

        let mut count = 0;
        let mut idx = 1;

        // Skip first and last "
        while idx < data.len() - 1 {
            match data[idx] {
                '\\' => match data[idx + 1] {
                    '\\' | '\"' => {
                        count += 1;
                        idx += 2;
                    }
                    'x' => {
                        count += 1;
                        idx += 4;
                    }
                    e => {
                        panic!("Invalid escape character {}", e);
                    }
                },
                _ => {
                    count += 1;
                    idx += 1;
                }
            }
        }

        count
    }

    fn encoded_len(&self) -> usize {
        let data = self.value.chars().collect::<Vec<_>>();
        let mut encoded = String::with_capacity(2 * data.len());

        encoded.push('"');

        for c in data.iter() {
            let enc = match c {
                '"' => "\\\"",
                '\\' => "\\\\",
                c => &c.to_string(),
            };

            encoded.push_str(enc);
        }

        encoded.push('"');

        encoded.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::example::literal::Literal;

    #[test]
    fn test_diff() {
        assert_eq!(2, Literal::new(r#""""#).diff());
        assert_eq!(2, Literal::new(r#""abc""#).diff());
        assert_eq!(3, Literal::new(r#""aaa\"aaa""#).diff());
        assert_eq!(5, Literal::new(r#""\x27""#).diff());
        assert_eq!(3, Literal::new(r#""daz\\zyyxddpwk""#).diff());
        assert_eq!(4, Literal::new(r#""g\"t\\o""#).diff());
        assert_eq!(4, Literal::new(r#""nxzo\"hf\\xp""#).diff());
    }

    #[test]
    fn test_diff_encoded() {
        assert_eq!(4, Literal::new(r#""""#).diff_encoded());
        assert_eq!(4, Literal::new(r#""abc""#).diff_encoded());
        assert_eq!(6, Literal::new(r#""aaa\"aaa""#).diff_encoded());
        assert_eq!(5, Literal::new(r#""\x27""#).diff_encoded());
        assert_eq!(6, Literal::new(r#""daz\\zyyxddpwk""#).diff_encoded());
        assert_eq!(8, Literal::new(r#""g\"t\\o""#).diff_encoded());
        assert_eq!(8, Literal::new(r#""nxzo\"hf\\xp""#).diff_encoded());
    }
}
