use std::process::exit;

use common::env::environment::get_project_root;
use example::reader::Reader;

pub mod example;

fn main() {
    println!("--- Day 15: Science for Hungry People ---");
    println!();

    let input_file = get_project_root().join("resources").join("input.txt");
    let recipe = Reader::read_recipe(&input_file).unwrap_or_else(|err| {
        eprintln!(
            "Failed to read input file '{}' with error '{}'",
            input_file.to_str().unwrap(),
            err
        );
        exit(1);
    });

    println!(
        "Part 1: Highest score                  : {}",
        recipe.highest_score()
    );
    println!(
        "Part 2: Highest score with 500 calories: {}",
        recipe.highest_score_with_calories(500)
    );
}
