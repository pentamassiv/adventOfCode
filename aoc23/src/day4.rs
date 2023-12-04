use std::{collections::VecDeque, path::Path};

pub fn run<P>(path: P) -> (usize, usize)
where
    P: AsRef<Path> + std::fmt::Display,
{
    // Annoying helper to be able to run the tests in regular and debug mode (see https://github.com/rust-lang/rust-analyzer/issues/13208)
    let input = if let Ok(input) = std::fs::read_to_string(&path) {
        input
    } else {
        std::fs::read_to_string(format!("aoc23/{path}")).unwrap()
    };

    let mut part2 = 0;
    let mut scratch_copies: VecDeque<_> = VecDeque::new();

    let part1 = input
        // Each scratch card is represented by its own line
        .lines()
        // Split each scratch card into the winning numbers and your numbers
        .map(|l| l.split('|'))
        .map(|mut l| (l.next().unwrap(), l.next().unwrap()))
        // Parse the numbers to be usize
        .map(|(winning_numbers, numbers_you_have)| {
            (
                winning_numbers
                    .split_ascii_whitespace()
                    .skip(2)
                    .map(|n| n.parse().unwrap())
                    .collect::<Vec<usize>>(),
                numbers_you_have
                    .split_ascii_whitespace()
                    .map(|n| n.parse().unwrap()),
            )
        })
        // Count how many of the numbers are winning numbers
        .map(|(winning_numbers, numbers_you_have)| {
            numbers_you_have
                .filter(move |n| winning_numbers.contains(n))
                .count()
                .try_into()
                .unwrap()
        })
        // Keep track of the number of copies
        .inspect(|&c| {
            let no_cards = scratch_copies.pop_front().unwrap_or(0) + 1;
            for num in scratch_copies.iter_mut().take(c) {
                *num += no_cards;
            }
            for i in scratch_copies.len()..(c) {
                scratch_copies.insert(i, no_cards);
            }
            part2 += no_cards;
        })
        // Calculate the score for each card according to the rules of part 1
        .map(|c| {
            if c > 0 {
                2_usize.pow((c - 1).try_into().unwrap())
            } else {
                0
            }
        })
        // Sum up the score to get the solution for part 1
        .sum();

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let (part1, part2) = run("input/examples/4/1.txt");
        assert_eq!(part1, 13);
        assert_eq!(part2, 30);
    }

    #[test]
    fn test_input() {
        let (part1, part2) = run("input/4.txt");
        assert_eq!(part1, 27_845);
        assert_eq!(part2, 9_496_801);
    }
}
