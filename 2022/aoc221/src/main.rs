use std::fs;

fn main() {
    // Read the input file
    let mut calories: Vec<i32> = fs::read_to_string("2022/aoc221/input")
        .unwrap()
        .split("\n\n")
        .map(calculate_calories_of_one_elf)
        .collect();
    calories.sort();

    println!("Part 1:");
    println!("{}", calories.iter().rev().take(1).sum::<i32>());
    println!("Part 2:");
    println!("{}", calories.iter().rev().take(3).sum::<i32>());
}

fn calculate_calories_of_one_elf(lines_of_one_elf: &str) -> i32 {
    lines_of_one_elf
        .lines()
        .filter_map(|str| str.parse::<i32>().ok())
        .sum()
}
