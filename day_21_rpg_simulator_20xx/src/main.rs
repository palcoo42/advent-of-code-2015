use std::process::exit;

use common::env::environment::get_project_root;
use example::{reader::Reader, simulation::Simulation};

pub mod example;

fn main() {
    println!("--- Day 21: RPG Simulator 20XX ---");
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

    const PLAYER_HEALTH: u32 = 100;

    let sim = Simulation::new(PLAYER_HEALTH, boss);
    println!(
        "Part 1: Minimal costs to win a battle: {}",
        sim.find_minimal_cost_to_win_battle()
    );
}
