use std::path::Path;

pub fn run<P>(path: P) -> (i32, i32)
where
    P: AsRef<Path> + std::fmt::Display,
{
    // Annoying helper to be able to run the tests in regular and debug mode (see https://github.com/rust-lang/rust-analyzer/issues/13208)
    let input = if let Ok(input) = std::fs::read_to_string(&path) {
        input
    } else {
        std::fs::read_to_string(format!("aoc15/{path}")).unwrap()
    };
    for line in input.lines() {}
    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let (part1, part2) = run("input/examples/8/1.txt");
        assert_eq!(part1, 24000);
        assert_eq!(part2, 45000);
    }

    #[test]
    fn test_input() {
        let (part1, part2) = run("input/8.txt");
        assert_eq!(part1, 75_622);
        assert_eq!(part2, 213_159);
    }
}
