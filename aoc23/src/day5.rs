use std::path::Path;

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

    let end_first_line = input.find('\n').unwrap();

    // The first source are the seeds
    let mut sources_part1: Vec<_> = input[..end_first_line]
        .split_ascii_whitespace()
        .skip(1)
        .map(|n| n.parse::<isize>().unwrap())
        .collect();
    let mut sources_part2 = vec![];
    for i in (0..sources_part1.len()).step_by(2) {
        let seed_start = sources_part1[i];
        for j in 1..sources_part1[i + 1] {
            sources_part2.push(seed_start + j);
        }
    }

    let mut destinations_1 = vec![];
    let mut destinations_2 = vec![];

    for line in input[end_first_line + 2..]
        .lines()
        // get rid of empty lines
        .filter(|&l| !l.is_empty())
    {
        if line.chars().next().unwrap().is_ascii_digit() {
            let iter: Vec<_> = line
                .split_ascii_whitespace()
                .map(|n| n.parse::<isize>().unwrap())
                .take(3)
                .collect();
            let (dest, source, len) = (iter[0], iter[1], iter[2]);

            let mut mapped_1 = sources_part1
                .extract_if(|x| *x >= source && *x <= source + len)
                .collect::<Vec<_>>();
            let mut mapped_2 = sources_part2
                .extract_if(|x| *x >= source && *x <= source + len)
                .collect::<Vec<_>>();

            let difference = dest - source;
            for e in &mut mapped_1 {
                *e += difference;
            }
            for e in &mut mapped_2 {
                *e += difference;
            }

            destinations_1.append(&mut mapped_1);
            destinations_2.append(&mut mapped_2);
        } else {
            println!("Next header");
            sources_part1.append(&mut destinations_1);
            sources_part2.append(&mut destinations_2);
        }
    }
    sources_part1.append(&mut destinations_1);
    sources_part2.append(&mut destinations_2);
    let part1 = *sources_part1.iter().min().unwrap();
    let part2 = *sources_part2.iter().min().unwrap();

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let (part1, part2) = run("input/examples/5/1.txt");
        assert_eq!(part1, 35);
        assert_eq!(part2, 46);
    }

    #[test]
    fn test_input() {
        let (part1, part2) = run("input/5.txt");
        assert_eq!(part1, 484_023_871);
        assert_eq!(part2, 46_294_175);
    }
}
