use std::process::exit;

use common::env::environment::get_project_root;
use example::reader::Reader;

pub mod example;

fn main() {
    println!("--- Day 18: Like a GIF For Your Yard ---");
    println!();

    let input_file = get_project_root().join("resources").join("input.txt");
    let mut grid = Reader::read_grid(&input_file).unwrap_or_else(|err| {
        eprintln!(
            "Failed to read file '{}' with error '{}'",
            input_file.to_str().unwrap(),
            err
        );
        exit(1);
    });

    grid.steps(100);
    println!("Part 1: Number of lights on: {}", grid.lights_on_count());
}
