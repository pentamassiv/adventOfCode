use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    // Read the input file
    let lines = io::BufReader::new(File::open("2022/aoc221/input").unwrap()).lines();

    let mut max_calories = vec![0, 0, 0];
    let mut current_calories = 0;
    let mut pos = 3;

    for line in lines.flatten() {
        if line.is_empty() {
            for (no, max_cal) in max_calories.iter().enumerate() {
                if current_calories > *max_cal {
                    pos = no;
                    break;
                }
            }
            max_calories.insert(pos, current_calories);
            pos = 3;
            max_calories.pop();
            current_calories = 0;
        } else {
            current_calories += line.parse().unwrap_or(0);
        }
    }
    println!("Part 1:");
    println!("{}", max_calories[0]);
    println!("Part 2:");
    println!("{}", max_calories.iter().sum::<i32>());
}
