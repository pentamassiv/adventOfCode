#![feature(iter_array_chunks)]
#![warn(clippy::pedantic)]
//#![allow(cargo_common_metadata)]
fn main() {
    // Read the input file
    let input = std::fs::read_to_string("2022/aoc226/input").unwrap();

    let mut solution = (0, 0);
    solution.0 = find_idx_n_distinct_chars(input.as_bytes(), 4).unwrap_or(input.len() + 1);
    solution.1 = find_idx_n_distinct_chars(input.as_bytes(), 14).unwrap_or(input.len() + 1);

    println!("Part 1:");
    println!("{}", solution.0);
    println!("Part 2:");
    println!("{}", solution.1);
}

fn find_idx_n_distinct_chars(input: &[u8], n_distinct: usize) -> Option<usize> {
    input
        .windows(n_distinct)
        .enumerate()
        .find_map(|(no, ch)| (all_distinct(ch)).then_some(no + n_distinct))
}

fn all_distinct(slice: &[u8]) -> bool {
    for (no, char_a) in slice.iter().enumerate() {
        for char_b in slice.iter().skip(no + 1) {
            if char_a == char_b {
                return false;
            }
        }
    }
    true
}
