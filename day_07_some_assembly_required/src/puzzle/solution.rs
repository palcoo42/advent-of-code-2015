use std::collections::{HashMap, VecDeque};
use std::error::Error;

use puzzler::env::project;
use puzzler::puzzler::puzzle::Puzzle;

use crate::puzzle::blueprint::Blueprint;
use crate::puzzle::instruction::Instruction;

pub struct Solution {
    blueprints: Vec<Blueprint>,
    wires: HashMap<String, u16>,
}

impl Puzzle for Solution {
    fn name(&self) -> &str {
        "--- Day 07: Some Assembly Required ---"
    }

    fn get_input_file_path(&self) -> Option<std::path::PathBuf> {
        Some(
            project::get_project_file("../input/day_07.txt")
                .unwrap_or_else(|err| panic!("Failed to fetch file ../input/day_07.txt [{err}]")),
        )
    }

    // Parse the file content for the puzzle
    fn parse_content(&mut self, lines: Vec<String>) -> Result<(), Box<dyn Error>> {
        self.blueprints = lines
            .iter()
            .map(|line| Solution::parse_blueprint(line))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(())
    }

    // Solve first part of the puzzle
    fn solve_part1(&mut self) -> Result<String, Box<dyn Error>> {
        self.execute_blueprints();

        let signal_wire_a = self.get_wire_signal("a").unwrap();
        Ok(signal_wire_a.to_string())
    }

    // Solve second part of the puzzle
    fn solve_part2(&mut self) -> Result<String, Box<dyn Error>> {
        // Prepare
        self.wires.clear();
        self.wires.insert("b".to_string(), 46065);
        self.execute_blueprints();

        let signal_wire_a = self.get_wire_signal("a").unwrap();
        Ok(signal_wire_a.to_string())
    }
}

impl Solution {
    pub fn new() -> Self {
        Self {
            blueprints: vec![],
            wires: HashMap::new(),
        }
    }

    pub fn parse_blueprint(line: &str) -> Result<Blueprint, Box<dyn Error>> {
        // Split line to seperate instruction and destination wire
        let instructions = line.split_terminator("->").collect::<Vec<_>>();
        if instructions.len() != 2 {
            return Err(format!(
                "Blueprint shall have exactly 2 parts, found {} '{line}'",
                instructions.len(),
            )
            .into());
        }

        // Fetch name of the destination wire
        let wire = instructions[1].trim().to_string();

        // Arguments depends on instruction type
        let args = instructions[0].split_terminator(" ").collect::<Vec<_>>();

        let blueprint = if line.contains("AND") {
            Blueprint {
                instruction: Instruction::And(args[0].to_string(), args[2].to_string()),
                wire,
            }
        } else if line.contains("OR") {
            Blueprint {
                instruction: Instruction::Or(args[0].to_string(), args[2].to_string()),
                wire,
            }
        } else if line.contains("LSHIFT") {
            Blueprint {
                instruction: Instruction::Lshift(args[0].to_string(), args[2].to_string()),
                wire,
            }
        } else if line.contains("RSHIFT") {
            Blueprint {
                instruction: Instruction::Rshift(args[0].to_string(), args[2].to_string()),
                wire,
            }
        } else if line.contains("NOT") {
            Blueprint {
                instruction: Instruction::Not(args[1].to_string()),
                wire,
            }
        } else if instructions.len() == 2 {
            Blueprint {
                instruction: Instruction::Signal(args[0].to_string()),
                wire,
            }
        } else {
            return Err(format!("Unsupported blueprint '{line}'").into());
        };

        Ok(blueprint)
    }

    pub fn get_wire_signal(&self, wire: &str) -> Option<&u16> {
        self.wires.get(wire)
    }

