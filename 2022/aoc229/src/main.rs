use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    // Read the input file
    let lines = io::BufReader::new(File::open("2022/aoc229/input").unwrap()).lines();

    // Iterator over the lines
    for line in lines.flatten() {
        println!("{}", line);
    }
}
