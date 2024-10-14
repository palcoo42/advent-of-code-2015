use std::process::exit;

use common::env::environment::get_project_root;
use example::reader::Reader;

pub mod example;

fn main() {
    println!("--- Day 2: I Was Told There Would Be No Math ---");
    println!();

    let input_file = get_project_root().join("resources").join("input.txt");
    let presents = Reader::read_presents(input_file, 1000).unwrap_or_else(|e| {
        eprintln!("Failed to read presents with error '{}'", e);
        exit(1);
    });

    println!("Part 1: Wrapping paper: {}", presents.wrapping_paper());
    println!("Part 2: Ribbon        : {}", presents.ribbon());
}
