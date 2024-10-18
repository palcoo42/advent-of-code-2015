use std::collections::{HashMap, VecDeque};

use super::{instructions::Instructions, operation::Operation, variable::Variable};

#[derive(Default)]
pub struct Circuit {
    variables: HashMap<Variable, u16>,
}

impl Circuit {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn get_wire(&self, wire: &str) -> Option<u16> {
        self.get_variable(&Variable::new(wire))
    }

    fn get_variable(&self, key: &Variable) -> Option<u16> {
        match key.value.parse::<u16>() {
            Ok(number) => {
                // Value is already an number
                Some(number)
            }
            Err(_) => {
                // Value is still an variable, try to find its value
                self.variables.get(key).copied()
            }
        }
    }

    pub fn process(&mut self, instructions: &Instructions) {
        let mut pending = instructions.data.iter().cloned().collect::<VecDeque<_>>();

        while let Some(instruction) = pending.pop_back() {
            let result = match &instruction.op {
                Operation::None => self.none(&instruction.arg_1, &instruction.result),
                Operation::Or => self.or(
                    &instruction.arg_1,
                    instruction.arg_2.as_ref().unwrap(),
                    &instruction.result,
                ),
                Operation::And => self.and(
                    &instruction.arg_1,
                    instruction.arg_2.as_ref().unwrap(),
                    &instruction.result,
                ),
                Operation::LeftShift => self.left_shift(
                    &instruction.arg_1,
                    instruction.arg_2.as_ref().unwrap(),
                    &instruction.result,
                ),
                Operation::RightShift => self.right_shift(
                    &instruction.arg_1,
                    instruction.arg_2.as_ref().unwrap(),
                    &instruction.result,
                ),
                Operation::Not => self.not(&instruction.arg_1, &instruction.result),
            };

            if !result {
                pending.push_front(instruction);
            }
        }
    }

    fn write_number(&mut self, key: &Variable, value: u16) {
        self.variables.insert(key.clone(), value);
    }

    fn none(&mut self, a: &Variable, r: &Variable) -> bool {
        if let Some(number) = self.get_variable(a) {
            self.write_number(r, number);
            return true;
        }
        false
    }

    fn or(&mut self, a: &Variable, b: &Variable, r: &Variable) -> bool {
        let a = self.get_variable(a);
        let b = self.get_variable(b);

        match (a, b) {
            (Some(a), Some(b)) => {
                let result = a | b;
                self.write_number(r, result);
                true
            }
            _ => false,
        }
    }

    fn and(&mut self, a: &Variable, b: &Variable, r: &Variable) -> bool {
        let a = self.get_variable(a);
        let b = self.get_variable(b);

        match (a, b) {
            (Some(a), Some(b)) => {
                let result = a & b;
                self.write_number(r, result);
                true
            }
            _ => false,
        }
    }

    fn left_shift(&mut self, a: &Variable, b: &Variable, r: &Variable) -> bool {
        let a = self.get_variable(a);
        let b = self.get_variable(b);

        match (a, b) {
            (Some(a), Some(b)) => {
                let result = a << b;
                self.write_number(r, result);
                true
            }
            _ => false,
        }
    }

    fn right_shift(&mut self, a: &Variable, b: &Variable, r: &Variable) -> bool {
        let a = self.get_variable(a);
        let b = self.get_variable(b);

        match (a, b) {
            (Some(a), Some(b)) => {
                let result = a >> b;
                self.write_number(r, result);
                true
            }
            _ => false,
        }
    }

