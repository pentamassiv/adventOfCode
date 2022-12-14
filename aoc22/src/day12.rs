#![allow(clippy::char_lit_as_u8)]

use std::{collections::HashSet, path::Path};

pub fn run<P>(path: P) -> (i32, i32)
where
    P: AsRef<Path> + std::fmt::Display,
{
    // Annoying helper to be able to run the tests in regular and debug mode (see https://github.com/rust-lang/rust-analyzer/issues/13208)
    let input = if let Ok(input) = std::fs::read_to_string(&path) {
        input
    } else {
        std::fs::read_to_string(format!("aoc22/{path}")).unwrap()
    };

    let mut height_map: Vec<Vec<u8>> = input
        .lines()
        .map(|l| l.as_bytes().to_owned())
        .collect::<Vec<_>>();
    let grid_dimensions = (height_map[0].len(), height_map.len());

    let mut start = (0, 0);
    let mut goal = (0, 0);
    for (y, line) in height_map.iter().enumerate() {
        for (x, heights) in line.iter().enumerate() {
            if *heights == 'S' as u8 {
                start = (x, y);
            } else if *heights == 'E' as u8 {
                goal = (x, y);
            }
        }
    }
    height_map[start.1][start.0] = 'a' as u8;
    height_map[goal.1][goal.0] = 'z' as u8;

    let solution1;
    let mut visited_positions: HashSet<(usize, usize)> = HashSet::new();
    let mut current_positions = HashSet::new();
    let mut next_positions = HashSet::new();
    current_positions.insert(start);
    let mut step_no = 1;

    'steps: loop {
        for position in &current_positions {
            if visited_positions.contains(&goal) {
                solution1 = step_no;
                break 'steps;
            }
            for new_position in potential_next_positions(*position, grid_dimensions)
                .iter()
                .filter(|(x, y)| !visited_positions.contains(&(*x, *y)))
                .inspect(|(x, y)| {
                    println!("a({x},{y}");
                })
                .filter(|(x, y)| {
                    println!("({x}{y})");
                    println!("({x}{y})");
                    println!("({x}{y})");
                    let height_next_postion = height_map[*y][*x];
                    let height_current_position = height_map[position.1][position.0];
                    height_next_postion <= height_current_position + 1
                })
                .inspect(|(x, y)| {
                    println!("b({x},{y}");
                })
            {
                next_positions.insert(*new_position);
            }
            for (x, y) in &next_positions {
                visited_positions.insert((*x, *y));
            }
        }
        for next_position in next_positions.drain() {
            current_positions.insert(next_position);
        }
        step_no += 1;
    }

    (solution1, 0)
}

fn potential_next_positions(position: (usize, usize), max: (usize, usize)) -> Vec<(usize, usize)> {
    let mut next_steps = vec![];
    let left = position.0.saturating_sub(1);
    let right = position.0 + 1;
    let up = position.1.saturating_sub(1);
    let down = position.1 + 1;

    if right <= max.0 {
        next_steps.push((right, position.1));
    }
    if down <= max.1 {
        next_steps.push((position.0, down));
    }
    next_steps.push((left, position.1));
    next_steps.push((position.0, up));

    next_steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let (part1, part2) = run("input/examples/12/1.txt");
        assert_eq!(part1, 31);
        //assert_eq!(part2, 45000);
    }

    #[test]
    fn test_input() {
        let (part1, part2) = run("input/12.txt");
        //assert_eq!(part1, 75_622);
        //assert_eq!(part2, 213_159);
    }
}
