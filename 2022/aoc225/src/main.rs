#![warn(clippy::pedantic)]

use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let debug = false; // Activate debug print statements

    let mut lines = io::BufReader::new(File::open("2022/aoc225/input").unwrap())
        .lines()
        .flatten()
        .peekable();

    let mut stacks = vec![];
    for _ in 0..=lines.peek().unwrap().len() / 4 {
        stacks.push(VecDeque::new());
    }

    let mut crate_name;
    let mut end_start_config = 0;
    // Get starting positions of the crates
    for (line_no, line) in lines.enumerate() {
        if line.chars().nth(1).unwrap() == '1' {
            end_start_config = line_no;
            break;
        }
        for (stack_no, stack) in stacks.iter_mut().enumerate() {
            crate_name = line.chars().nth(stack_no * 4 + 1).unwrap();
            if crate_name != ' ' {
                stack.push_back(crate_name);
            }
        }
    }

    let mut stacks_part2 = stacks.clone();

    // Move the crates
    let lines = io::BufReader::new(File::open("2022/aoc225/input").unwrap())
        .lines()
        .flatten()
        .skip(end_start_config + 2);

    let mut mover_9001 = vec![];
    for line in lines {
        let [no, from, to]: [usize; 3] = line
            .split(' ')
            .filter_map(|str| str.parse().ok())
            .collect::<Vec<usize>>()
            .try_into()
            .unwrap();
        for _ in 0..no {
            let item = stacks[from - 1].pop_front().unwrap();
            stacks[to - 1].push_front(item);
            mover_9001.push(stacks_part2[from - 1].pop_front().unwrap());
        }
        for &item in mover_9001.iter().rev() {
            stacks_part2[to - 1].push_front(item);
        }
        mover_9001.clear();

        if debug {
            //Debug print
            let mut stack_max_len = 0;
            for stack in &stacks {
                stack_max_len = stack_max_len.max(stack.len());
            }

            println!();
            for row in 0..stack_max_len {
                for stack in &stacks {
                    if stack
                        .get(row.wrapping_sub(stack_max_len - stack.len()))
                        .is_some()
                    {
                        print!(
                            "[{}] ",
                            stack
                                .get(row.wrapping_sub(stack_max_len - stack.len()))
                                .unwrap()
                        );
                    } else {
                        print!("    ");
                    }
                }
                println!();
            }

            for no in 0..stacks.len() {
                print!(" {no}  ");
            }
            println!();
            println!();
        }
    }
    for stack in stacks {
        print!("{}", stack.front().unwrap());
    }
    println!();
    for stack in stacks_part2 {
        print!("{}", stack.front().unwrap());
    }
    println!();
}

/*

    lines.flatten().for_each(|line| {
        line.splitn(4, |c| c == '-' || c == ',')
*/
