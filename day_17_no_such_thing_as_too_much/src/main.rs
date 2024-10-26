use std::process::exit;

use common::env::environment::get_project_root;
use example::reader::Reader;

pub mod example;

fn main() {
    println!("--- Day 17: No Such Thing as Too Much ---");
    println!();

    let input_file = get_project_root().join("resources").join("input.txt");
    let eggnog = Reader::read_eggnog(&input_file).unwrap_or_else(|err| {
        eprintln!(
            "Failed to read file '{}' with error '{}'",
            input_file.to_str().unwrap(),
            err
        );
        exit(1);
    });

    println!("Part 1: Number of combinations: {}", eggnog.count(150));
}
