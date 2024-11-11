use std::process::exit;

use common::env::environment::get_project_root;
use example::{generator::Generator, reader::Reader};

pub mod example;

fn main() {
    println!("--- Day 24: It Hangs in the Balance ---");
    println!();

    let input_file = get_project_root().join("resources").join("input.txt");
    let packages = Reader::read_packages(&input_file).unwrap_or_else(|err| {
        eprintln!(
            "Failed to read file '{}' with error '{}'",
            input_file.to_str().unwrap(),
            err
        );
        exit(1);
    });

    let gen = Generator::new(packages);

    println!(
        "Part 1: Quantum entanglement: {}",
        gen.find_min_entanglement(3)
    );

    println!(
        "Part 2: Quantum entanglement: {}",
        gen.find_min_entanglement(4)
    );
}
