use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // 입력값을 i32로 변환하고, 에러 처리
    match input.trim().parse::<i32>() {
        Ok(num) => {
            println!("You entered: {}", num);
        }
        Err(_) => {
            println!("Please enter a valid number.");
        }
    }
}