    fn not(&mut self, a: &Variable, r: &Variable) -> bool {
        if let Some(a) = self.get_variable(a) {
            let result = !a;
            self.write_number(r, result);
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::example::instruction::Instruction;

    use super::*;

    #[test]
    fn test_or() {
        let mut circuit = Circuit::new();
        let instructions = Instructions {
            data: vec![
                Instruction {
                    op: Operation::None,
                    arg_1: Variable::new("123"),
                    arg_2: None,
                    result: Variable::new("x"),
                },
                Instruction {
                    op: Operation::None,
                    arg_1: Variable::new("456"),
                    arg_2: None,
                    result: Variable::new("y"),
                },
                Instruction {
                    op: Operation::Or,
                    arg_1: Variable::new("x"),
                    arg_2: Some(Variable::new("y")),
                    result: Variable::new("e"),
                },
            ],
        };

        circuit.process(&instructions);

        assert_eq!(circuit.get_variable(&Variable::new("x")), Some(123));
        assert_eq!(circuit.get_variable(&Variable::new("y")), Some(456));
        assert_eq!(circuit.get_variable(&Variable::new("e")), Some(507));
    }

    #[test]
    fn test_and() {
        let mut circuit = Circuit::new();
        let instructions = Instructions {
            data: vec![
                Instruction {
                    op: Operation::None,
                    arg_1: Variable::new("123"),
                    arg_2: None,
                    result: Variable::new("x"),
                },
                Instruction {
                    op: Operation::None,
                    arg_1: Variable::new("456"),
                    arg_2: None,
                    result: Variable::new("y"),
                },
                Instruction {
                    op: Operation::And,
                    arg_1: Variable::new("x"),
                    arg_2: Some(Variable::new("y")),
                    result: Variable::new("d"),
                },
            ],
        };

        circuit.process(&instructions);

        assert_eq!(circuit.get_variable(&Variable::new("x")), Some(123));
        assert_eq!(circuit.get_variable(&Variable::new("y")), Some(456));
        assert_eq!(circuit.get_variable(&Variable::new("d")), Some(72));
    }

    #[test]
    fn test_left_shift() {
        let mut circuit = Circuit::new();
        let instructions = Instructions {
            data: vec![
                Instruction {
                    op: Operation::None,
                    arg_1: Variable::new("123"),
                    arg_2: None,
                    result: Variable::new("x"),
                },
                Instruction {
                    op: Operation::LeftShift,
                    arg_1: Variable::new("x"),
                    arg_2: Some(Variable::new("2")),
                    result: Variable::new("s"),
                },
            ],
        };

        circuit.process(&instructions);

        assert_eq!(circuit.get_variable(&Variable::new("x")), Some(123));
        assert_eq!(circuit.get_variable(&Variable::new("s")), Some(492));
    }

    #[test]
    fn test_right_shift() {
        let mut circuit = Circuit::new();
        let instructions = Instructions {
            data: vec![
                Instruction {
                    op: Operation::None,
                    arg_1: Variable::new("123"),
                    arg_2: None,
                    result: Variable::new("x"),
                },
                Instruction {
                    op: Operation::RightShift,
                    arg_1: Variable::new("x"),
                    arg_2: Some(Variable::new("2")),
                    result: Variable::new("s"),
                },
            ],
        };

        circuit.process(&instructions);

        assert_eq!(circuit.get_variable(&Variable::new("x")), Some(123));
        assert_eq!(circuit.get_variable(&Variable::new("s")), Some(30));
    }

    #[test]
    fn test_not() {
        let mut circuit = Circuit::new();
        let instructions = Instructions {
            data: vec![
                Instruction {
                    op: Operation::None,
                    arg_1: Variable::new("123"),
                    arg_2: None,
                    result: Variable::new("x"),
                },
                Instruction {
                    op: Operation::Not,
                    arg_1: Variable::new("x"),
                    arg_2: None,
                    result: Variable::new("n"),
                },
            ],
        };

        circuit.process(&instructions);

        assert_eq!(circuit.get_variable(&Variable::new("x")), Some(123));
        assert_eq!(circuit.get_variable(&Variable::new("n")), Some(65412));
    }

    #[test]
    fn test_circuit() {
        let mut circuit = Circuit::new();
        let instructions = Instructions {
            data: vec![
                Instruction {
                    op: Operation::None,
                    arg_1: Variable::new("123"),
                    arg_2: None,
                    result: Variable::new("x"),
                },
                Instruction {
                    op: Operation::None,
                    arg_1: Variable::new("456"),
                    arg_2: None,
                    result: Variable::new("y"),
                },
                Instruction {
                    op: Operation::And,
                    arg_1: Variable::new("x"),
                    arg_2: Some(Variable::new("y")),
                    result: Variable::new("d"),
                },
                Instruction {
                    op: Operation::Or,
                    arg_1: Variable::new("x"),
                    arg_2: Some(Variable::new("y")),
                    result: Variable::new("e"),
                },
                Instruction {
                    op: Operation::LeftShift,
                    arg_1: Variable::new("x"),
                    arg_2: Some(Variable::new("2")),
                    result: Variable::new("f"),
                },
                Instruction {
                    op: Operation::RightShift,
                    arg_1: Variable::new("y"),
                    arg_2: Some(Variable::new("2")),
                    result: Variable::new("g"),
                },
                Instruction {
                    op: Operation::Not,
                    arg_1: Variable::new("x"),
                    arg_2: None,
                    result: Variable::new("h"),
                },
                Instruction {
                    op: Operation::Not,
                    arg_1: Variable::new("y"),
                    arg_2: None,
                    result: Variable::new("i"),
                },
            ],
        };

        circuit.process(&instructions);

        assert_eq!(circuit.get_variable(&Variable::new("d")), Some(72));
        assert_eq!(circuit.get_variable(&Variable::new("e")), Some(507));
        assert_eq!(circuit.get_variable(&Variable::new("f")), Some(492));
        assert_eq!(circuit.get_variable(&Variable::new("g")), Some(114));
        assert_eq!(circuit.get_variable(&Variable::new("h")), Some(65412));
        assert_eq!(circuit.get_variable(&Variable::new("i")), Some(65079));
        assert_eq!(circuit.get_variable(&Variable::new("x")), Some(123));
        assert_eq!(circuit.get_variable(&Variable::new("y")), Some(456));
    }
}
