use example::password::Password;

pub mod example;

fn main() {
    println!("--- Day 11: Corporate Policy ---");
    println!();

    println!(
        "Part 1: New password: {}",
        Password::new("hepxcrrq").find_next_valid_password().get()
    );
}
