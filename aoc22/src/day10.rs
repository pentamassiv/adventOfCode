#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_possible_truncation)]

use std::path::Path;

pub fn run<P>(path: P) -> (i32, i32)
where
    P: AsRef<Path> + std::fmt::Display,
{
    // Annoying helper to be able to run the tests in regular and debug mode (see https://github.com/rust-lang/rust-analyzer/issues/13208)
    let input = if let Ok(input) = std::fs::read_to_string(&path) {
        input
    } else {
        std::fs::read_to_string(format!("aoc22/{path}")).unwrap()
    };
    let mut cycle_values = vec![0];
    input.lines().for_each(|l| {
        if let Some(no) = l.strip_prefix("addx ") {
            cycle_values.push(0);
            cycle_values.push(no.parse::<i32>().unwrap_or(0));
        } else {
            cycle_values.push(0);
        }
    });
    let cpu_cycles = cycle_values
        .iter()
        .scan(1, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect::<Vec<_>>();

    let solution1 = 20 * cpu_cycles[19]
        + 60 * cpu_cycles[59]
        + 100 * cpu_cycles[99]
        + 140 * cpu_cycles[139]
        + 180 * cpu_cycles[179]
        + 220 * cpu_cycles[219];

    let mut sprite;
    let mut pixel;

    for (cycle_no, cycle) in cpu_cycles.iter().enumerate() {
        sprite = *cycle - 1;
        pixel = (cycle_no % 40) as i32;
        if pixel == sprite || pixel == sprite + 1 || pixel == sprite + 2 {
            print!("#");
        } else {
            print!(".");
        }
        if pixel == 39 {
            println!();
        }
    }

    (solution1, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        //let (part1, part2) = run("input/examples/10/1.txt");
        //assert_eq!(part1, 24000);
        //assert_eq!(part2, 45000);
        let (part1, _) = run("input/examples/10/2.txt");
        assert_eq!(part1, 13140);
        //assert_eq!(part2, 45000);
    }

    #[test]
    fn test_input() {
        let (part1, _) = run("input/10.txt");
        assert!(part1 < 14360);
        //assert_eq!(part2, 213_159);
    }
}
