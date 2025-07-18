use std::error::Error;
use std::path::PathBuf;

use puzzler::env::project;
use puzzler::puzzler::puzzle::Puzzle;

use crate::puzzle::instruction::Instruction;
use crate::puzzle::registers::Registers;

pub struct Solution {
    program: Vec<Instruction>,
}

impl Puzzle for Solution {
    fn name(&self) -> &str {
        "--- Day 23: Opening the Turing Lock ---"
    }

    fn get_input_file_path(&self) -> Option<PathBuf> {
        Some(
            project::get_project_file("../input/day_23.txt")
                .unwrap_or_else(|err| panic!("Failed to fetch file ../input/day_23.txt [{err}]")),
        )
    }

    fn parse_content(&mut self, lines: Vec<String>) -> Result<(), Box<dyn Error>> {
        let mut instructions = Vec::new();

        for line in lines {
            // Split line by whitespaces
            let mut split = line.split_whitespace();

            // First word is type of the instruction
            let cmd = split
                .next()
                .ok_or_else(|| format!("Failed to extract type of instruction '{line}'"))?;

            let instr = match cmd {
                "hlf" | "tpl" | "inc" => {
                    let register = split
                        .next()
                        .ok_or_else(|| format!("Failed to extract register '{line}'"))?
                        .trim();

                    match cmd {
                        "hlf" => Instruction::Hlf {
                            reg: String::from(register),
                        },
                        "tpl" => Instruction::Tpl {
                            reg: String::from(register),
                        },
                        "inc" => Instruction::Inc {
                            reg: String::from(register),
                        },
                        _ => {
                            panic!("Unexpected command '{cmd}'")
                        }
                    }
                }
                "jmp" => {
                    let offset = split
                        .next()
                        .ok_or_else(|| format!("Failed to extract offset '{line}'"))?
                        .trim()
                        .replace("+", "")
                        .parse::<isize>()
                        .map_err(|_| format!("Failed to extract offset '{line}'"))?;

                    Instruction::Jmp { offset }
                }
                "jie" | "jio" => {
                    let register = split
                        .next()
                        .ok_or_else(|| format!("Failed to extract register '{line}'"))?
                        .trim();

                    let register = register.replace(",", "");

                    let offset = split
                        .next()
                        .ok_or_else(|| format!("Failed to extract offset '{line}'"))?
                        .replace("+", "")
                        .trim()
                        .parse::<isize>()
                        .map_err(|_| format!("Failed to extract offset '{line}'"))?;

                    match cmd {
                        "jie" => Instruction::Jie {
                            reg: register,
                            offset,
                        },
                        "jio" => Instruction::Jio {
                            reg: register,
                            offset,
                        },
                        _ => {
                            panic!("Unexpected command '{cmd}'")
                        }
                    }
                }
                _ => {
                    return Err(format!("Unsupported instruction '{cmd}' [{line}]").into());
                }
            };

            instructions.push(instr);
        }

        self.program = instructions;
        Ok(())
    }

    fn solve_part1(&mut self) -> Result<String, Box<dyn Error>> {
        let register_b = Self::run_program(&self.program, 0, "b");
        Ok(register_b.to_string())
    }

    fn solve_part2(&mut self) -> Result<String, Box<dyn Error>> {
        let register_b = Self::run_program(&self.program, 1, "b");
        Ok(register_b.to_string())
    }
}

impl Solution {
    pub fn new() -> Self {
        Self {
            program: Vec::new(),
        }
    }

    fn run_program(program: &[Instruction], register_a: usize, register_output: &str) -> usize {
        // Create registers
        let mut registers = Registers::new();
        registers["a"] = register_a;

        // Start at the beginning of the program
        let mut stack_index = 0;
        let program_length = program.len();

        loop {
            let inst = &program[stack_index];

            let new_offest = match inst {
                Instruction::Hlf { reg } => {
                    // Half the value
                    registers[reg] /= 2;
                    (stack_index + 1) as isize
                }
                Instruction::Tpl { reg } => {
                    // Triple the value
                    registers[reg] *= 3;
                    (stack_index + 1) as isize
                }
                Instruction::Inc { reg } => {
                    // Increment by 1
                    registers[reg] += 1;
                    (stack_index + 1) as isize
                }
                Instruction::Jmp { offset } => stack_index as isize + offset,
                Instruction::Jie { reg, offset } => {
                    // Jump if even
                    match registers[reg] % 2 == 0 {
                        true => stack_index as isize + offset,
                        false => (stack_index + 1) as isize,
                    }
                }
                Instruction::Jio { reg, offset } => {
                    // Jump if one
                    match registers[reg] == 1 {
                        true => stack_index as isize + offset,
                        false => (stack_index + 1) as isize,
                    }
                }
            };

            // Program ends when new offset is out of program bounds
            if new_offest < 0 || new_offest >= program_length as isize {
                break;
            }

            // Assign index to the next instruction
            stack_index = new_offest as usize;
        }

        registers[register_output]
    }
}

#[cfg(test)]
mod tests {
    use puzzler::puzzler::puzzle::Puzzle;

    use crate::puzzle::{instruction::Instruction, solution::Solution};

    fn get_puzzle() -> Solution {
        let mut solution = Solution::new();

        solution
            .parse_input_file()
            .unwrap_or_else(|err| panic!("Failed to parse input file [{err}]"));

        solution
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(get_puzzle().solve_part1().unwrap(), "170");
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(get_puzzle().solve_part2().unwrap(), "247");
    }

    #[test]
    fn test_run_program() {
        let program = vec![
            Instruction::Inc {
                reg: String::from("a"),
            },
            Instruction::Jio {
                reg: String::from("a"),
                offset: 2,
            },
            Instruction::Tpl {
                reg: String::from("a"),
            },
            Instruction::Inc {
                reg: String::from("a"),
            },
        ];

        assert_eq!(Solution::run_program(&program, 0, "a"), 2);
    }
}
