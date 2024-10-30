use example::street::Street;

pub mod example;

fn main() {
    println!("--- Day 20: Infinite Elves and Infinite Houses ---");
    println!();

    let mut street = Street::new(1_000_000);

    println!(
        "Part 1: Lowest house number: {}",
        street.deliver_presents(36_000_000).unwrap()
    );
    println!(
        "Part 2: Lowest house number: {}",
        street.deliver_presents_finite(36_000_000).unwrap()
    );
}
