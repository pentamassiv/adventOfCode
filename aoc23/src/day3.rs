use std::{cmp::min, collections::HashMap, path::Path};

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
    let line_len = input.find('\n').unwrap() + 1; // Number of chars in each line

    let mut part_sum = 0; // Sum of all part numbers
    let mut part_no: Option<(usize, usize)> = None; // Index of the beginning and end of a new part number

    let mut gears: HashMap<usize, Vec<usize>> = HashMap::new(); // List of potential gears (position in input -> machine_no)

    for (i, c) in input.chars().enumerate() {
        if c.is_ascii_digit() {
            part_no = Some((part_no.unwrap_or((i, i)).0, i + 1));
            continue;
        }
        if let Some((start, end)) = part_no {
            let machine_no = input[start..end].parse::<usize>().unwrap();
            if surrounded_by_symbol(&input, (start, end), line_len) {
                part_sum += machine_no;
            }
            for gear in potential_gears(&input, (start, end), line_len) {
                if let Some(x) = gears.get_mut(&gear) {
                    x.push(machine_no);
                } else {
                    gears.insert(gear, vec![machine_no]);
                }
            }

            part_no = None;
        }
    }

    // A gear is any * symbol that is adjacent to EXACTLY two part numbers
    gears.retain(|_, v| v.len() == 2);

    // Its gear ratio is the result of multiplying those two numbers together.
    let part2: usize = gears
        .values()
        .map(|machine_nos| machine_nos.iter().product::<usize>())
        .sum();

    (part_sum, part2)
}

/// Check there is a symbol somewhere next to the machine number (it only is a valid machine number if there is)
fn surrounded_by_symbol(input: &str, place: (usize, usize), line_len: usize) -> bool {
    let row_above =
        &input[place.0.saturating_sub(line_len + 1)..place.1.saturating_sub(line_len - 1)];
    let row_below =
        &input[min(place.0 + line_len - 1, input.len())..min(place.1 + line_len + 1, input.len())];
    let left = &input[place.0.saturating_sub(1)..place.0];
    let right = &input[min(place.1, input.len())..min(place.1 + 1, input.len())];

    row_above
        .chars()
        .chain(row_below.chars())
        .chain(left.chars())
        .chain(right.chars())
        .any(|c| c.is_ascii_punctuation() && c != '.')
}

/// Return all potential gears surrounding the machine number
fn potential_gears(input: &str, place: (usize, usize), line_len: usize) -> Vec<usize> {
    let row_above = place.0.saturating_sub(line_len + 1)..place.1.saturating_sub(line_len - 1);
    let row_below =
        min(place.0 + line_len - 1, input.len())..min(place.1 + line_len + 1, input.len());
    let left = place.0.saturating_sub(1)..place.0;
    let right = min(place.1, input.len())..min(place.1 + 1, input.len());

    row_above
        .chain(row_below)
        .chain(left)
        .chain(right)
        .filter(|&i| &input[i..=i] == "*")
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let (part1, part2) = run("input/examples/3/1.txt");
        assert_eq!(part1, 4_361);
        assert_eq!(part2, 467_835);
    }

    #[test]
    fn test_input() {
        let (part1, part2) = run("input/3.txt");
        assert_eq!(part1, 553_825);
        assert_eq!(part2, 93_994_191);
    }
}
