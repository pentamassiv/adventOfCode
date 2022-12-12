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

    let height_map = input.lines().map(str::as_bytes).collect::<Vec<_>>();
    let grid = (height_map[0].len(), height_map.len());

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

    let solution1;
    let mut visited_positions: HashSet<(usize, usize)> = HashSet::new();
    let mut current_positions;
    let mut next_positions = vec![start];
    let mut step_no = 1;
    'steps: loop {
        current_positions = next_positions.clone();
        for i in 0..current_positions.len() {
            if visited_positions.contains(&goal) {
                solution1 = step_no;
                break 'steps;
            }
            next_positions = potential_next_positions(current_positions[i], grid)
                .iter()
                .filter(|(x, y)| !visited_positions.contains(&(*x, *y)))
                .inspect(|(x, y)| {
                    println!("a({x},{y}");
                })
                .filter(|(x, y)| {
                    height_map[*x][*y]
                        < (height_map[current_positions[i].1][current_positions[i].0] + 1u8)
                })
                .inspect(|(x, y)| {
                    println!("b({x},{y}");
                })
                .copied()
                .collect::<Vec<_>>();
            for (x, y) in &next_positions {
                visited_positions.insert((*x, *y));
            }
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
