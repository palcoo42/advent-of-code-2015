use std::process::exit;

use common::env::environment::get_project_root;
use example::reader::Reader;

pub mod example;

fn main() {
    println!("--- Day 14: Reindeer Olympics ---");
    println!();

    let input_file = get_project_root().join("resources").join("input.txt");
    let herd = Reader::read_herd(&input_file).unwrap_or_else(|err| {
        eprintln!(
            "Failed to read input file '{}' with error '{}'",
            input_file.to_str().unwrap(),
            err
        );
        exit(1)
    });

    println!(
        "Part 1: Distance of the winning reindeer: {}",
        herd.race_winning_distance(2503)
    );
}
