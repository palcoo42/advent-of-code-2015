use std::process::exit;

use common::env::environment::get_project_root;
use example::{manual::Manual, reader::Reader};

pub mod example;

fn main() {
    println!("--- Day 25: Let It Snow ---");
    println!();

    let input_file = get_project_root().join("resources").join("input.txt");
    let coordinates = Reader::read_coordinates(&input_file).unwrap_or_else(|err| {
        eprintln!(
            "Failed to parse file '{}' with error '{}'",
            input_file.to_str().unwrap(),
            err
        );
        exit(1);
    });

    println!("Part 1: Manual code: {}", Manual::get_code(coordinates));
}
