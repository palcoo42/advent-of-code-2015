use std::process::exit;

use common::env::environment::get_project_root;
use example::reader::Reader;

pub mod example;

fn main() {
    println!("--- Day 5: Doesn't He Have Intern-Elves For This? ---");
    println!();

    let input_file = get_project_root().join("resources").join("input.txt");
    let words = Reader::read_words(&input_file).unwrap_or_else(|err| {
        eprintln!(
            "Failed to read words from file '{:?}' with error '{}'",
            input_file, err
        );
        exit(1)
    });

    println!("Part 1: Nice  words: {}", words.count_nice_words());
    println!("Part 2: Nicer words: {}", words.count_nicer_words());
}
