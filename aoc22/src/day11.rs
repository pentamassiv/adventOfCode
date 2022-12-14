use std::{collections::VecDeque, path::Path};

type Monkeyrules = Vec<(
    VecDeque<usize>,
    std::boxed::Box<dyn std::ops::Fn(usize) -> usize>,
    usize,
    usize,
    usize,
)>;

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
    let monkeys = input
        .lines()
        .filter(|l| !l.is_empty())
        .array_chunks::<6>()
        .map(|chunk| {
            let starting_items = chunk[1][18..]
                .split(", ")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<VecDeque<_>>();
            let operation = parse_operation(chunk[2]);
            let test = strip_parse(chunk[3], "  Test: divisible by ");
            let true_case = strip_parse(chunk[4], "    If true: throw to monkey ");
            let false_case = strip_parse(chunk[5], "    If false: throw to monkey ");

            (starting_items, operation, test, true_case, false_case)
        })
        .collect::<Vec<_>>();

    let modulus: usize = monkeys.iter().map(|m| m.2).product();

    let calming_fn_part1 = |x| x / 3;
    let calming_fn_part2 = |x| x % modulus;
    let mut held_items_part1 = monkeys.iter().map(|m| m.0.clone()).collect::<Vec<_>>();
    let mut held_items_part2 = monkeys.iter().map(|m| m.0.clone()).collect::<Vec<_>>();

    let mut inspections_part1 =
        count_monkey_actions(&mut held_items_part1, 20, calming_fn_part1, &monkeys);
    let mut inspections_part2 =
        count_monkey_actions(&mut held_items_part2, 10_000, calming_fn_part2, &monkeys);

    inspections_part1.sort_unstable();
    let solution1 = inspections_part1.iter().rev().take(2).product();
    inspections_part2.sort_unstable();
    let solution2 = inspections_part2.iter().rev().take(2).product();
    (solution1, solution2)
}
fn strip_parse(line: &str, prefix: &str) -> usize {
    line.strip_prefix(prefix).unwrap().parse::<usize>().unwrap()
}
fn parse_operation(line: &str) -> Box<dyn Fn(usize) -> usize> {
    if let Ok(operand) = line[25..].parse::<usize>() {
        match &line[23..24] {
            "+" => Box::new(move |x: usize| x + operand),
            "*" => Box::new(move |x: usize| x * operand),
            _ => panic!(),
        }
    } else {
        match &line[23..24] {
            "+" => Box::new(move |x: usize| x + x),
            "*" => Box::new(move |x: usize| x * x),
            _ => panic!(),
        }
    }
}

fn count_monkey_actions<F: Fn(usize) -> usize>(
    held_items: &mut [VecDeque<usize>],
    rounds: usize,
    calming_fn: F,
    monkeys: &Monkeyrules,
) -> Vec<usize> {
    let mut new_worry_level;
    let mut receiving_monkey;
    let mut inspections = vec![0; monkeys.len()];
    for _ in 0..rounds {
        for monkey_no in 0..monkeys.len() {
            for _ in 0..held_items[monkey_no].len() {
                if let Some(worry_level) = held_items[monkey_no].pop_front() {
                    new_worry_level = monkeys[monkey_no].1(worry_level);
                    new_worry_level %= monkeys.iter().map(|m| m.2).product::<usize>();
                    new_worry_level = calming_fn(new_worry_level);
                    receiving_monkey = if new_worry_level % monkeys[monkey_no].2 == 0 {
                        monkeys[monkey_no].3
                    } else {
                        monkeys[monkey_no].4
                    };
                    held_items[receiving_monkey].push_back(new_worry_level);
                };
                inspections[monkey_no] += 1;
            }
        }
    }
    inspections
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let (part1, part2) = run("input/examples/11/1.txt");
        assert_eq!(part1, 10605);
        assert_eq!(part2, 2_713_310_158); /**/
    }

    #[test]
    fn test_input() {
        let (part1, part2) = run("input/11.txt");
        assert_eq!(part1, 78_678);
        assert_eq!(part2, 15_333_249_714);
    }
}
