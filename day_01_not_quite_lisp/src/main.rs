use std::process::exit;

use common::{env::environment::get_project_root, reader::text_reader::TextReader};
use example::building::Building;

pub mod example;

fn main() {
    println!("--- Day 1: Not Quite Lisp ---");
    println!();

    let input_file = get_project_root().join("resources").join("input.txt");

    let reader = TextReader::new(input_file);
    let lines = reader.read_lines(1).unwrap_or_else(|e| {
        println!("Failed to read lines with error '{}'", e);
        exit(1);
    });

    let building = Building::new(lines.first().expect("No lines"));

    println!("Part 1: Floor number: {}", building.count());
    println!(
        "Part 2: Position    : {}",
        building.find_entry(-1).expect("Failed to find position")
    );
}
