#![allow(clippy::char_lit_as_u8)]
use std::{path::Path, str::Chars};

pub fn run<P>(path: P) -> (usize, i32)
where
    P: AsRef<Path> + std::fmt::Display,
{
    // Annoying helper to be able to run the tests in regular and debug mode (see https://github.com/rust-lang/rust-analyzer/issues/13208)
    let input = if let Ok(input) = std::fs::read_to_string(&path) {
        input
    } else {
        std::fs::read_to_string(format!("aoc22/{path}")).unwrap()
    };

    let mut lines = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            println!("{l}");
            l.chars()
        })
        .array_chunks::<2>()
        .collect::<Vec<_>>();
    let solution1 = lines
        .iter_mut()
        .enumerate()
        .filter_map(|(no, [top, bottom])| {
            println!("Comparing pair {}", no + 1);
            if compare(top, bottom) {
                Some(no + 1)
            } else {
                None
            }
        })
        .sum();
    (solution1, 0)
}

#[allow(clippy::too_many_lines)]
fn compare(top: &mut Chars, bottom: &mut Chars) -> bool {
    let mut top_char;
    let mut bottom_char;
    let mut next_top = top.next();
    let mut next_bottom = bottom.next();
    let mut wrapped = (false, false);
    println!();
    loop {
        if let Some(tc) = next_top {
            top_char = tc;
        } else {
            println!("true");
            println!("Top row ran out of items");
            return true;
        }
        if let Some(bc) = next_bottom {
            bottom_char = bc;
        } else {
            println!("false");
            println!("Bottom row ran out of items");
            return false;
        }
        println!("Comparing: {top_char}{bottom_char}");

        if top_char == bottom_char {
            next_top = top.next();
            next_bottom = bottom.next();
            continue;
        } else if !(top_char == '[' || top_char == ']' || bottom_char == '[' || bottom_char == ']')
        {
            println!("{}", top_char < bottom_char);
            println!("Numbers were compared");
            return top_char < bottom_char;
        }
        match top_char {
            '[' => match bottom_char {
                '[' => {
                    unreachable!()
                }
                ']' => {
                    println!("false");
                    println!("Bottom row ran out of items (empty array)");
                    return false;
                }
                '0'..='9' => {
                    if wrapped.1 {
                        println!("false");
                        println!("Bottom row would have to be wrapped more than  once");
                        return false;
                    }
                    next_top = top.next();
                    wrapped.1 = true;
                }
                ',' => {
                    next_top = top.next();
                }
                _ => panic!(),
            },
            ']' => match bottom_char {
                '[' => {
                    println!("true");
                    println!("Top row ran out of items (empty array)");
                    return true;
                }
                ']' => {
                    unreachable!()
                }
                '0'..='9' => {
                    if wrapped.1 {
                        wrapped.1 = false;
                        next_top = top.next();
                    } else {
                        println!("true");
                        println!("Top row ran out of items");
                        return true;
                    }
                }
                ',' => {
                    println!("true");
                    println!("Top row ran out of items (comma)");
                    return true;
                }
                _ => panic!(),
            },
            ln @ '0'..='9' => match bottom_char {
                '[' => {
                    if wrapped.0 {
                        println!("true");
                        println!("Top row would have to be wrapped more than  once");
                        return true;
                    }
                    next_bottom = bottom.next();
                    wrapped.0 = true;
                }
                ']' => {
                    if wrapped.0 {
                        wrapped.0 = false;
                        next_bottom = bottom.next();
                    } else {
                        println!("false");
                        println!("Bottom row ran out of items");
                        return false;
                    }
                }
                rn @ '0'..='9' => {
                    println!("{}", ln < rn);
                    println!("Numbers were compared");
                }
                _ => panic!(),
            },
            ',' => {
                println!("false");
                println!("Bottom row ran out of items (comma)");
                return false;
            }
            _ => panic!(),
        }
    }
}

enum Item {
    ClosingBracket,
    OpeningBracket,
    Number(usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let (part1, part2) = run("input/examples/13/1.txt");
        assert_eq!(part1, 13);
        //assert_eq!(part2, 45000);
    }

    #[test]
    fn test_input() {
        let (part1, part2) = run("input/13.txt");
        assert!(part1 > 5770);
        assert!(part1 < 6020);

        assert_eq!(part1, 5770);
        //assert_eq!(part2, 213_159);
    }

    #[test]
    fn test_comparison_fn() {
        let mut left = "[1,1,3,1,1]".chars();
        let mut right = "[1,1,5,1,1]".chars();
        assert!(compare(&mut left, &mut right));

        let mut left = "[[1],[2,3,4]]".chars();
        let mut right = "[[1],4]".chars();
        assert!(compare(&mut left, &mut right));

        let mut left = "[9]".chars();
        let mut right = "[[8,7,6]]".chars();
        assert!(!compare(&mut left, &mut right));

        let mut left = "[[4,4],4,4]".chars();
        let mut right = "[[4,4],4,4,4]".chars();
        assert!(compare(&mut left, &mut right));

        let mut left = "[7,7,7,7]".chars();
        let mut right = "[7,7,7]".chars();
        assert!(!compare(&mut left, &mut right));

        let mut left = "[]".chars();
        let mut right = "[3]".chars();
        assert!(compare(&mut left, &mut right));

        let mut left = "[[[]]]".chars();
        let mut right = "[[]]".chars();
        assert!(!compare(&mut left, &mut right));

        let mut left = "[1,[2,[3,[4,[5,6,7]]]],8,9]".chars();
        let mut right = "[1,[2,[3,[4,[5,6,0]]]],8,9]".chars();
        assert!(!compare(&mut left, &mut right));

        let mut left = "[[1,[[5]],[4,1,[]],10,[4,[2,6,5,4,2]]]]".chars();
        let mut right = "[[[],5,[],5],[8,3,4,6,7],[1,[]]]".chars();
        assert!(!compare(&mut left, &mut right));
    }
}
