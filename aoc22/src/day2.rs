#[allow(clippy::identity_op)]
pub fn run(path: &str) -> (i32, i32) {
    // Read the input file
    let input = std::fs::read_to_string(path).unwrap();

    let mut score = (0, 0); // Keeps the score for part 1 & part 2

    for line in input.lines() {
        score = match line {
            "A X" => (score.0 + 1 + 3, score.1 + 0 + 3),
            "A Y" => (score.0 + 2 + 6, score.1 + 3 + 1),
            "A Z" => (score.0 + 3 + 0, score.1 + 6 + 2),

            "B X" => (score.0 + 1 + 0, score.1 + 0 + 1),
            "B Y" => (score.0 + 2 + 3, score.1 + 3 + 2),
            "B Z" => (score.0 + 3 + 6, score.1 + 6 + 3),

            "C X" => (score.0 + 1 + 6, score.1 + 0 + 2),
            "C Y" => (score.0 + 2 + 0, score.1 + 3 + 3),
            "C Z" => (score.0 + 3 + 3, score.1 + 6 + 1),
            &_ => (score.0 + 0, score.1 + 0),
        }
    }

    (score.0, score.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let (part1, part2) = run("input/examples/2/1.txt");
        assert_eq!(part1, 15);
        assert_eq!(part2, 12);
    }

    #[test]
    fn test_input() {
        let (part1, part2) = run("input/2.txt");
        assert_eq!(part1, 12_586);
        assert_eq!(part2, 13_193);
    }
}
