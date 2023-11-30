#![allow(clippy::char_lit_as_u8)]
use std::cmp::Ordering;
use std::{path::Path, str::Chars};

pub fn run<P>(path: P) -> (usize, usize)
where
    P: AsRef<Path> + std::fmt::Display,
{
    // Annoying helper to be able to run the tests in regular and debug mode (see https://github.com/rust-lang/rust-analyzer/issues/13208)
    let input = if let Ok(input) = std::fs::read_to_string(&path) {
        input
    } else {
        std::fs::read_to_string(format!("aoc22/{path}")).unwrap()
    };

    let lines = input
        .lines()
        .filter(|l| !l.is_empty())
        .array_chunks::<2>()
        .collect::<Vec<_>>();
    let solution1 = lines
        .iter()
        .enumerate()
        .filter_map(|(no, [first, second])| {
            if cmp(&mut first.chars(), &mut second.chars()) == Ordering::Less {
                Some(no + 1)
            } else {
                None
            }
        })
        .sum();
    let mut all_packets = input.lines().filter(|l| !l.is_empty()).collect::<Vec<_>>();
    all_packets.push("[[2]]");
    all_packets.push("[[6]]");
    all_packets.sort_by(|a, b| cmp(&mut a.chars(), &mut b.chars()));
    let solution2 = all_packets
        .iter()
        .enumerate()
        .filter_map(|(no, &x)| {
            if x == "[[2]]" || x == "[[6]]" {
                Some(no + 1)
            } else {
                None
            }
        })
        .product();

    (solution1, solution2)
}

fn cmp(first: &mut Chars, second: &mut Chars) -> Ordering {
    let mut first = first.peekable();
    let mut second = second.peekable();
    let mut wrapped = (0, 0);
    loop {
        let Some(mut first_c) = first.next() else {
            return Ordering::Less;
        };
        let Some(mut second_c) = second.next() else {
            return Ordering::Greater;
        };
        if first_c == second_c {
            continue;
        }

        if first_c == ']' {
            return Ordering::Less;
        }
        if second_c == ']' {
            return Ordering::Greater;
        }

        while first_c == '[' {
            wrapped.0 += 1;
            first_c = first.next().unwrap();
        }
        while second_c == '[' {
            wrapped.1 += 1;
            second_c = second.next().unwrap();
        }

        let Some(mut first_value) = first_c.to_digit(10) else {
            return Ordering::Less;
        };
        let Some(mut second_value) = second_c.to_digit(10) else {
            return Ordering::Greater;
        };

        while let Some(digit) = first.next_if(char::is_ascii_digit) {
            first_value *= 10;
            first_value += digit.to_digit(10).unwrap();
        }

        while let Some(digit) = second.next_if(char::is_ascii_digit) {
            second_value *= 10;
            second_value += digit.to_digit(10).unwrap();
        }

        let comparison = first_value.cmp(&second_value);
        if comparison != Ordering::Equal {
            return comparison;
        }
        while first.next_if(|&x| wrapped.0 > 0 && x == ']').is_some() {
            wrapped.0 -= 1;
        }
        if wrapped.0 > 0 {
            return Ordering::Greater;
        }

        while second.next_if(|&x| wrapped.1 > 0 && x == ']').is_some() {
            wrapped.1 -= 1;
        }
        if wrapped.1 > 0 {
            return Ordering::Less;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let (part1, part2) = run("input/examples/13/1.txt");
        assert_eq!(part1, 13);
        assert_eq!(part2, 140);
    }

    #[test]
    fn test_input() {
        let (part1, part2) = run("input/13.txt");
        //println!("{part1}");
        assert_eq!(part1, 5808);
        assert_eq!(part2, 22_713);
    }

    #[test]
    fn test_from_fn() {
        let mut left = "[1,1,3,1,1]".chars();
        let mut right = "[1,1,5,1,1]".chars();
        assert_eq!(cmp(&mut left, &mut right), Ordering::Less);

        let mut left = "[[1],[2,3,4]]".chars();
        let mut right = "[[1],4]".chars();
        assert_eq!(cmp(&mut left, &mut right), Ordering::Less);

        let mut left = "[9]".chars();
        let mut right = "[[8,7,6]]".chars();
        assert_eq!(cmp(&mut left, &mut right), Ordering::Greater);

        let mut left = "[[4,4],4,4]".chars();
        let mut right = "[[4,4],4,4,4]".chars();
        assert_eq!(cmp(&mut left, &mut right), Ordering::Less);

        let mut left = "[7,7,7,7]".chars();
        let mut right = "[7,7,7]".chars();
        assert_eq!(cmp(&mut left, &mut right), Ordering::Greater);

        let mut left = "[]".chars();
        let mut right = "[3]".chars();
        assert_eq!(cmp(&mut left, &mut right), Ordering::Less);

        let mut left = "[[[]]]".chars();
        let mut right = "[[]]".chars();
        assert_eq!(cmp(&mut left, &mut right), Ordering::Greater);

        let mut left = "[1,[2,[3,[4,[5,6,7]]]],8,9]".chars();
        let mut right = "[1,[2,[3,[4,[5,6,0]]]],8,9]".chars();
        assert_eq!(cmp(&mut left, &mut right), Ordering::Greater);

        let mut left = "[[1,[[5]],[4,1,[]],10,[4,[2,6,5,4,2]]]]".chars();
        let mut right = "[[[],5,[],5],[8,3,4,6,7],[1,[]]]".chars();
        assert_eq!(cmp(&mut left, &mut right), Ordering::Greater);

        let mut left = "[[[[3,0,4,4],9,2],8,[[1],3,[2,0,9,3],[5,3,6,4,5]]],[],[]]".chars();
        let mut right = "[[[],[]],[[[8,4],[5,5,9,9],[1,9,1,8],0]]]".chars();
        assert_eq!(cmp(&mut left, &mut right), Ordering::Greater);

        let mut left =
            "[[[[9],[8]],[6,3,4],5],[[6,3],[7],[[5,10]],10],[],[3,5,[3,0,[0,3,3]],8,[]]]".chars();
        let mut right = "[[3,[10,10],4],[8,[[7,1,0,6]],1],[8,[[5,0,7,4],[1,4],1],5,[[7,1],[2],[0,9,5,7,5]]],[[5,[6,3],[0,8,3,2],10,[10]]]]".chars();
        assert_eq!(cmp(&mut left, &mut right), Ordering::Greater); // Should this be correct?

        // Input pair 35
        let mut left =
            "[[3,6,4,[2,5,0,[8,6,8],4],[[1],[5]]],[0,[2]],[],[5,[],[5,4,[10,9,4],9,6]]]".chars();
        let mut right = "[[[[5,10,3],6,[2],[]],[[8,1,7,7,10],0,[6,8,9,3,9],[10,5,4],[9]],[[]],[[6,1,4,3,9],[10,9,9]],8],[9,7,6,[[3,8,0,1,1],1],8]]".chars();
        assert_eq!(cmp(&mut left, &mut right), Ordering::Less);

        // Input similar to pair 35
        let mut left =
            "[[8,6,4,[2,5,0,[8,6,8],4],[[1],[5]]],[0,[2]],[],[5,[],[5,4,[10,9,4],9,6]]]".chars();
        let mut right = "[[[[5,10,3],6,[2],[]],[[8,1,7,7,10],0,[6,8,9,3,9],[10,5,4],[9]],[[]],[[6,1,4,3,9],[10,9,9]],8],[9,7,6,[[3,8,0,1,1],1],8]]".chars();
        assert_eq!(cmp(&mut left, &mut right), Ordering::Greater);
    }
}
