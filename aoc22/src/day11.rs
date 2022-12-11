use std::{collections::VecDeque, path::Path};

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

    //let calming_fn = usize::saturating_sub; // x/y
    let calming_fn = |x, y| x / y;

    let mut new_worry_level;
    let mut receiving_monkey;
    let mut inspections = vec![0; monkeys.len()];
    'operand: for candidate in 3..4 {
        //'operand: for candidate in 0..1000 {
        let mut held_items = monkeys.iter().map(|m| m.0.clone()).collect::<Vec<_>>();

        inspections = vec![0; monkeys.len()];
        for round_no in 0..20 {
            //for round_no in 0..10_000 {
            for no in 0..held_items.len() {
                for _ in 0..held_items[no].len() {
                    if let Some(worry_level) = held_items[no].pop_front() {
                        new_worry_level = monkeys[no].1(worry_level);
                        new_worry_level = calming_fn(new_worry_level, candidate);
                        receiving_monkey = if new_worry_level % monkeys[no].2 == 0 {
                            monkeys[no].3
                        } else {
                            monkeys[no].4
                        };
                        held_items[receiving_monkey].push_back(new_worry_level);
                    };
                    inspections[no] += 1;
                }
            }
            println!("{}", round_no + 1);
            for (no, held_items) in held_items.iter().enumerate() {
                println!("Monkey {no}: {held_items:?}",);
            }
            /*if !check_monkey_business(&inspections, round_no + 1) {
                continue 'operand;
            }*/
            println!();
        }
        println!("Found operand {candidate}");
    }

    println!("{inspections:?}");
    inspections.sort_unstable();
    let solution1 = inspections.iter().rev().take(2).product();
    (solution1, 0)
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

fn check_monkey_business(business: &Vec<usize>, round: usize) -> bool {
    match round {
        1 => *business == vec![2, 4, 3, 6],
        20 => *business == vec![99, 97, 8, 103],
        1000 => *business == vec![5204, 4792, 199, 5192],
        2000 => *business == vec![10419, 9577, 392, 10391],
        3000 => *business == vec![15638, 14358, 587, 15593],
        4000 => *business == vec![20858, 19138, 780, 20797],
        5000 => *business == vec![26075, 23921, 974, 26000],
        6000 => *business == vec![31294, 28702, 1165, 31204],
        7000 => *business == vec![36508, 33488, 1360, 36400],
        8000 => *business == vec![41728, 38268, 1553, 41606],
        9000 => *business == vec![46945, 43051, 1746, 46807],
        10000 => *business == vec![52166, 47830, 1938, 52013],
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let (part1, part2) = run("input/examples/11/1.txt");
        assert_eq!(part1, 10605);
        //assert_eq!(part2, 45000);
    }

    #[test]
    fn test_input() {
        let (part1, part2) = run("input/11.txt");
        //assert_eq!(part1, 78_678);
        //assert_eq!(part2, 213_159);
    }
}
