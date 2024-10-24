use std::process::exit;

use common::env::environment::get_project_root;
use example::reader::Reader;

pub mod example;

fn main() {
    println!("--- Day 13: Knights of the Dinner Table ---");
    println!();

    let input_file = get_project_root().join("resources").join("input.txt");
    let table = Reader::read_table(&input_file).unwrap_or_else(|err| {
        eprintln!(
            "Failed to read input file '{}'with error '{}'",
            input_file.to_str().unwrap(),
            err
        );
        exit(1);
    });

    println!(
        "Part 1: Highest happiness: {}",
        table.seat_persons_with_maximum_happiness()
    );

    let input_file = get_project_root().join("resources").join("input_me.txt");
    let table = Reader::read_table(&input_file).unwrap_or_else(|err| {
        eprintln!(
            "Failed to read input file '{}'with error '{}'",
            input_file.to_str().unwrap(),
            err
        );
        exit(1);
    });

    println!(
        "Part 2: Highest happiness: {}",
        table.seat_persons_with_maximum_happiness()
    );
}
