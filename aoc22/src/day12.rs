#![allow(clippy::char_lit_as_u8)]

use std::{collections::HashSet, path::Path};

use colored::*;

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

    let (solution1, solution2) = shortest_path(start, goal, &height_map, grid_dimensions);
    /*let mut solution2 = usize::MAX;
    for y in 0..grid_dimensions.1 {
        for x in 0..grid_dimensions.0 {
            if height_map[y][x] == 'a' as u8 {
                solution2 = solution2.min(shortest_path(
                    (x, y),
                    goal,
                    &height_map,
                    grid_dimensions,
                    solution2,
                ));
            }
        }
    }*/

    (solution1, solution2)
}

fn shortest_path(
    start: (usize, usize),
    goal: (usize, usize),
    height_map: &[Vec<u8>],
    grid_dimensions: (usize, usize),
) -> (usize, usize) {
    let solution1;
    let mut solution2 = usize::MAX;
    let mut visited_positions: HashSet<(usize, usize)> = HashSet::new();
    let mut current_positions = HashSet::new();
    let mut next_positions = HashSet::new();
    current_positions.insert(goal);
    visited_positions.insert(goal);
    let mut step_no = 0;

    'steps: loop {
        for position in current_positions.drain() {
            if visited_positions.contains(&start) {
                solution1 = step_no;
                break 'steps;
            }
            let potential_next_positions = potential_next_positions(position, grid_dimensions)
                .into_iter()
                .filter(|(x, y)| !visited_positions.contains(&(*x, *y)))
                .filter(|(x, y)| {
                    let height_next_postion = height_map[*y][*x];
                    let height_current_position = height_map[position.1][position.0];

                    height_next_postion >= height_current_position - 1
                });
            for new_position in potential_next_positions {
                next_positions.insert(new_position);
            }
        }
        step_no += 1;

        for (x, y) in next_positions.drain() {
            if height_map[y][x] == 'a' as u8 {
                solution2 = solution2.min(step_no);
            }
            visited_positions.insert((x, y));
            current_positions.insert((x, y));
        }
        /*
        println!("Step {step_no}");
        for (y, row) in height_map.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if visited_positions.contains(&(x, y)) {
                    if current_positions.contains(&(x, y)) {
                        print!(
                            "{}",
                            String::from(char::from(col.to_ascii_uppercase())).green()
                        );
                    } else {
                        print!(
                            "{}",
                            String::from(char::from(col.to_ascii_uppercase())).blue()
                        );
                    }
                } else {
                    print!("{}", char::from(*col));
                }
            }
            println!();
        }
        println!();
        println!();*/
    }
    (solution1, solution2)
}

fn potential_next_positions(position: (usize, usize), max: (usize, usize)) -> Vec<(usize, usize)> {
    let mut next_steps = vec![];
    let left = position.0.saturating_sub(1);
    let right = position.0 + 1;
    let up = position.1.saturating_sub(1);
    let down = position.1 + 1;

    if right < max.0 {
        next_steps.push((right, position.1));
    }
    if down < max.1 {
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
        assert_eq!(part2, 29);
    }

    #[test]
    fn test_input() {
        let (part1, part2) = run("input/12.txt");
        assert_eq!(part1, 425);
        assert_eq!(part2, 418);
    }
}
