use regex::Regex;
use std::path::Path;

pub fn run<P>(path: P) -> (u32, u32)
where
    P: AsRef<Path> + std::fmt::Display,
{
    // Annoying helper to be able to run the tests in regular and debug mode (see https://github.com/rust-lang/rust-analyzer/issues/13208)
    let input = if let Ok(input) = std::fs::read_to_string(&path) {
        input
    } else {
        std::fs::read_to_string(format!("aoc23/{path}")).unwrap()
    };

    let re = Regex::new(&r"[[:digit:]]|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let re_rev = Regex::new(&r"[[:digit:]]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();

    // Read the input file
    input
        .lines()
        .map(|l| {
            let l_rev: String = l.chars().rev().collect();
            (
                l.chars()
                    .filter(char::is_ascii_digit)
                    .next()
                    .unwrap_or('0')
                    .to_digit(10)
                    .unwrap()
                    * 10
                    + l_rev
                        .chars()
                        .filter(char::is_ascii_digit)
                        .next()
                        .unwrap_or('0')
                        .to_digit(10)
                        .unwrap(),
                parse_str(re.find(l).unwrap().as_str()) * 10
                    + parse_str(re_rev.find(&l_rev).unwrap().as_str()),
            )
        })
        .fold((0, 0), |(sum_a, sum_b), (a, b)| (sum_a + a, sum_b + b))
}

fn parse_str(input: &str) -> u32 {
    match input {
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "eno" => 1,
        "owt" => 2,
        "eerht" => 3,
        "ruof" => 4,
        "evif" => 5,
        "xis" => 6,
        "neves" => 7,
        "thgie" => 8,
        "enin" => 9,
        _ => {
            panic!("unable to parse str")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_re() {
        let re = Regex::new(&r"[[:digit:]]|one|two|three|four|five|six|seven|eight|nine").unwrap();
        let re_rev =
            Regex::new(&r"[[:digit:]]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();

        let l = "qzjggk1one";
        let l_rev: String = l.chars().rev().collect();
        let mut line_res = parse_str(re.find(l).unwrap().as_str()) * 10;
        line_res += parse_str(re_rev.find(&l_rev).unwrap().as_str());

        assert_eq!(line_res, 11);
    }

    #[test]
    fn test_example() {
        let (part1, part2) = run("input/examples/1/1.txt");
        assert_eq!(part1, 142);
        assert_eq!(part2, 142);
    }

    #[test]
    fn test_example2() {
        let (part1, part2) = run("input/examples/1/2.txt");
        assert_eq!(part1, 209);
        assert_eq!(part2, 281);
    }

    #[test]
    fn test_input() {
        let (part1, part2) = run("input/1.txt");
        assert_eq!(part1, 55_172);
        assert_eq!(part2, 54_925);
    }
}
