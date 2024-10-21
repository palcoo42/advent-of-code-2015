use super::literal::Literal;

pub struct Literals {
    pub data: Vec<Literal>,
}

impl Literals {
    pub fn new(literals: Vec<Literal>) -> Self {
        Self { data: literals }
    }

    pub fn diff(&self) -> usize {
        self.data.iter().map(|literal| literal.diff()).sum()
    }
}

#[cfg(test)]
mod tests {
    use common::env::environment::get_project_root;

    use crate::example::{literals::Literals, reader::Reader};

    fn read_input_from_file(file_name: &str) -> Literals {
        let input_file = get_project_root().join("resources").join(file_name);

        Reader::read_literals(&input_file).unwrap_or_else(|err| {
            panic!(
                "Failed to read literal from input file '{:?}' with error '{}'",
                input_file, err
            );
        })
    }

    #[test]
    fn test_diff_simple() {
        let literals = read_input_from_file("input_sample.txt");
        assert_eq!(literals.diff(), 252);
    }
}