    fn execute_blueprints(&mut self) {
        // Blueprint can be executed only of all input wires has signals.
        // Repeat the calculation until we have blueprints left.
        let mut queue = self.blueprints.iter().cloned().collect::<VecDeque<_>>();

        while let Some(blueprint) = queue.pop_front() {
            // println!("len: {}", queue.len());
            // println!("blueprint: {:?}", blueprint);

            let value: Option<u16> = match &blueprint.instruction {
                Instruction::Signal(signal) => self.get_signal_value(signal),
                Instruction::Not(signal) => self.get_signal_value(signal).map(|v| !v),
                Instruction::And(a, b) => {
                    let a = self.get_signal_value(a);
                    let b = self.get_signal_value(b);

                    if let Some(a) = a
                        && let Some(b) = b
                    {
                        Some(a & b)
                    } else {
                        None
                    }
                }
                Instruction::Or(a, b) => {
                    let a = self.get_signal_value(a);
                    let b = self.get_signal_value(b);

                    if let Some(a) = a
                        && let Some(b) = b
                    {
                        Some(a | b)
                    } else {
                        None
                    }
                }
                Instruction::Lshift(a, shift) => {
                    let a = self.get_signal_value(a);
                    let shift = self.get_signal_value(shift);

                    if let Some(a) = a
                        && let Some(shift) = shift
                    {
                        Some(a << shift)
                    } else {
                        None
                    }
                }
                Instruction::Rshift(a, shift) => {
                    let a = self.get_signal_value(a);
                    let shift = self.get_signal_value(shift);

                    if let Some(a) = a
                        && let Some(shift) = shift
                    {
                        Some(a >> shift)
                    } else {
                        None
                    }
                }
            };

            if let Some(number) = value {
                // Do not insert signal if it is already present - this trick will help to solve part 2,
                // because it will skip overwrite of hardcoded value for signal b.
                self.wires.entry(blueprint.wire).or_insert(number);
            } else {
                queue.push_back(blueprint);
            }
        }
    }

    fn get_signal_value(&self, signal: &str) -> Option<u16> {
        // 1. If value is number return its value
        // 2. If signal is already calculated return its value
        // 3. Return None as not available yet
        if let Ok(number) = signal.parse::<u16>() {
            Some(number)
        } else {
            self.wires.get(signal).copied()
        }
    }
}

#[cfg(test)]
mod tests {
    use puzzler::puzzler::puzzle::Puzzle;

    use crate::puzzle::solution::Solution;

    fn get_puzzle() -> Solution {
        let mut solution = Solution::new();

        solution
            .parse_input_file()
            .unwrap_or_else(|err| panic!("Failed to parse input file [{err}]"));

        solution
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(get_puzzle().solve_part1().unwrap(), "46065");
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(get_puzzle().solve_part2().unwrap(), "14134");
    }

    #[test]
    fn test_execute_blueprint() {
        let mut solution = Solution::new();
        solution.blueprints = vec![
            Solution::parse_blueprint("123 -> x").unwrap(),
            Solution::parse_blueprint("456 -> y").unwrap(),
            Solution::parse_blueprint("x AND y -> d").unwrap(),
            Solution::parse_blueprint("x OR y -> e").unwrap(),
            Solution::parse_blueprint("x LSHIFT 2 -> f").unwrap(),
            Solution::parse_blueprint("y RSHIFT 2 -> g").unwrap(),
            Solution::parse_blueprint("NOT x -> h").unwrap(),
            Solution::parse_blueprint("NOT y -> i").unwrap(),
        ];

        solution.execute_blueprints();
        assert_eq!(solution.get_wire_signal("d"), Some(&72));
        assert_eq!(solution.get_wire_signal("e"), Some(&507));
        assert_eq!(solution.get_wire_signal("f"), Some(&492));
        assert_eq!(solution.get_wire_signal("g"), Some(&114));
        assert_eq!(solution.get_wire_signal("h"), Some(&65412));
        assert_eq!(solution.get_wire_signal("i"), Some(&65079));
        assert_eq!(solution.get_wire_signal("x"), Some(&123));
        assert_eq!(solution.get_wire_signal("y"), Some(&456));
    }
}
