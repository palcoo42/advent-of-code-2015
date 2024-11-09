use std::path::Path;

use common::reader::{text_reader::TextReader, text_reader_error::TextReaderError};
use once_cell::sync::Lazy;
use regex::Regex;

use crate::example::cpu_instructions::{
    half::Half, jump::Jump, jump_if_even::JumpIfEven, jump_if_one::JumpIfOne,
};

use super::{
    cpu_instructions::{increment::Increment, instruction::Instruction, triple::Triple},
    instructions::Instructions,
};

pub struct Reader {}

impl Reader {
    pub fn read_instructions(path: &Path) -> Result<Instructions, TextReaderError> {
        let reader = TextReader::new(path.to_path_buf());
        let lines = reader.read_lines(50)?;

        let instructions = Self::parse_from_lines(lines)?;
        Ok(instructions)
    }

    fn parse_from_lines(lines: Vec<String>) -> Result<Instructions, TextReaderError> {
        let mut instructions = Instructions::new();

        for line in lines {
            // Skip empty lines
            if !line.is_empty() {
                let instr = Self::parse_instruction(&line)?;
                instructions.push(instr);
            }
        }

        Ok(instructions)
    }

    // Parse single line to the instruction. Supported instructions:
    //
    // hlf r            sets register r to half its current value
    // tpl r            sets register r to triple its current value
    // inc r            increments register r, adding 1 to it
    // jmp offset       is a jump
    // jie r, offset    is like jmp, but only jumps if register r is even ("jump if even").
    // jio r, offset    is like jmp, but only jumps if register r is 1 ("jump if one", not odd).
    fn parse_instruction(line: &str) -> Result<Box<dyn Instruction>, TextReaderError> {
        // Fetch name of the instruction (first value)
        let name = line.split_whitespace().nth(0).ok_or_else(|| {
            TextReaderError::GenericError(format!("Failed to split line '{}'", line))
        })?;

        // Depending on the instruction name decode it's content
        let inst: Box<dyn Instruction> = match name {
            "hlf" => Box::new(Self::parse_hlf(line)?),
            "tpl" => Box::new(Self::parse_tpl(line)?),
            "inc" => Box::new(Self::parse_inc(line)?),
            "jmp" => Box::new(Self::parse_jmp(line)?),
            "jie" => Box::new(Self::parse_jie(line)?),
            "jio" => Box::new(Self::parse_jio(line)?),
            _ => {
                return Err(TextReaderError::GenericError(format!(
                    "Unsupported instruction '{}'",
                    name
                )));
            }
        };

        Ok(inst)
    }

