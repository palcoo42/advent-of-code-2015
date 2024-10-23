use std::process::exit;

use common::env::environment::get_project_root;
use example::abacus::Abacus;

pub mod example;

fn main() {
    println!("--- Day 12: JSAbacusFramework.io ---");
    println!();

    let input_file = get_project_root().join("resources").join("input.txt");
    let abacus = Abacus::from_file(&input_file).unwrap_or_else(|err| {
        eprintln!("Failed to create Abacus with error '{}'", err);
        exit(1);
    });

    println!("Part 1: Sum of all numbers: {}", abacus.sum_simple());
    println!("Part 2: Sum of all numbers: {}", abacus.sum_complex());
}
