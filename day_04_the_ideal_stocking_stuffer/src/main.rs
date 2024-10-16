use example::advent_coin::AdventCoin;

pub mod example;

fn main() {
    println!("--- Day 4: The Ideal Stocking Stuffer ---");
    println!();

    println!("Part 1: {}", AdventCoin::mine("yzbqklnj", 5));
    println!("Part 2: {}", AdventCoin::mine("yzbqklnj", 6));
}
