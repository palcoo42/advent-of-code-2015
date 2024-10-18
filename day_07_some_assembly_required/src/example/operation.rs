#[derive(Clone, Debug, PartialEq)]
pub enum Operation {
    None,
    And,
    Or,
    LeftShift,
    RightShift,
    Not,
}
