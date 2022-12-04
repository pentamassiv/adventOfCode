use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    // Read the input file
    let lines = io::BufReader::new(File::open("2022/aoc224/input").unwrap()).lines();

    let mut count = 0;
    // Iterator over the lines
    for line in lines.flatten() {
        let line: Vec<&str> = line.split_terminator(&['-', ',']).take(4).collect();
        let mut line = vec![(1, line[0]), (1, line[1]), (2, line[2]), (2, line[3])];
        line.sort_by_key(|(_, value)| value.parse::<i32>().unwrap());
        if line[0].0 == line[3].0 {
            count += 1;
        }
    }
    println!("{count}");
}
