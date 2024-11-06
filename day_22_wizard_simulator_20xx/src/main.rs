use std::process::exit;

use common::env::environment::get_project_root;
use example::{reader::Reader, simulation::Simulation, wizard::Wizard};

pub mod example;

fn main() {
    println!("--- Day 22: Wizard Simulator 20XX ---");
    println!();

    let input_file = get_project_root().join("resources").join("input.txt");
    let boss = Reader::read_boss(&input_file).unwrap_or_else(|err| {
        eprintln!(
            "Failed to read input file '{}' with error '{}'",
            input_file.to_str().unwrap(),
            err
        );
        exit(1);
    });

    let wizard = Wizard::new(50, 500);
    let mut sim = Simulation::new(wizard, boss);

    let games_lowest_mana =
        sim.find_lowest_mana_cost_to_win(example::difficulty::Difficulty::Normal);

    println!(
        "Part 1: Lowest mana cost to win: {} [{} solutions]",
        games_lowest_mana.get_spent_mana().unwrap(),
        games_lowest_mana.get_games().len()
    );

    println!();
    for (id, solution) in games_lowest_mana.get_games().iter().enumerate() {
        println!("{:2} {:?}", id + 1, solution.get_history());
    }
    println!();

    // Part 2
    let games_lowest_mana = sim.find_lowest_mana_cost_to_win(example::difficulty::Difficulty::Hard);

    println!(
        "Part 2: Lowest mana cost to win: {} [{} solutions]",
        games_lowest_mana.get_spent_mana().unwrap(),
        games_lowest_mana.get_games().len()
    );

    println!();
    for (id, solution) in games_lowest_mana.get_games().iter().enumerate() {
        println!("{:2} {:?}", id + 1, solution.get_history());
    }
}
