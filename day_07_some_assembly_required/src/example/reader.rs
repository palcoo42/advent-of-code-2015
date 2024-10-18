use std::path::Path;

use common::reader::{text_reader::TextReader, text_reader_error::TextReaderError};
use once_cell::sync::Lazy;
use regex::Regex;

use crate::example::variable::Variable;

use super::{instruction::Instruction, instructions::Instructions};

pub struct Reader {}

impl Reader {
    pub fn read_instructions(path: &Path) -> Result<Instructions, TextReaderError> {
        let reader = TextReader::new(path.to_path_buf());
        let lines = reader.read_lines(400)?;

        let mut instructions = Vec::new();
        for line in lines {
            let instruction = Self::decode_instruction(&line)?;
            instructions.push(instruction);
        }

        Ok(Instructions::new(instructions))
    }

    fn decode_instruction(line: &str) -> Result<Instruction, TextReaderError> {
        // Order of regex is important, from most complex to simplest
        if let Some(instruction) = Self::try_right_shift(line) {
            return Ok(instruction);
        }

        if let Some(instruction) = Self::try_left_shift(line) {
            return Ok(instruction);
        }

        if let Some(instruction) = Self::try_and(line) {
            return Ok(instruction);
        }

        if let Some(instruction) = Self::try_or(line) {
            return Ok(instruction);
        }

        if let Some(instruction) = Self::try_not(line) {
            return Ok(instruction);
        }

        if let Some(instruction) = Self::try_none(line) {
            return Ok(instruction);
        }

        Err(TextReaderError::GenericError(format!(
            "Failed to decode instruction '{}'",
            line
        )))
    }

    fn try_right_shift(line: &str) -> Option<Instruction> {
        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"(\w+)\s+RSHIFT\s+(\w+)\s+->\s+(\w+)")
                .expect("Failed to create RSHIFT regex")
        });

        if let Some(captures) = RE.captures(line) {
            let arg_1 = Variable::new(&captures[1]);
            let arg_2 = Variable::new(&captures[2]);
            let result = Variable::new(&captures[3]);

            return Some(Instruction::new_right_shift(arg_1, arg_2, result));
        }

        None
    }

    fn try_left_shift(line: &str) -> Option<Instruction> {
        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"(\w+)\s+LSHIFT\s+(\w+)\s+->\s+(\w+)")
                .expect("Failed to create LSHIFT regex")
        });

        if let Some(captures) = RE.captures(line) {
            let arg_1 = Variable::new(&captures[1]);
            let arg_2 = Variable::new(&captures[2]);
            let result = Variable::new(&captures[3]);

            return Some(Instruction::new_left_shift(arg_1, arg_2, result));
        }

        None
    }

    fn try_not(line: &str) -> Option<Instruction> {
        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"NOT\s+(\w+)\s+->\s+(\w+)").expect("Failed to create NOT regex")
        });

        if let Some(captures) = RE.captures(line) {
            let arg_1 = Variable::new(&captures[1]);
            let result = Variable::new(&captures[2]);

            return Some(Instruction::new_not(arg_1, result));
        }

        None
    }

    fn try_none(line: &str) -> Option<Instruction> {
        static RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"(\w+)\s+->\s+(\w+)").expect("Failed to create NONE regex"));

        if let Some(captures) = RE.captures(line) {
            let arg_1 = Variable::new(&captures[1]);
            let result = Variable::new(&captures[2]);

            return Some(Instruction::new_none(arg_1, result));
        }

        None
    }

    fn try_and(line: &str) -> Option<Instruction> {
        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"(\w+)\s+AND\s+(\w+)\s+->\s+(\w+)").expect("Failed to create AND regex")
        });

        if let Some(captures) = RE.captures(line) {
            let arg_1 = Variable::new(&captures[1]);
            let arg_2 = Variable::new(&captures[2]);
            let result = Variable::new(&captures[3]);

            return Some(Instruction::new_and(arg_1, arg_2, result));
        }

        None
    }

    fn try_or(line: &str) -> Option<Instruction> {
        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"(\w+)\s+OR\s+(\w+)\s+->\s+(\w+)").expect("Failed to create OR regex")
        });

        if let Some(captures) = RE.captures(line) {
            let arg_1 = Variable::new(&captures[1]);
            let arg_2 = Variable::new(&captures[2]);
            let result = Variable::new(&captures[3]);

            return Some(Instruction::new_or(arg_1, arg_2, result));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::example::{operation::Operation, variable::Variable};

    use super::*;

    #[test]
    fn test_try_right_shift() {
        assert_eq!(
            Reader::try_right_shift("bn RSHIFT 2 -> bo").unwrap(),
            Instruction {
                op: Operation::RightShift,
                arg_1: Variable::new("bn"),
                arg_2: Some(Variable::new("2")),
                result: Variable::new("bo")
            }
        );

        assert!(Reader::try_right_shift("uups").is_none());
    }

    #[test]
    fn test_try_left_shift() {
        assert_eq!(
            Reader::try_left_shift("lc LSHIFT 1 -> lw").unwrap(),
            Instruction {
                op: Operation::LeftShift,
                arg_1: Variable::new("lc"),
                arg_2: Some(Variable::new("1")),
                result: Variable::new("lw")
            }
        );

        assert!(Reader::try_left_shift("uups").is_none());
    }

    #[test]
    fn test_try_not() {
        assert_eq!(
            Reader::try_not("NOT lo -> lp").unwrap(),
            Instruction {
                op: Operation::Not,
                arg_1: Variable::new("lo"),
                arg_2: None,
                result: Variable::new("lp")
            }
        );

        assert!(Reader::try_not("uups").is_none());
    }

    #[test]
    fn test_try_none() {
        assert_eq!(
            Reader::try_none("lx -> a").unwrap(),
            Instruction {
                op: Operation::None,
                arg_1: Variable::new("lx"),
                arg_2: None,
                result: Variable::new("a")
            }
        );

        assert!(Reader::try_none("uups").is_none());
    }

    #[test]
    fn test_try_and() {
        assert_eq!(
            Reader::try_and("1 AND bh -> bi").unwrap(),
            Instruction {
                op: Operation::And,
                arg_1: Variable::new("1"),
                arg_2: Some(Variable::new("bh")),
                result: Variable::new("bi")
            }
        );

        assert!(Reader::try_and("uups").is_none());
    }

    #[test]
    fn test_try_or() {
        assert_eq!(
            Reader::try_or("42 OR 24 -> xy").unwrap(),
            Instruction {
                op: Operation::Or,
                arg_1: Variable::new("42"),
                arg_2: Some(Variable::new("24")),
                result: Variable::new("xy")
            }
        );

        assert!(Reader::try_or("uups").is_none());
    }
}
