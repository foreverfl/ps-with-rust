use std::io::{self, BufRead};

fn main() {
    println!("Please enter a line of text: ");
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.lock().read_line(&mut buffer).expect("Failed to read line");
    println!("You entered: {}", buffer.trim());
}