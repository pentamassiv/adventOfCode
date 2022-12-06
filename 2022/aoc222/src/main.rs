#[allow(clippy::identity_op)]

fn main() {
    // Read the input file
    let input = std::fs::read_to_string("2022/aoc222/input").unwrap();

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
    println!("Part 1:");
    println!("{}", score.0);
    println!("Part 2:");
    println!("{}", score.1);
}
