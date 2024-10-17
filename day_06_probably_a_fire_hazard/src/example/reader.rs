use std::path::Path;

use common::reader::{text_reader::TextReader, text_reader_error::TextReaderError};
use once_cell::sync::Lazy;
use regex::Regex;

use super::{
    action::Action, instruction::Instruction, instructions::Instructions, position::Position,
};

pub struct Reader {}

impl Reader {
    pub fn read_instructions(path: &Path) -> Result<Instructions, TextReaderError> {
        let reader = TextReader::new(path.to_path_buf());
        let lines = reader.read_lines(300)?;

        let mut instructions = Vec::new();

        for line in lines {
            match Self::decode_instruction(&line) {
                Ok(instruction) => instructions.push(instruction),
                Err(e) => return Err(e),
            }
        }

        Ok(Instructions::new(instructions))
    }

    fn decode_instruction(line: &str) -> Result<Instruction, TextReaderError> {
        // turn on 887,9 through 959,629
        // turn off 539,243 through 559,965
        // toggle 720,196 through 897,994
        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"(.*)\s(\d+),(\d+)\s+through\s+(\d+),(\d+)").expect("Invalid regex")
        });

        if let Some(captures) = RE.captures(line) {
            // action
            let action = Self::decode_action(&captures[1])?;
            let from = Self::decode_position(&captures[2], &captures[3])?;
            let to = Self::decode_position(&captures[4], &captures[5])?;

            Ok(Instruction { action, from, to })
        } else {
            Err(TextReaderError::GenericError(format!(
                "Failed to decode line '{}' to instructions",
                line
            )))
        }
    }

    fn decode_action(action: &str) -> Result<Action, TextReaderError> {
        let action = match action {
            "turn on" => Action::TurnOn,
            "turn off" => Action::TurnOff,
            "toggle" => Action::Toggle,
            _ => {
                return Err(TextReaderError::GenericError(format!(
                    "Unsupported action '{}'",
                    action
                )))
            }
        };

        Ok(action)
    }

    fn decode_position(x: &str, y: &str) -> Result<Position, TextReaderError> {
        let x = x.parse::<usize>().map_err(|err| {
            TextReaderError::GenericError(format!(
                "Failed to convert position x '{}' to usize with error '{}'",
                x, err
            ))
        })?;

        let y = y.parse::<usize>().map_err(|err| {
            TextReaderError::GenericError(format!(
                "Failed to convert position y '{}' to usize with error '{}'",
                y, err
            ))
        })?;

        Ok(Position { x, y })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_action() {
        assert_eq!(Reader::decode_action("turn on").unwrap(), Action::TurnOn);
        assert_eq!(Reader::decode_action("turn off").unwrap(), Action::TurnOff);
        assert_eq!(Reader::decode_action("toggle").unwrap(), Action::Toggle);
    }

    #[test]
    fn test_decode_action_unsupported_action() {
        assert!(Reader::decode_action("").is_err());
        assert!(Reader::decode_action("blabla").is_err());
        assert!(Reader::decode_action("uups").is_err());
    }

    #[test]
    fn test_decode_position() {
        assert_eq!(
            Reader::decode_position("123", "456").unwrap(),
            Position { x: 123, y: 456 }
        );
    }

    #[test]
    fn test_decode_position_invalid() {
        assert!(Reader::decode_position("a", "456").is_err());
        assert!(Reader::decode_position("123", "b").is_err());
        assert!(Reader::decode_position("a", "b").is_err());
    }

    #[test]
    fn test_decode_instruction() {
        assert_eq!(
            Reader::decode_instruction("turn on 613,565 through 952,659").unwrap(),
            Instruction {
                action: Action::TurnOn,
                from: Position { x: 613, y: 565 },
                to: Position { x: 952, y: 659 }
            }
        );

        assert_eq!(
            Reader::decode_instruction("turn off 838,342 through 938,444").unwrap(),
            Instruction {
                action: Action::TurnOff,
                from: Position { x: 838, y: 342 },
                to: Position { x: 938, y: 444 }
            }
        );

        assert_eq!(
            Reader::decode_instruction("toggle 752,335 through 957,733").unwrap(),
            Instruction {
                action: Action::Toggle,
                from: Position { x: 752, y: 335 },
                to: Position { x: 957, y: 733 }
            }
        );
    }

    #[test]
    fn test_decode_insruction_invalid_action() {
        assert!(Reader::decode_instruction("jump 752,335 through 957,733").is_err());
        assert!(Reader::decode_instruction("752,335 through 957,733").is_err());
    }

    #[test]
    fn test_decode_insruction_invalid_from_position() {
        assert!(Reader::decode_instruction("turn on a,335 through 957,733").is_err());
        assert!(Reader::decode_instruction("turn on 752,a through 957,733").is_err());
    }

    #[test]
    fn test_decode_insruction_invalid_to_position() {
        assert!(Reader::decode_instruction("turn on 752,335 through x,733").is_err());
        assert!(Reader::decode_instruction("turn on 752,335 through 957,y").is_err());
    }

    #[test]
    fn test_decode_insruction_invalid() {
        assert!(Reader::decode_instruction("").is_err());
        assert!(Reader::decode_instruction("blablabla").is_err());
        assert!(Reader::decode_instruction("turn on 1,2 through 3;4").is_err());
        assert!(Reader::decode_instruction("turn on 1,2 throXgh 3,4").is_err());
    }
}
