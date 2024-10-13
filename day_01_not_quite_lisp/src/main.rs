use std::{path::PathBuf, process::exit};

use example::reader::Reader;

pub mod example;

fn main() {
    println!("--- Day 1: Not Quite Lisp ---");
    println!();

    let project_path = std::env::var("CARGO_MANIFEST_DIR")
        .expect("Environment variable CARGO_MANIFEST_DIR is not defined");

    let input_file = PathBuf::from(project_path)
        .join("resources")
        .join("input.txt");

    let reader = Reader::new(input_file);
    let building = reader.read().unwrap_or_else(|err| {
        println!("{}", err);
        exit(1);
    });

    println!("Part 1: Floor number: {}", building.count());
    println!(
        "Part 2: Position    : {}",
        building.find_entry(-1).expect("Failed to find position")
    );
}
