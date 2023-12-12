use std::{path::Path, str::Chars};

use itertools::{Itertools, MultiProduct};

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

    let part1 = input
        .lines()
        .map(|l| {
            let mut l = l.split_ascii_whitespace();
            (l.next().unwrap(), l.next().unwrap())
        })
        .map(|(r, n)| (r.chars(), n.split(',').map(|n| n.parse::<usize>().unwrap())))
        .map(|(r, n)| (possible_records(r), n))
        .map(|(r, n)| (r.collect::<Vec<_>>(), n.collect::<Vec<_>>()))
        .map(|(r, n)| valid_record(&r, n))
        .inspect(|i| println!("{i}"))
        .sum();

    /*
    let part2 = input
        .lines()
        .map(|l| {
            let mut l = l.split_ascii_whitespace();
            (l.next().unwrap(), l.next().unwrap())
        })
        .map(|(r, n)| (r.chars(), n.split(',').map(|n| n.parse::<usize>().unwrap())))
        .map(|(r, n)| (possible_records(r), n))
        .map(|(r, n)| {
            (
                r.collect::<Vec<_>>(),
                n.clone()
                    .chain(n.clone())
                    .chain(n.clone())
                    .chain(n.clone())
                    .chain(n)
                    .collect::<Vec<_>>(),
            )
        })
        .map(|(r, n)| valid_record(&r, n))
        // .inspect(|i| println!("{i}"))
        .sum();*/

    (part1, 0)
}

// Generate all records that are theoretically possible by replacing the ? with both options (. and #)
fn possible_records(c: Chars) -> MultiProduct<std::vec::IntoIter<char>> {
    c.map(|c| {
        if c == '?' {
            vec!['.', '#'].into_iter()
        } else {
            vec![c].into_iter()
        }
    })
    .multi_cartesian_product()
}

// Takes the list of possible records and the number definition as input
// Returns the number of valid possible records
fn valid_record(records: &[Vec<char>], numbers: Vec<usize>) -> usize {
    let mut valid_records = 0;
    let mut previous;
    let mut numbers_iter;
    let mut no;
    let mut temp_no;

    'rec: for record in records {
        previous = '.';
        numbers_iter = numbers.iter().cloned();
        no = numbers_iter.next();
        for current in record {
            match (previous, current) {
                ('#', '.') => {
                    if no != None {
                        continue 'rec;
                    } else {
                        previous = '.';
                        no = numbers_iter.next();

                        // this is expected
                    }
                }
                ('.', '.') => {}
                ('#', '#') | ('.', '#') => {
                    previous = '#';
                    if let Some(n) = no {
                        temp_no = n - 1;
                        if temp_no == 0 {
                            no = None;
                        } else {
                            no = Some(temp_no);
                        }
                    } else {
                        continue 'rec;
                    }
                }
                _ => {
                    panic!("should never happen")
                }
            }
        }
        if no.is_none() && numbers_iter.next().is_none() {
            valid_records += 1;
        }
    }
    valid_records
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let (part1, part2) = run("input/examples/12/1.txt");
        assert_eq!(part1, 21);
        assert_eq!(part2, 525_152);
    }

    #[test]
    fn test_input() {
        let (part1, part2) = run("input/12.txt");
        assert_eq!(part1, 7_622);
        assert_eq!(part2, 746_207_878_188);
    }
}
