use example::look_and_say::LookAndSay;

pub mod example;

fn main() {
    println!("--- Day 10: Elves Look, Elves Say ---");
    println!();

    println!(
        "Part 1: Length of Look-And-Say: {}",
        LookAndSay::translate_multiple_times("1321131112", 40).len()
    );
}
