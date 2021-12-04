use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let input = File::open("input.txt").expect("can't open input.txt");

    let mut prev_value: i64 = 0;
    let mut counter = 0;
    for (i, line) in io::BufReader::new(input).lines().enumerate() {
        let s = line.expect("failed to read line");
        let cur_value: i64 = str::parse(&s).expect("failed to parse number");

        if i > 0 && cur_value > prev_value {
            counter += 1;
        }

        prev_value = cur_value;
    }

    println!("number of depth increases: {}", counter);
}
