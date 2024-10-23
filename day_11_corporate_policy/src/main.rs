use example::password::Password;

pub mod example;

fn main() {
    println!("--- Day 11: Corporate Policy ---");
    println!();

    let original_pwd = Password::new("hepxcrrq");

    let next_pwd = original_pwd.find_next_valid_password();
    println!("Part 1: Next password: {}", next_pwd.get());

    let next_next_pwd = next_pwd.find_next_valid_password();
    println!("Part 2: Next password: {}", next_next_pwd.get());
}
