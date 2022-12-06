#![feature(iter_array_chunks)]

fn main() {
    // Read the input file
    let input = std::fs::read_to_string("2022/aoc226/input").unwrap();
    let mut processed_chars = (0, 0);
    for no in 3..input.len() {
        if first_n_uneq(&input[no - 3..=no]) > 3 {
            processed_chars.0 = no + 1;
            break;
        }
    }
    for no in 13..input.len() {
        if first_n_uneq(&input[no - 13..=no]) == 14 {
            processed_chars.1 = no + 1;
            break;
        }
    }

    println!("Part 1:");
    println!("{}", processed_chars.0);
    println!("Part 2:");
    println!("{}", processed_chars.1);
}

fn first_n_uneq(slice: &str) -> usize {
    for (no, char_a) in slice.chars().enumerate() {
        for char_b in slice.chars().skip(no + 1) {
            if char_a == char_b {
                return no + 1;
            }
        }
    }
    slice.len()
}
