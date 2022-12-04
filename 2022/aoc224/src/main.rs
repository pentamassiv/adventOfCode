#![feature(iter_array_chunks)]

use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    // Read the input file
    let lines = io::BufReader::new(File::open("2022/aoc224/input").unwrap()).lines();

    let mut count = 0;
    // Iterator over the lines
    for line in lines.flatten() {
        let line = line
            .splitn(4, |c| c == '-' || c == ',')
            .map(|x| x.parse::<i32>().unwrap())
            .array_chunks::<4>()
            .filter(|[start_a, end_a, start_b, end_b]| {
                start_a == start_b || end_a == end_b || start_a.cmp(start_b) != end_a.cmp(end_b)
            })
            .count();
        count += line;
    }
    println!("{count}");
}
