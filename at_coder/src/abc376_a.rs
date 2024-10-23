// A - Candy Button
// Algorithm: Greedy

use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x: &str| x.parse().expect("Not an integer"))
        .collect();
    let _a = nums[0];
    let b = nums[1];

    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let values: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Not an integer"))
        .collect();

    let mut count = 1;
    let mut last_time = values[0];

    for i in 1..values.len() {
        if values[i] - last_time >= b {
            count += 1;
            last_time = values[i];
        }
    }

    println!("{}", count);
}
