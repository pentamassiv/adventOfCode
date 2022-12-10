use std::{collections::HashSet, path::Path};

pub fn run<P>(path: P) -> (usize, usize)
where
    P: AsRef<Path>,
{
    let input = std::fs::read_to_string(path).unwrap();

    let mut always_santa_visited: HashSet<_> = HashSet::new();
    let mut sometimes_santa_visited: HashSet<_> = HashSet::new();
    always_santa_visited.insert((0, 0));
    sometimes_santa_visited.insert((0, 0));

    let mut is_robot = false;
    let mut santa = (0, 0);
    let mut santa_and_robot = ((0, 0), (0, 0));
    input.chars().for_each(|movement| {
        let (x, y) = if is_robot {
            (&mut santa_and_robot.0 .0, &mut santa_and_robot.0 .1)
        } else {
            (&mut santa_and_robot.1 .0, &mut santa_and_robot.1 .1)
        };
        match movement {
            '<' => {
                *x -= 1;
                santa.0 -= 1;
            }
            '>' => {
                *x += 1;
                santa.0 += 1;
            }
            '^' => {
                *y += 1;
                santa.1 += 1;
            }
            'v' => {
                *y -= 1;
                santa.1 -= 1;
            }
            _ => {}
        };
        is_robot = !is_robot;
        always_santa_visited.insert((santa.0, santa.1));
        sometimes_santa_visited.insert((*x, *y));
    });
    (always_santa_visited.len(), sometimes_santa_visited.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let (part1, part2) = run("input/examples/3/1.txt");
        assert_eq!(part1, 2);
        assert_eq!(part2, 2);
        let (part1, part2) = run("input/examples/3/2.txt");
        assert_eq!(part1, 4);
        assert_eq!(part2, 3);
        let (part1, part2) = run("input/examples/3/3.txt");
        assert_eq!(part1, 2);
        assert_eq!(part2, 11);
        let (part1, part2) = run("input/examples/3/4.txt");
        assert_eq!(part1, 2);
        assert_eq!(part2, 3);
        let (part1, part2) = run("input/examples/3/5.txt");
        assert_eq!(part1, 4);
        assert_eq!(part2, 3);
        let (part1, part2) = run("input/examples/3/6.txt");
        assert_eq!(part1, 2);
        assert_eq!(part2, 11);
    }

    #[test]
    fn test_input() {
        let (part1, part2) = run("input/3.txt");
        assert_eq!(part1, 2592);
        assert_eq!(part2, 2360);
    }
}
