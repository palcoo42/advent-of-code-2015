#[derive(Debug, PartialEq)]
pub struct Replacement {
    from: String,
    to: String,
}

impl Replacement {
    pub fn new(from: String, to: String) -> Self {
        Self { from, to }
    }

    pub fn replace(&self, text: &str) -> Vec<String> {
        text.match_indices(&self.from)
            .map(|(idx, _)| {
                let mut replaced = text.to_string();
                replaced.replace_range(idx..idx + self.from.len(), &self.to);
                replaced
            })
            .collect::<Vec<_>>()
    }

    pub fn from(&self) -> &str {
        &self.from
    }

    pub fn to(&self) -> &str {
        &self.to
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace() {
        let repl = Replacement::new("H".to_string(), "HO".to_string());
        assert_eq!(
            repl.replace("HOH"),
            vec!["HOOH".to_string(), "HOHO".to_string()]
        );

        let repl = Replacement::new("H".to_string(), "OH".to_string());
        assert_eq!(
            repl.replace("HOH"),
            vec!["OHOH".to_string(), "HOOH".to_string()]
        );

        let repl = Replacement::new("O".to_string(), "HH".to_string());
        assert_eq!(repl.replace("HOH"), vec!["HHHH".to_string()]);

        let repl = Replacement::new("Ca".to_string(), "CaCa".to_string());
        assert_eq!(
            repl.replace("CaXaCa"),
            vec!["CaCaXaCa".to_string(), "CaXaCaCa".to_string()]
        );
    }
}
