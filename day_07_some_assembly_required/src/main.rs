use std::process::exit;

use common::env::environment::get_project_root;
use example::{circuit::Circuit, reader::Reader};

pub mod example;

fn main() {
    println!("--- Day 7: Some Assembly Required ---");
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

    let mut circuit = Circuit::new();
    circuit.process(&instructions);

    println!(
        "Part 1: Wire a: {}",
        circuit.get_wire("a").expect("Failed to find wire 'a'")
    );
}
