#![feature(iter_array_chunks)]

use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    // Read the input file
    let lines = io::BufReader::new(File::open("2022/aoc224/input").unwrap()).lines();

    let mut complete_overlaps = 0;
    let mut partial_overlaps = 0;
    // Iterator over the lines
    lines.flatten().for_each(|line| {
        line.splitn(4, |c| c == '-' || c == ',')
            .map(|x| x.parse::<i32>().unwrap())
            .array_chunks::<4>()
            .filter(|[start_a, end_a, start_b, end_b]| {
                (*start_a..=*end_a).contains(start_b)
                    || (*start_a..=*end_a).contains(end_b)
                    || (*start_b..=*end_b).contains(start_a)
                    || (*start_b..=*end_b).contains(end_a)
            })
            .inspect(|_| {
                partial_overlaps += 1;
            })
            .filter(|[start_a, end_a, start_b, end_b]| start_a.cmp(start_b) != end_a.cmp(end_b))
            .for_each(|_| {
                complete_overlaps += 1;
            })
    });

    println!("{complete_overlaps}");
    println!("{partial_overlaps}");
}
