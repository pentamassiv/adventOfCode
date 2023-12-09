use std::path::Path;

pub fn run<P>(path: P) -> (isize, usize)
where
    P: AsRef<Path> + std::fmt::Display,
{
    // Annoying helper to be able to run the tests in regular and debug mode (see https://github.com/rust-lang/rust-analyzer/issues/13208)
    let input = if let Ok(input) = std::fs::read_to_string(&path) {
        input
    } else {
        std::fs::read_to_string(format!("aoc23/{path}")).unwrap()
    };

    let sequence_len = input
        .lines()
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .count();

    // Prepare the vec to store the diffs
    let mut diffs = vec![];
    for i in (1..=sequence_len).rev() {
        diffs.push(vec![0; i]);
    }
    diffs.shrink_to_fit();

    let mut part1 = 0;
    input.lines().for_each(|l| {
        l.split_ascii_whitespace()
            // .inspect(|i| print!("{i}"))
            .map(|i| i.parse::<isize>().unwrap())
            .enumerate()
            .for_each(|(i, v)| diffs[0][i] = v);

        let last_row = calc_diffs(&mut diffs);

        part1 += diffs
            .iter()
            .map(|v| v.last().unwrap())
            .take(last_row)
            .sum::<isize>();
    });

    print!("");
    (part1, 0)
}

fn calc_diffs(input: &mut [Vec<isize>]) -> usize {
    for row in 0..input.len() - 1 {
        for column in 0..input[row].len() - 1 {
            input[row + 1][column] = input[row][column + 1] - input[row][column];
        }
        if input[row + 1].iter().all(|&v| v == 0) {
            return row + 1;
        }
    }
    input.len() - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let (part1, part2) = run("input/examples/9/1.txt");
        assert_eq!(part1, 114);
        assert_eq!(part2, 2);
    }

    #[test]
    fn test_input() {
        let (part1, part2) = run("input/9.txt");
        assert_eq!(part1, 2_008_960_228);
        assert_eq!(part2, 246_894_760);
    }
}
