use std::{collections::HashSet, path::Path};

pub fn run<P>(path: P) -> (usize, i32)
where
    P: AsRef<Path>,
{
    let input = std::fs::read_to_string(path).unwrap();
    let mut visited: HashSet<_> = input
        .chars()
        .scan((0, 0), |(x, y), movement| match movement {
            '<' => Some((*x - 1, *y)),
            '>' => Some((*x + 1, *y)),
            '^' => Some((*x + 1, *y + 1)),
            'v' => Some((*x, *y - 1)),
            _ => Some((*x, *y)),
        })
        .collect();
    visited.insert((0, 0));
    (visited.len(), 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let (part1, part2) = run("input/examples/3/1.txt");
        assert_eq!(part1, 24000);
        assert_eq!(part2, 45000);
    }

    #[test]
    fn test_input() {
        let (part1, part2) = run("input/3.txt");
        assert_eq!(part1, 75_622);
        assert_eq!(part2, 213_159);
    }
}
