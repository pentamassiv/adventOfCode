use std::{collections::HashSet, path::Path};

pub fn run<P>(path: P) -> (usize, usize)
where
    P: AsRef<Path>,
{
    let debug = false;

    let input = std::fs::read_to_string(path).unwrap();
    let mut nods = vec![(0, 0); 10];
    let mut visited_by_1: HashSet<(i32, i32)> = HashSet::new();
    let mut visited_by_9: HashSet<(i32, i32)> = HashSet::new();

    for line in input.lines() {
        let distance: usize = line[2..].parse().unwrap_or(0);
        let direction = match &line[..1] {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => (0, 0),
        };
        for _ in 0..distance {
            nods[0] = (nods[0].0 + direction.0, nods[0].1 + direction.1);
            for i in 0..nods.len() - 1 {
                move_tail(nods[i], &mut nods[i + 1]);
            }
            visited_by_1.insert(nods[1]);
            visited_by_9.insert(nods[9]);
            if debug {
                print_rope(&nods);
            }
        }
    }
    let solution1 = visited_by_1.len();
    let solution2 = visited_by_9.len();

    (solution1, solution2)
}

fn move_tail(head: (i32, i32), tail: &mut (i32, i32)) {
    if head.0 - tail.0 == 2 && head.1 - tail.1 == 2 {
        *tail = (head.0 - 1, head.1 - 1);
    } else if head.0 - tail.0 == -2 && head.1 - tail.1 == 2 {
        *tail = (head.0 + 1, head.1 - 1);
    } else if head.0 - tail.0 == 2 && head.1 - tail.1 == -2 {
        *tail = (head.0 - 1, head.1 + 1);
    } else if head.0 - tail.0 == -2 && head.1 - tail.1 == -2 {
        *tail = (head.0 + 1, head.1 + 1);
    } else if head.0 - tail.0 == 2 {
        *tail = (head.0 - 1, head.1);
    } else if head.0 - tail.0 == -2 {
        *tail = (head.0 + 1, head.1);
    } else if head.1 - tail.1 == 2 {
        *tail = (head.0, head.1 - 1);
    } else if head.1 - tail.1 == -2 {
        *tail = (head.0, head.1 + 1);
    }
}

#[allow(clippy::cast_sign_loss)]
fn print_rope(nods: &[(i32, i32)]) {
    let max_x = 6;
    let max_y = 6;
    //let max_x = nods.iter().max_by(|x, y| x.0.cmp(&y.0)).unwrap().0 as usize;
    //let max_y = nods.iter().max_by(|x, y| x.1.cmp(&y.1)).unwrap().1 as usize;
    let mut grid = vec![vec!['.'; max_x]; max_y];
    grid[nods[9].1 as usize][nods[9].0 as usize] = '9';
    grid[nods[8].1 as usize][nods[8].0 as usize] = '8';
    grid[nods[7].1 as usize][nods[7].0 as usize] = '7';
    grid[nods[6].1 as usize][nods[6].0 as usize] = '6';
    grid[nods[5].1 as usize][nods[5].0 as usize] = '5';
    grid[nods[4].1 as usize][nods[4].0 as usize] = '4';
    grid[nods[3].1 as usize][nods[3].0 as usize] = '3';
    grid[nods[2].1 as usize][nods[2].0 as usize] = '2';
    grid[nods[1].1 as usize][nods[1].0 as usize] = '1';
    grid[nods[0].1 as usize][nods[0].0 as usize] = 'H';
    for row in grid.iter().rev() {
        for cell in row {
            print!("{cell}");
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let (part1, part2) = run("input/examples/9/1.txt");
        assert_eq!(part1, 13);
        assert_eq!(part2, 1);
        let (part1, part2) = run("input/examples/9/2.txt");
        assert_eq!(part1, 88);
        assert_eq!(part2, 36);
    }

    #[test]
    fn test_input() {
        let (part1, part2) = run("input/9.txt");
        assert_eq!(part1, 6498);
        assert_eq!(part2, 2531);
    }
}
