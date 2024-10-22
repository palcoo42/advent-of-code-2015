#[derive(Debug, Eq, Hash, PartialEq)]
pub struct City {
    pub name: String,
}

impl City {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}
