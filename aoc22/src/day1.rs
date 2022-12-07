use std::path::Path;

pub fn run<P>(path: P) -> (i32, i32)
where
    P: AsRef<Path>,
{
    // Read the input file
    let mut calories: Vec<i32> = std::fs::read_to_string(path)
        .unwrap()
        .split("\n\n")
        .map(calculate_calories_of_one_elf)
        .collect();
    calories.sort_unstable();

    (
        calories.iter().rev().take(1).sum::<i32>(),
        calories.iter().rev().take(3).sum::<i32>(),
    )
}

fn calculate_calories_of_one_elf(lines_of_one_elf: &str) -> i32 {
    lines_of_one_elf
        .lines()
        .filter_map(|str| str.parse::<i32>().ok())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let (part1, part2) = run("input/examples/1/1.txt");
        assert_eq!(part1, 24000);
        assert_eq!(part2, 45000);
    }

    #[test]
    fn test_input() {
        let (part1, part2) = run("input/1.txt");
        assert_eq!(part1, 75_622);
        assert_eq!(part2, 213_159);
    }
}
