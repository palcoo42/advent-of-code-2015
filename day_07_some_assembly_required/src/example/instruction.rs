use super::{operation::Operation, variable::Variable};

#[derive(Clone, Debug, PartialEq)]
pub struct Instruction {
    pub op: Operation,
    pub arg_1: Variable,
    pub arg_2: Option<Variable>,
    pub result: Variable,
}

impl Instruction {
    pub fn new_none(arg_1: Variable, result: Variable) -> Self {
        Self {
            op: Operation::None,
            arg_1,
            arg_2: None,
            result,
        }
    }

    pub fn new_or(arg_1: Variable, arg_2: Variable, result: Variable) -> Self {
        Self {
            op: Operation::Or,
            arg_1,
            arg_2: Some(arg_2),
            result,
        }
    }

    pub fn new_and(arg_1: Variable, arg_2: Variable, result: Variable) -> Self {
        Self {
            op: Operation::And,
            arg_1,
            arg_2: Some(arg_2),
            result,
        }
    }

    pub fn new_left_shift(arg_1: Variable, arg_2: Variable, result: Variable) -> Self {
        Self {
            op: Operation::LeftShift,
            arg_1,
            arg_2: Some(arg_2),
            result,
        }
    }

    pub fn new_right_shift(arg_1: Variable, arg_2: Variable, result: Variable) -> Self {
        Self {
            op: Operation::RightShift,
            arg_1,
            arg_2: Some(arg_2),
            result,
        }
    }

    pub fn new_not(arg_1: Variable, result: Variable) -> Self {
        Self {
            op: Operation::Not,
            arg_1,
            arg_2: None,
            result,
        }
    }
}
