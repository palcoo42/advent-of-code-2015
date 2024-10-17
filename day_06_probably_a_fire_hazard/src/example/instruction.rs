use super::{action::Action, position::Position};

#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub action: Action,
    pub from: Position,
    pub to: Position,
}
