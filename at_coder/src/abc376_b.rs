// B - Hands on Ring (Easy)
// Algorithm: Circular Array
// URL: https://atcoder.jp/contests/abc376/tasks/abc376_b

use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn calculate_min_distance(
    n: usize,
    current_pos: usize,
    other_hand_pos: usize,
    target: usize,
) -> usize {
    let clockwise_distance = if target >= current_pos {
        target - current_pos
    } else {
        n - current_pos + target
    };

    let counter_clockwise_distance = if current_pos >= target {
        current_pos - target
    } else {
        current_pos + n - target
    };

    if target == other_hand_pos {
        usize::MAX // 다른 손이 목표 위치에 있으므로 이동 불가
    } else {
        // 두 경로 중 최소값 반환
        clockwise_distance.min(counter_clockwise_distance)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin.lock());

    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    let mut input = String::new();
    reader.read_line(&mut input).expect("Failed to read line");
    let parts: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect();

    let n = parts[0];
    let k = parts[1];

    let mut operations = Vec::new();

    for _ in 0..k {
        input.clear();
        reader.read_line(&mut input).expect("Failed to read line");
        let mut iter = input.split_whitespace();
        let command = iter.next().unwrap();
        let value: usize = iter
            .next()
            .unwrap()
            .parse()
            .expect("Failed to parse number");
        operations.push((command.to_string(), value));
    }

    let mut left_pos = 0;
    let mut right_pos = 1;
    let mut res = 0;

    for (command, target) in &operations {
        let min_distance = if command == "L" {
            let dist = calculate_min_distance(n, left_pos, right_pos, *target - 1);
            left_pos = *target - 1; // 왼손 위치 갱신
            dist
        } else {
            let dist = calculate_min_distance(n, right_pos, left_pos, *target - 1);
            right_pos = *target - 1; // 오른손 위치 갱신
            dist
        };

        // 이동 불가능한 경우를 고려하지 않는다면 거리 더하기
        if min_distance != usize::MAX {
            res += min_distance;
        }
    }

    writeln!(writer, "{}", res).expect("Failed to write");

    writer.flush().expect("Failed to flush output");
}
