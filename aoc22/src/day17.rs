use std::{path::Path, vec};

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

    let mut movement = input
        .chars()
        .map(|c| {
            if c == '<' {
                Direction::Left
            } else {
                Direction::Right
            }
        })
        .intersperse(Direction::Down)
        .cycle();

    let shape_order = vec![
        Shape::HLine,
        Shape::Plus,
        Shape::InvertL,
        Shape::VLine,
        Shape::Square,
    ];

    let falling_shapes = shape_order.iter().cycle();

    let mut highest_rock = 0;
    let mut tower: Vec<Vec<bool>> = vec![vec![true; 9]];
    let row = vec![true, false, false, false, false, false, false, false, true];
    let mut direction;
    for &shape in falling_shapes.take(2022) {
        for _ in (tower.len() - highest_rock)..8 {
            tower.push(row.clone());
        }
        let mut stone = Stone::new(shape, highest_rock);
        loop {
            direction = movement.next().unwrap();
            /*if direction == Direction::Left {
                println!("<");
            } else if direction == Direction::Right {
                println!(">");
            } else {
                println!("v");
            }*/
            if !stone.move_stone(&tower, stone.next_position(&direction))
                && direction == Direction::Down
            {
                break;
            }
        }

        stone.persist_position(&mut tower, &mut highest_rock);
        for y in 1..tower.len() {
            print!("|");
            for x in 1..tower[0].len() - 1 {
                if tower[tower.len() - y][x] {
                    print!("#");
                } else {
                    print!(".")
                }
            }
            println!("|");
        }
        println!("+-------+");
        println!();
    }

    (highest_rock, 0)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Down,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Shape {
    HLine,
    Plus,
    InvertL,
    VLine,
    Square,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Stone {
    location: Vec<(usize, usize)>,
}

impl Stone {
    fn new(shape: Shape, highest_rock: usize) -> Self {
        let location = match shape {
            Shape::HLine => {
                vec![
                    (3, highest_rock + 4),
                    (4, highest_rock + 4),
                    (5, highest_rock + 4),
                    (6, highest_rock + 4),
                ]
            }
            Shape::Plus => {
                vec![
                    (4, highest_rock + 4),
                    (3, highest_rock + 5),
                    (4, highest_rock + 5),
                    (5, highest_rock + 5),
                    (4, highest_rock + 6),
                ]
            }
            Shape::InvertL => {
                vec![
                    (3, highest_rock + 4),
                    (4, highest_rock + 4),
                    (5, highest_rock + 4),
                    (5, highest_rock + 5),
                    (5, highest_rock + 6),
                ]
            }
            Shape::VLine => {
                vec![
                    (3, highest_rock + 4),
                    (3, highest_rock + 5),
                    (3, highest_rock + 6),
                    (3, highest_rock + 7),
                ]
            }
            Shape::Square => {
                vec![
                    (3, highest_rock + 4),
                    (4, highest_rock + 4),
                    (3, highest_rock + 5),
                    (4, highest_rock + 5),
                ]
            }
        };
        Self { location }
    }

    // Calculates the next position of a rock
    fn next_position(&self, direction: &Direction) -> Vec<(usize, usize)> {
        let mut next_position = vec![];
        for (x, y) in &self.location {
            match direction {
                Direction::Left => {
                    next_position.push((*x - 1, *y));
                }
                Direction::Right => {
                    next_position.push((*x + 1, *y));
                }
                Direction::Down => {
                    next_position.push((*x, *y - 1));
                }
            };
        }
        next_position
    }

    // returns true if the move is possible and false if it is not
    fn move_stone(&mut self, tower: &[Vec<bool>], next_position: Vec<(usize, usize)>) -> bool {
        for &(x, y) in &next_position {
            if tower[y][x] {
                return false;
            }
        }
        self.location = next_position;
        true
    }

    // persists the stone in the tower and returns the value of the highest row
    fn persist_position(self, tower: &mut [Vec<bool>], highest_rock: &mut usize) {
        *highest_rock = (*highest_rock).max(self.location.last().unwrap().1);
        for &(x, y) in &self.location {
            tower[y][x] = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_example() {
        let (part1, part2) = run("input/examples/17/1.txt");
        assert_eq!(part1, 3068);
        //assert_eq!(part2, 45000);
    }

    #[ignore]
    #[test]
    fn test_input() {
        let (part1, part2) = run("input/17.txt");
        assert_eq!(part1, 3149);
        //assert_eq!(part2, 1514285714288);
    }
}
