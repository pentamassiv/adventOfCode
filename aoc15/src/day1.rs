use std::path::Path;

pub fn run<P>(path: P) -> (i32, Option<usize>)
where
    P: AsRef<Path>,
{
    // Read the input file
    let mut solution1: i32 = 0;
    let mut solution2 = None;
    std::fs::read_to_string(path)
        .unwrap()
        .chars()
        .enumerate()
        .inspect(|(no, x)| {
            if *x == '(' {
                solution1 += 1;
            } else {
                solution1 -= 1;
            }
            if solution1 < 0 && solution2.is_none() {
                solution2 = Some(*no + 1);
            }
        })
        .last()
        .unwrap();

    (solution1, solution2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let (part1, part2) = run("input/examples/1/1.txt");
        assert_eq!(part1, 0);
        assert_eq!(part2, None);
        let (part1, part2) = run("input/examples/1/2.txt");
        assert_eq!(part1, 0);
        assert_eq!(part2, None);
        let (part1, part2) = run("input/examples/1/3.txt");
        assert_eq!(part1, 3);
        assert_eq!(part2, None);
        let (part1, part2) = run("input/examples/1/4.txt");
        assert_eq!(part1, 3);
        assert_eq!(part2, None);
        let (part1, part2) = run("input/examples/1/5.txt");
        assert_eq!(part1, 3);
        assert_eq!(part2, Some(1));
        let (part1, part2) = run("input/examples/1/6.txt");
        assert_eq!(part1, -1);
        assert_eq!(part2, Some(3));
        let (part1, part2) = run("input/examples/1/7.txt");
        assert_eq!(part1, -1);
        assert_eq!(part2, Some(1));
        let (part1, part2) = run("input/examples/1/8.txt");
        assert_eq!(part1, -3);
        assert_eq!(part2, Some(1));
        let (part1, part2) = run("input/examples/1/9.txt");
        assert_eq!(part1, -3);
        assert_eq!(part2, Some(1));
        let (part1, part2) = run("input/examples/1/10.txt");
        assert_eq!(part1, -1);
        assert_eq!(part2, Some(1));
        let (part1, part2) = run("input/examples/1/11.txt");
        assert_eq!(part1, -1);
        assert_eq!(part2, Some(5));
    }

    #[test]
    fn test_input() {
        let (part1, part2) = run("input/1.txt");
        assert_eq!(part1, 138);
        assert_eq!(part2, Some(1771));
    }
}
