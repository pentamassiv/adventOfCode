use std::path::Path;

pub fn run<P>(path: P) -> (isize, isize)
where
    P: AsRef<Path> + std::fmt::Display,
{
    // Annoying helper to be able to run the tests in regular and debug mode (see https://github.com/rust-lang/rust-analyzer/issues/13208)
    let input = if let Ok(input) = std::fs::read_to_string(&path) {
        input
    } else {
        std::fs::read_to_string(format!("aoc23/{path}")).unwrap()
    };

    let maze = Maze::from(input);
    let neighbors = maze.neighbors(maze.start);
    let mut possible_starts = vec![];

    // Left neighbour
    if let Some(pos) = neighbors[0] {
        let c = maze.tile(pos);
        match c {
            '-' | 'L' | 'F' => possible_starts.push(pos),
            _ => (),
        }
    }
    // Right neighbour
    if let Some(pos) = neighbors[1] {
        let c = maze.tile(pos);
        match c {
            '-' | 'J' | '7' => possible_starts.push(pos),
            _ => (),
        }
    }
    // Neighbour above
    if let Some(pos) = neighbors[2] {
        let c = maze.tile(pos);
        match c {
            '|' | '7' | 'F' => possible_starts.push(pos),
            _ => (),
        }
    }
    // Neighbour below
    if let Some(pos) = neighbors[3] {
        let c = maze.tile(pos);
        match c {
            // match maze.tile(pos) {
            '|' | 'L' | 'J' => possible_starts.push(pos),
            _ => (),
        }
    }
    let mut part1 = 1;
    'for_loop: for next_step in possible_starts {
        let mut pipe = Pipe {
            current_pos: next_step,
            previous_pos: maze.start,
        };

        part1 = 1;
        while maze.follow(&mut pipe) {
            part1 += 1;
            if pipe.current_pos == maze.start {
                break 'for_loop;
            }
        }
    }
    print!("");
    (part1 / 2, 0)
}

struct Maze {
    input: String,
    start: (usize, usize),
    width: usize,
    height: usize,
}

impl Maze {
    fn neighbors(&self, current_position: (usize, usize)) -> [Option<(usize, usize)>; 4] {
        match current_position {
            (0, 0) => [None, Some((1, 0)), None, Some((0, 1))],
            (0, y) if y == self.height => [None, Some((1, y)), Some((0, y - 1)), None],
            (0, y) => [None, Some((1, y)), Some((0, y - 1)), Some((0, y + 1))],
            (x, 0) if x == self.width => [Some((x - 1, 0)), None, None, Some((x, 1))],
            (x, 0) => [Some((x - 1, 0)), Some((x + 1, 0)), None, Some((x, 1))],
            (x, y) if x == self.width && y == self.height => {
                [Some((x - 1, y)), None, Some((x, y - 1)), None]
            }
            (x, y) if x == self.width => {
                [Some((x - 1, y)), None, Some((x, y - 1)), Some((x, y + 1))]
            }
            (x, y) if y == self.height => {
                [Some((x - 1, y)), Some((x + 1, y)), Some((x, y - 1)), None]
            }
            (x, y) => [
                Some((x - 1, y)),
                Some((x + 1, y)),
                Some((x, y - 1)),
                Some((x, y + 1)),
            ],
        }
    }

    fn tile(&self, pos: (usize, usize)) -> char {
        self.input
            .chars()
            .nth((self.width) * pos.1 + pos.0)
            .unwrap()
    }

    /// Follow the pipe to the next tile
    /// Returns true, if there is something to follow and false if it was not a pipe or the start
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
    fn follow(&self, pipe: &mut Pipe) -> bool {
        let tile = self.tile(pipe.current_pos);
        let (connected_a, connected_b) = match tile {
            '|' => ((0, -1), (0, 1)),
            '-' => ((-1, 0), (1, 0)),
            'L' => ((0, -1), (1, 0)),
            'J' => ((-1, 0), (0, -1)),
            '7' => ((-1, 0), (0, 1)),
            'F' => ((0, 1), (1, 0)),
            '.' | 'S' => return false,
            _ => panic!("the input must not contain any other chars"),
        };
        let pipe_end_a = (
            (pipe.current_pos.0 as isize + connected_a.0) as usize,
            (pipe.current_pos.1 as isize + connected_a.1) as usize,
        );
        if pipe.previous_pos == pipe_end_a {
            pipe.previous_pos = pipe.current_pos;
            pipe.current_pos = (
                (pipe.current_pos.0 as isize + connected_b.0) as usize,
                (pipe.current_pos.1 as isize + connected_b.1) as usize,
            );
        } else {
            pipe.previous_pos = pipe.current_pos;
            pipe.current_pos = pipe_end_a;
        }
        true
    }
}

impl From<String> for Maze {
    fn from(input: String) -> Self {
        let width = input.find('\n').unwrap() + 1;
        let height = input.len() / width;

        let start_idx = input.find('S').unwrap();
        let start = (start_idx % width, start_idx / width);

        Self {
            input,
            start,
            width,
            height,
        }
    }
}

struct Pipe {
    current_pos: (usize, usize),
    previous_pos: (usize, usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let (part1, part2) = run("input/examples/10/1.txt");
        assert_eq!(part1, 4);
        assert_eq!(part2, 4);
    }

    #[test]
    fn test_example2() {
        let (part1, part2) = run("input/examples/10/2.txt");
        assert_eq!(part1, 8);
        assert_eq!(part2, 4);
    }

    #[test]
    fn test_input() {
        let (part1, part2) = run("input/10.txt");
        assert_eq!(part1, 6_640);
        assert_eq!(part2, 1_097);
    }
}
