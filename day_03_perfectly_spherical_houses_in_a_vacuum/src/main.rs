use std::process::exit;

use common::env::environment::get_project_root;
use example::{houses::Houses, reader::Reader};

pub mod example;

fn main() {
    println!("--- Day 3: Perfectly Spherical Houses in a Vacuum ---");
    println!();

    let input_file = get_project_root().join("resources").join("input.txt");
    let instructions = Reader::read_instructions(input_file.as_path()).unwrap_or_else(|e| {
        eprintln!("Failed to read instructions with error '{}'", e);
        exit(1);
    });

    let mut houses = Houses::new();
    houses.deliver_presents(&instructions);

    println!(
        "Part 1: Houses with at least one present: {}",
        houses.at_least_one_present()
    );
}
