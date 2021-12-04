use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let input = File::open("../day1/input.txt").expect("failed to open input.txt");

    let mut p1: i64 = 0;
    let mut p2: i64 = 0;
    let mut p3: i64 = 0;
    let mut counter = 0;

    for (i, line) in io::BufReader::new(input).lines().enumerate() {
        let s = line.expect("failed to read line");
        let value: i64 = str::parse(&s).expect("failed to parse number");

        if i > 3 {
            if (p2 + p3 + value) > (p1 + p2 + p3) {
                counter += 1;
            }
        }

        p1 = p2;
        p2 = p3;
        p3 = value;
    }

    println!("three-measurement sliding window increases: {}", counter);
}
