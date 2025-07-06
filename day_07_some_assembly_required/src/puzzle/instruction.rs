#[derive(Debug, Clone)]
pub enum Instruction {
    Signal(String),
    Not(String),
    And(String, String),
    Or(String, String),
    Lshift(String, String),
    Rshift(String, String),
}
