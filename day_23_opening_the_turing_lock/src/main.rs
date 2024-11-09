use std::process::exit;

use common::env::environment::get_project_root;
use example::{cpu::Cpu, reader::Reader};

pub mod example;

fn main() {
    println!("--- Day 23: Opening the Turing Lock ---");
    println!();

    let input_file = get_project_root().join("resources").join("input.txt");
    let instructions = Reader::read_instructions(&input_file).unwrap_or_else(|err| {
        eprintln!(
            "Failed to read input file '{}' with error '{}'",
            input_file.to_str().unwrap(),
            err
        );
        exit(1);
    });

    let mut cpu = Cpu::new(instructions);

    cpu.run().unwrap_or_else(|err| {
        eprintln!("Failed to run instruction with error '{}'", err);
        exit(1);
    });

    println!(
        "Part 1: Register b has value: {}",
        cpu.get_register("b").unwrap()
    );

    // Part 2
    cpu.reset_registers_with_one();

    cpu.run().unwrap_or_else(|err| {
        eprintln!("Failed to run instruction with error '{}'", err);
        exit(1);
    });

    println!(
        "Part 2: Register b has value: {}",
        cpu.get_register("b").unwrap()
    );
}
