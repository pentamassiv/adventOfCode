pub fn run(path: &str) -> (usize, usize) {
    // Read the input file
    let input = std::fs::read_to_string(path).unwrap();

    let mut solution = (0, 0);
    solution.0 = find_idx_n_distinct_chars(input.as_bytes(), 4).unwrap_or(input.len() + 1);
    solution.1 = find_idx_n_distinct_chars(input.as_bytes(), 14).unwrap_or(input.len() + 1);

    (solution.0, solution.1)
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
