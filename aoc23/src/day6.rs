use std::{path::Path, vec};

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

    let top_row = &input[..input.len() / 2];
    let bottom_row = &input[input.len() / 2..];

    // Part 1: The input are pairs of times and distances travelled
    let times_part1 = top_row
        .split_ascii_whitespace()
        .skip(1)
        .map(|n| n.parse::<f64>().unwrap());
    let distances_part1 = bottom_row
        .split_ascii_whitespace()
        .skip(1)
        .map(|n| n.parse::<f64>().unwrap());

    // Part 2: The kerning was wrong, it is just one race, ignore the whitespace between numbers
    let times_part2 = top_row[11..]
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect::<String>();
    let times_part2 = vec![times_part2.parse::<f64>().unwrap()].into_iter();
    let distances_part2 = bottom_row[11..]
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect::<String>();
    let distances_part2 = vec![distances_part2.parse::<f64>().unwrap()].into_iter();

    let part1 = calculate_posibilities(times_part1, distances_part1);
    let part2 = calculate_posibilities(times_part2, distances_part2);

    (part1, part2)
}

#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_possible_truncation)]
fn calculate_posibilities<I: Iterator<Item = f64>, J: Iterator<Item = f64>>(
    times: I,
    distances: J,
) -> usize {
    times
        .zip(distances)
        .map(|(t, s)| {
            let dif = f64::sqrt(t.powi(2) / 4.0 - s);
            (t / 2.0 - dif, t / 2.0 + dif)
        })
        .map(|(a, b)| ((b.ceil() as usize) - (a.floor() as usize + 1)))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let (part1, part2) = run("input/examples/6/1.txt");
        assert_eq!(part1, 288);
        assert_eq!(part2, 71_503);
    }

    #[test]
    fn test_input() {
        let (part1, part2) = run("input/6.txt");
        assert_eq!(part1, 2_756_160);
        assert_eq!(part2, 34_788_142);
    }
}

/*

s < v * t
s < w * (t-w)
s < tw - w²
0 < -w² +tw - s
0 > w² -tw + s

 t/2 - root(t²/4 - s)< x < t/2 + root(t²/4 - s)

s=9 t=7:    7/2 - root(7²/4 - 9)< x < 7/2 + root(7²/4 - 9)



*/