    fn parse_hlf(line: &str) -> Result<Half, TextReaderError> {
        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r#"^hlf\s+(\w+)"#).expect("Failed to create regex for instruction 'hlf'")
        });

        if let Some(captures) = RE.captures(line) {
            let register = &captures[1];
            return Ok(Half::new(register));
        }

        Err(TextReaderError::GenericError(format!(
            "Failed to parse line '{}' to 'hlf' instruction",
            line
        )))
    }

    fn parse_tpl(line: &str) -> Result<Triple, TextReaderError> {
        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r#"^tpl\s+(\w+)"#).expect("Failed to create regex for instruction 'tpl'")
        });

        if let Some(captures) = RE.captures(line) {
            let register = &captures[1];
            return Ok(Triple::new(register));
        }

        Err(TextReaderError::GenericError(format!(
            "Failed to parse line '{}' to 'tpl' instruction",
            line
        )))
    }

    fn parse_inc(line: &str) -> Result<Increment, TextReaderError> {
        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r#"^inc\s+(\w+)"#).expect("Failed to create regex for instruction 'inc'")
        });

        if let Some(captures) = RE.captures(line) {
            let register = &captures[1];
            return Ok(Increment::new(register));
        }

        Err(TextReaderError::GenericError(format!(
            "Failed to parse line '{}' to 'inc' instruction",
            line
        )))
    }

    fn parse_jmp(line: &str) -> Result<Jump, TextReaderError> {
        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r#"^jmp\s+([\+\-]?\w+)"#)
                .expect("Failed to create regex for instruction 'jmp'")
        });

        if let Some(captures) = RE.captures(line) {
            let offset = &captures[1].parse::<i32>().map_err(|err| {
                TextReaderError::GenericError(format!(
                    "Failed to convert offset '{}' to i32 with error '{}', line: {}",
                    &captures[1], err, line
                ))
            })?;

            return Ok(Jump::new(*offset));
        }

        Err(TextReaderError::GenericError(format!(
            "Failed to parse line '{}' to 'jmp' instruction",
            line
        )))
    }

    fn parse_jie(line: &str) -> Result<JumpIfEven, TextReaderError> {
        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r#"^jie\s+(\w+)\,\s+([\+\-]?\w+)"#)
                .expect("Failed to create regex for instruction 'jie'")
        });

        if let Some(captures) = RE.captures(line) {
            let register = &captures[1];
            let offset = &captures[2].parse::<i32>().map_err(|err| {
                TextReaderError::GenericError(format!(
                    "Failed to convert offset '{}' to i32 with error '{}', line: {}",
                    &captures[2], err, line
                ))
            })?;

            return Ok(JumpIfEven::new(register, *offset));
        }

        Err(TextReaderError::GenericError(format!(
            "Failed to parse line '{}' to 'jie' instruction",
            line
        )))
    }

    fn parse_jio(line: &str) -> Result<JumpIfOne, TextReaderError> {
        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r#"^jio\s+(\w+)\,\s+([\+\-]?\w+)"#)
                .expect("Failed to create regex for instruction 'jio'")
        });

        if let Some(captures) = RE.captures(line) {
            let register = &captures[1];
            let offset = &captures[2].parse::<i32>().map_err(|err| {
                TextReaderError::GenericError(format!(
                    "Failed to convert offset '{}' to i32 with error '{}', line: {}",
                    &captures[2], err, line
                ))
            })?;

            return Ok(JumpIfOne::new(register, *offset));
        }

        Err(TextReaderError::GenericError(format!(
            "Failed to parse line '{}' to 'jio' instruction",
            line
        )))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_from_lines() {
        let lines = ["inc a", "jio a, +2", "tpl a", "inc a"]
            .iter()
            .map(|&x| String::from(x))
            .collect::<Vec<_>>();

        let result = Reader::parse_from_lines(lines);

        assert!(result.is_ok(), "Failed with error: {:?}", result);
        assert_eq!(result.unwrap().len(), 4);
    }

    #[test]
    fn test_parse_inc() {
        assert_eq!(Reader::parse_inc("inc a").unwrap().get_register_name(), "a");
        assert_eq!(Reader::parse_inc("inc x").unwrap().get_register_name(), "x");
        assert!(Reader::parse_inc("inc").is_err());
    }

    #[test]
    fn test_parse_hlf() {
        assert_eq!(Reader::parse_hlf("hlf a").unwrap().get_register_name(), "a");
        assert_eq!(Reader::parse_hlf("hlf x").unwrap().get_register_name(), "x");
        assert!(Reader::parse_hlf("hlf").is_err());
    }

    #[test]
    fn test_parse_tpl() {
        assert_eq!(Reader::parse_tpl("tpl a").unwrap().get_register_name(), "a");
        assert_eq!(Reader::parse_tpl("tpl x").unwrap().get_register_name(), "x");
        assert!(Reader::parse_tpl("tpl").is_err());
    }

    #[test]
    fn test_parse_jmp() {
        assert_eq!(Reader::parse_jmp("jmp +42").unwrap().get_offset(), 42);
        assert_eq!(Reader::parse_jmp("jmp -42").unwrap().get_offset(), -42);
        assert_eq!(Reader::parse_jmp("jmp 0").unwrap().get_offset(), 0);
        assert!(Reader::parse_jmp("jmp").is_err());
    }

    #[test]
    fn test_parse_jie() {
        let jie = Reader::parse_jie("jie a, +42");
        assert!(jie.is_ok(), "Failed with error '{:?}'", jie);
        assert_eq!(jie.as_ref().unwrap().get_register_name(), "a");
        assert_eq!(jie.as_ref().unwrap().get_offset(), 42);

        let jie = Reader::parse_jie("jie a, -42");
        assert!(jie.is_ok(), "Failed with error '{:?}'", jie);
        assert_eq!(jie.as_ref().unwrap().get_register_name(), "a");
        assert_eq!(jie.as_ref().unwrap().get_offset(), -42);

        let jie = Reader::parse_jie("jie a, 0");
        assert!(jie.is_ok(), "Failed with error '{:?}'", jie);
        assert_eq!(jie.as_ref().unwrap().get_register_name(), "a");
        assert_eq!(jie.as_ref().unwrap().get_offset(), 0);

        assert!(Reader::parse_jmp("jie").is_err());
    }

    #[test]
    fn test_parse_jio() {
        let jio = Reader::parse_jio("jio a, +42");
        assert!(jio.is_ok(), "Failed with error '{:?}'", jio);
        assert_eq!(jio.as_ref().unwrap().get_register_name(), "a");
        assert_eq!(jio.as_ref().unwrap().get_offset(), 42);

        let jio = Reader::parse_jio("jio a, -42");
        assert!(jio.is_ok(), "Failed with error '{:?}'", jio);
        assert_eq!(jio.as_ref().unwrap().get_register_name(), "a");
        assert_eq!(jio.as_ref().unwrap().get_offset(), -42);

        let jio = Reader::parse_jio("jio a, 0");
        assert!(jio.is_ok(), "Failed with error '{:?}'", jio);
        assert_eq!(jio.as_ref().unwrap().get_register_name(), "a");
        assert_eq!(jio.as_ref().unwrap().get_offset(), 0);

        assert!(Reader::parse_jmp("jio").is_err());
    }
}
