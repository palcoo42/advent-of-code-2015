use std::process::exit;

use common::env::environment::get_project_root;
use example::reader::Reader;

pub mod example;

fn main() {
    println!("--- Day 19: Medicine for Rudolph ---");
    println!();

    let input_file = get_project_root().join("resources").join("input.txt");
    let (machine, molecule) =
        Reader::read_machine_and_molecule(&input_file).unwrap_or_else(|err| {
            eprintln!(
                "Failed to open file '{}' with an error '{}'",
                input_file.to_str().unwrap(),
                err
            );
            exit(1);
        });

    println!(
        "Part 1: Number of distinct molecules: {}",
        machine.get_number_of_distinct_molecules(&molecule)
    );

    println!(
        "Part 2: Number of minimal steps     : {}",
        machine.fabricate_molecule_minimum_steps(&molecule).unwrap()
    )
}
