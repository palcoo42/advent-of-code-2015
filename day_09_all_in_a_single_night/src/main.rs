use std::process::exit;

use common::env::environment::get_project_root;
use example::reader::Reader;

pub mod example;

fn main() {
    println!("--- Day 9: All in a Single Night ---");
    println!();

    let input_file = get_project_root().join("resources").join("input.txt");
    let locations = Reader::read_locations(&input_file).unwrap_or_else(|err| {
        println!(
            "Failed to read locations from file '{:?}' with error '{}'",
            input_file, err
        );
        exit(1)
    });

    println!("Part 1: Shortest path: {}", locations.find_shortest_path());
    println!("Part 2: Longest path : {}", locations.find_longest_path());
}
