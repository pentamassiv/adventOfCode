use std::{collections::HashSet, path::Path};

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

    let mut material = HashSet::new();

    let mut max_y = 0;
    input.lines().for_each(|l| {
        insert_rocks(l, &mut material, &mut max_y);
    });

    let no_rocks = material.len();

    // Calculate solution for part 1
    while let Some((resting_x, resting_y)) = falling_sand(&mut material, max_y, false) {
        if permeate_sand(&mut material, resting_x, resting_y) {
            break;
        };
    }
    let solution1 = material.len() - no_rocks;

    // Calculate solution for part 2
    while let Some((resting_x, resting_y)) = falling_sand(&mut material, max_y, true) {
        if permeate_sand(&mut material, resting_x, resting_y) {
            print!("");
            break;
        };
    }
    let solution2 = material.len() - no_rocks;
    (solution1, solution2)
}

fn insert_rocks(line: &str, material: &mut HashSet<(usize, usize)>, max_y: &mut usize) {
    let rock_coordinates = line
        .split(" -> ")
        .map(|coordinate| {
            let idx = coordinate.find(',').unwrap();
            (
                coordinate[..idx].parse::<usize>().unwrap(),
                coordinate[idx + 1..].parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    let mut rock_coordinates = rock_coordinates.iter().peekable();
    for _ in 0..rock_coordinates.len() - 1 {
        let &(from_x, from_y) = rock_coordinates.next().unwrap();
        let (to_x, to_y) = rock_coordinates.peek().unwrap();
        let range_x = if from_x > *to_x {
            *to_x..=from_x
        } else {
            from_x..=*to_x
        };
        *max_y = (*max_y).max(from_y);
        *max_y = (*max_y).max(*to_y);
        let range_y = if from_y > *to_y {
            *to_y..=from_y
        } else {
            from_y..=*to_y
        };
        for i in range_x {
            for j in range_y.clone() {
                material.insert((i, j));
            }
        }
    }
}

// returns true if the sand is blocking the entrance, otherwise it returns false
fn permeate_sand(
    material: &mut HashSet<(usize, usize)>,
    resting_x: usize,
    resting_y: usize,
) -> bool {
    material.insert((resting_x, resting_y));
    resting_y == 0
}

fn falling_sand(
    material: &mut HashSet<(usize, usize)>,
    max_y: usize,
    has_floor: bool,
) -> Option<(usize, usize)> {
    let (mut x, mut y) = (500, 0);
    let end_y = if has_floor { max_y + 2 } else { max_y };
    while y < end_y {
        y += 1;
        if !material.contains(&(x, y)) {
        } else if !material.contains(&(x - 1, y)) {
            x -= 1;
        } else if !material.contains(&(x + 1, y)) {
            x += 1;
        } else {
            return Some((x, y - 1));
        }
    }
    if has_floor {
        return Some((x, y - 1));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let (part1, part2) = run("input/examples/14/1.txt");
        assert_eq!(part1, 24);
        assert_eq!(part2, 93);
    }

    #[test]
    fn test_input() {
        let (part1, part2) = run("input/14.txt");
        assert_eq!(part1, 1_330);
        assert_eq!(part2, 26_139);
    }
}
