use std::path::Path;

use itertools::Itertools;

pub fn run<P>(path: P) -> (isize, isize)
where
    P: AsRef<Path> + std::fmt::Display,
{
    // Annoying helper to be able to run the tests in regular and debug mode (see https://github.com/rust-lang/rust-analyzer/issues/13208)
    let input = if let Ok(input) = std::fs::read_to_string(&path) {
        input
    } else {
        std::fs::read_to_string(format!("aoc23/{path}")).unwrap()
    };
    let part1 = sum_galaxy_pairs(&input, 2);
    let part2 = sum_galaxy_pairs(&input, 1_000_000);

    (part1, part2)
}

// Calculate the shortest distance between all pairs of galaxies described in the input
// Empty rows and columns get stretched by the given factor
fn sum_galaxy_pairs(input: &str, stretch: usize) -> isize {
    let mut galaxies = vec![];
    let mut empty_rows = 0;
    let mut is_empty = true;

    // Read the input and store the coordinates (x,y) of the galaxies in a vec. Adjust the y coordinate by the number of empty rows and how much the universe expands for the empty rows
    input
        .lines()
        .map(|l| l.chars().enumerate())
        .enumerate()
        .for_each(|(y, l)| {
            is_empty = true;
            l.for_each(|(x, e)| {
                if e == '#' {
                    galaxies.push((x, y + empty_rows));
                    is_empty = false;
                }
            });
            if is_empty {
                empty_rows += stretch - 1;
            }
        });
    galaxies.sort_unstable();

    // Adjust the x coordinate by the number of empty columns and how much the universe expands for the empty columns
    let mut empty_columns = 0;
    galaxies
        .iter()
        .scan((0, (0, 0)), |columns, x| {
            columns.0 = columns.1 .0;
            columns.1 = *x;
            Some(*columns)
        })
        .map(|(prev_column, (x, y))| {
            empty_columns += (stretch - 1) * x.saturating_sub(prev_column + 1);
            (x + empty_columns, y)
        })
        // take all pairs of galaxies
        .combinations(2)
        // change their type to ((isize, isize), (isize, isize))
        .map(|e| {
            (
                (e[0].0.try_into().unwrap(), e[0].1.try_into().unwrap()),
                (e[1].0.try_into().unwrap(), e[1].1.try_into().unwrap()),
            )
        })
        // calculate the shortest distance between a pair of galaxies
        .map(|(a, b): ((isize, isize), (isize, isize))| {
            isize::abs(a.0 - b.0) + isize::abs(a.1 - b.1)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        // Annoying helper to be able to run the tests in regular and debug mode (see https://github.com/rust-lang/rust-analyzer/issues/13208)
        let input = if let Ok(input) = std::fs::read_to_string(&"input/examples/11/1.txt") {
            input
        } else {
            std::fs::read_to_string(format!("aoc23/input/examples/11/1.txt")).unwrap()
        };

        let stretch2 = sum_galaxy_pairs(&input, 2);
        let stretch10 = sum_galaxy_pairs(&input, 10);
        let stretch100 = sum_galaxy_pairs(&input, 100);
        let stretch1000000 = sum_galaxy_pairs(&input, 1_000_000);

        assert_eq!(stretch2, 374); // 2 times
        assert_eq!(stretch10, 1_030); // ten times
        assert_eq!(stretch100, 8_410); // 100 times
        assert_eq!(stretch1000000, 82_000_210); // 1_000_000 times
    }

    #[test]
    fn test_input() {
        let (part1, part2) = run("input/11.txt");
        assert_eq!(part1, 9_370_588);
        assert_eq!(part2, 746_207_878_188);
    }
}
