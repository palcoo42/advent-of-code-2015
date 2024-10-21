use std::process::exit;

use common::env::environment::get_project_root;
use example::reader::Reader;

pub mod example;

fn main() {
    println!("--- Day 8: Matchsticks ---");
    println!();

    let input_file = get_project_root().join("resources").join("input.txt");
    let literals = Reader::read_literals(&input_file).unwrap_or_else(|err| {
        eprintln!(
            "Failed to read literals from file '{:?}' with error '{}'",
            input_file, err
        );
        exit(1);
    });

    println!("Part 1: Difference: {}", literals.diff());
}
