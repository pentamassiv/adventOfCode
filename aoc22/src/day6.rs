use std::collections::HashSet;

#[must_use]
pub fn run(path: &str) -> (usize, usize) {
    // Read the input file
    let input = std::fs::read_to_string(path).unwrap();

    let solution1 = find_marker(input.as_bytes(), 4);
    let solution2 = find_marker(input.as_bytes(), 14);

    (solution1, solution2)
}

fn find_marker(input: &[u8], n_distinct: usize) -> usize {
    input
        .windows(n_distinct)
        .position(|x| x.iter().copied().collect::<HashSet<u8>>().len() == n_distinct)
        .unwrap_or(input.len() + 1)
        + n_distinct
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let (part1, part2) = run("input/examples/6/1.txt");
        assert_eq!(part1, 7);
        assert_eq!(part2, 19);
    }

    #[test]
    fn test_example2() {
        let (part1, part2) = run("input/examples/6/2.txt");
        assert_eq!(part1, 5);
        assert_eq!(part2, 23);
    }

    #[test]
    fn test_example3() {
        let (part1, part2) = run("input/examples/6/3.txt");
        assert_eq!(part1, 6);
        assert_eq!(part2, 23);
    }

    #[test]
    fn test_example4() {
        let (part1, part2) = run("input/examples/6/4.txt");
        assert_eq!(part1, 10);
        assert_eq!(part2, 29);
    }

    #[test]
    fn test_example5() {
        let (part1, part2) = run("input/examples/6/5.txt");
        assert_eq!(part1, 11);
        assert_eq!(part2, 26);
    }

    #[test]
    fn test_example6() {
        let (part1, part2) = run("input/examples/6/6.txt");
        assert_eq!(part1, 7);
        assert_eq!(part2, 19);
    }

    #[test]
    fn test_example7() {
        let (part1, part2) = run("input/examples/6/7.txt");
        assert_eq!(part1, 5);
        assert_eq!(part2, 23);
    }

    #[test]
    fn test_example8() {
        let (part1, part2) = run("input/examples/6/8.txt");
        assert_eq!(part1, 6);
        assert_eq!(part2, 23);
    }

    #[test]
    fn test_example9() {
        let (part1, part2) = run("input/examples/6/9.txt");
        assert_eq!(part1, 10);
        assert_eq!(part2, 29);
    }

    #[test]
    fn test_example10() {
        let (part1, part2) = run("input/examples/6/10.txt");
        assert_eq!(part1, 11);
        assert_eq!(part2, 26);
    }

    #[test]
    fn test_input() {
        let (part1, part2) = run("input/6.txt");
        assert_eq!(part1, 1848);
        assert_eq!(part2, 2308);
    }
}
