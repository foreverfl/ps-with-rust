use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin.lock());

    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    let mut input = String::new();
    reader.read_line(&mut input).expect("Failed to read line");

    // 입력 값을 처리하고 출력하기 (예시)
    writeln!(writer, "입력한 값: {}", input.trim()).expect("Failed to write output");

    writer.flush().expect("Failed to flush output");
}
