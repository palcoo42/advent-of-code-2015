#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Variable {
    pub value: String,
}

impl Variable {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_owned(),
        }
    }
}
