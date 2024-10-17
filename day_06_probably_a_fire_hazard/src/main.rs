use std::process::exit;

use common::env::environment::get_project_root;
use example::{action::Action, grid::Grid, reader::Reader};

pub mod example;

fn main() {
    println!("--- Day 6: Probably a Fire Hazard ---");
    println!();

    let input_file = get_project_root().join("resources").join("input.txt");
    let instructions = Reader::read_instructions(&input_file).unwrap_or_else(|err| {
        eprintln!(
            "Failed to read instructions from file '{}' with error '{}'",
            input_file.to_str().unwrap(),
            err
        );
        exit(1);
    });

    let mut grid = Grid::new(1000, 1000);

    for instruction in &instructions.data {
        match instruction.action {
            Action::TurnOn => grid.turn_on(&instruction.from, &instruction.to),
            Action::TurnOff => grid.turn_off(&instruction.from, &instruction.to),
            Action::Toggle => grid.toggle(&instruction.from, &instruction.to),
        }
    }

    println!("Part 1: Lights on: {}", grid.count_bulbs_on());
}
