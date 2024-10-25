use std::process::exit;

use common::env::environment::get_project_root;
use example::{computer::Computer, reader::Reader, tape::Tape};

pub mod example;

fn main() {
    println!("--- Day 16: Aunt Sue ---");
    println!();

    let input_file = get_project_root().join("resources").join("input.txt");

    let aunts = Reader::read_aunts(&input_file).unwrap_or_else(|err| {
        eprintln!(
            "Failed to read input file '{}' with error '{}'",
            input_file.to_str().unwrap(),
            err
        );
        exit(1);
    });

    let tape = Tape::new(vec![
        ("children".to_string(), 3),
        ("cats".to_string(), 7),
        ("samoyeds".to_string(), 2),
        ("pomeranians".to_string(), 3),
        ("akitas".to_string(), 0),
        ("vizslas".to_string(), 0),
        ("goldfish".to_string(), 5),
        ("trees".to_string(), 3),
        ("cars".to_string(), 2),
        ("perfumes".to_string(), 1),
    ]);

    let computer = Computer::new(&aunts);

    println!(
        "Part 1: Number of the Sue: {}",
        computer.find_sue(&tape).unwrap().id()
    );
}
