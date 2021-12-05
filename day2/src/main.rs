use std::fs::File;
use std::io::{self, BufRead};

fn value_for_prefix(s: &str, prefix: &str) -> Option<i64> {
    if !s.starts_with(prefix) {
        return None;
    }

    let result: i64 = str::parse(&s[prefix.len()..]).expect("failed to parse");
    Some(result)
}

fn part1() {
    let input = File::open("input.txt").expect("failed to open input.txt");

    let mut depth: i64 = 0;
    let mut distance: i64 = 0;

    for (_i, line) in io::BufReader::new(input).lines().enumerate() {
        let s = line.expect("failed to read line");

        distance += value_for_prefix(&s, "forward ").unwrap_or_default();

        depth += value_for_prefix(&s, "down ").unwrap_or_default();
        depth -= value_for_prefix(&s, "up ").unwrap_or_default();
    }

    println!("depth: {}", depth);
    println!("distance: {}", distance);
    println!("answer: {}", depth * distance);
}

fn part2() {
    let input = File::open("input.txt").expect("failed to open input.txt");

    let mut aim: i64 = 0;
    let mut depth: i64 = 0;
    let mut distance: i64 = 0;

    for (_i, line) in io::BufReader::new(input).lines().enumerate() {
        let s = line.expect("failed to read line");

        if let Some(forward) = value_for_prefix(&s, "forward ") {
            distance += forward;
            depth += aim * forward;
        } else {
            aim += value_for_prefix(&s, "down ").unwrap_or_default();
            aim -= value_for_prefix(&s, "up ").unwrap_or_default();
        }
    }

    println!("depth: {}", depth);
    println!("distance: {}", distance);
    println!("answer: {}", depth * distance);
}

fn main() {
    println!("\n🦀 Part 1\n");
    part1();
    println!("\n🦀 Part 2\n");
    part2();
}
