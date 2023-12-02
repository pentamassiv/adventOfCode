use std::path::Path;

pub fn run<P>(path: P) -> (usize, usize)
where
    P: AsRef<Path> + std::fmt::Display,
{
    // Annoying helper to be able to run the tests in regular and debug mode (see https://github.com/rust-lang/rust-analyzer/issues/13208)
    let input = if let Ok(input) = std::fs::read_to_string(&path) {
        input
    } else {
        std::fs::read_to_string(format!("aoc23/{path}")).unwrap()
    };

    let max = [12, 13, 14]; // Max number of 12 red cubes, 13 green cubes, and 14 blue cubes
    let mut mins = [0, 0, 0]; // Min number of cubes of the colors red, green and blue to be able to play a game

    // Read the input file
    input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            mins = [0, 0, 0];
            let mut cube_color = l.split(':').nth(1).unwrap().split(|c| c == ';' || c == ',');
            let cube_color_clone = cube_color.clone();

            let possible_game_no = if cube_color.all(|c| is_possible(c, &max)) {
                i + 1
            } else {
                0
            };
            cube_color_clone.for_each(|c| new_min(c, &mut mins));
            (possible_game_no, mins[0] * mins[1] * mins[2])
        })
        .fold((0, 0), |(sum_a, sum_b), (a, b)| (sum_a + a, sum_b + b))
}

/// Check if the game is possible
fn is_possible(input: &str, max: &[usize]) -> bool {
    let len = input.len();
    // Looking at the last char is enough to know which color it is
    let (no, max) = match input.chars().next_back() {
        Some('d') => (&input[1..len - 4], max[0]), // red
        Some('n') => (&input[1..len - 6], max[1]), // green
        Some('e') => (&input[1..len - 5], max[2]), // blue
        Some(_) | None => {
            panic!("no color detected")
        }
    };
    let no: usize = no.parse().unwrap();
    no <= max
}

/// Calculate the new minimum amount of cubes of the given color for the game to be possible
fn new_min(input: &str, mins: &mut [usize]) {
    let len = input.len();
    let (no, color) = match input.chars().next_back() {
        Some('d') => (&input[1..len - 4], 0), // red
        Some('n') => (&input[1..len - 6], 1), // green
        Some('e') => (&input[1..len - 5], 2), // blue
        Some(_) | None => {
            panic!("no color detected")
        }
    };
    let no: usize = no.parse().unwrap();

    mins[color] = std::cmp::max(no, mins[color]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let (part1, part2) = run("input/examples/2/1.txt");
        assert_eq!(part1, 8);
        assert_eq!(part2, 2_286);
    }

    #[test]
    fn test_input() {
        let (part1, part2) = run("input/2.txt");
        assert_eq!(part1, 1_867);
        assert_eq!(part2, 84_538);
    }
}
