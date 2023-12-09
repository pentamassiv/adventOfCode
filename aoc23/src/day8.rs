use std::path::Path;

pub fn run<P>(path: P) -> (usize, usize)
where
    P: AsRef<Path> + std::fmt::Display,
{
    // Annoying helper to be able to run the tests in regular and debug mode (see https://github.com/rust-lang/rust-analyzer/issues/13208)
    let input = if let Ok(input) = std::fs::read_to_string(&path) {
        input
    } else {
        std::fs::read_to_string(format!("aoc23/{path}")).unwrap()
    };
    let mut input = input.lines();

    let mut directions = input.next().unwrap().chars().cycle();
    let mut nodes: [(usize, usize, bool); 26 * 26 * 26] = [(0, 0, false); 26 * 26 * 26];
    input.next(); // Skip empty line

    let mut part2_current_nodes = vec![];

    input.for_each(|l| {
        let node_no = position(&l[..3]);
        let left = position(&l[7..10]);
        let right = position(&l[12..15]);
        if &l[2..3] == "A" {
            part2_current_nodes.push(node_no)
        };
        let possible_end = if &l[2..3] == "Z" { true } else { false };

        nodes[node_no as usize] = (left, right, possible_end)
    });

    /*
    let mut part1 = 0;
    let mut i: usize = 0;

    while i != 26 * 26 * 26 - 1 {
        match directions.next().unwrap() {
            'L' => i = nodes[i as usize].0,
            'R' => i = nodes[i as usize].1,
            _ => panic!("the directions must only contain L and R"),
        }
        part1 += 1;
    }*/

    println!("Done with part 1");

    let mut part2 = 0;
    let mut all_at_end_nodes = false;
    let mut current_node;
    let mut next_node;
    let mut direction;

    for _ in 0..part2_current_nodes.len() - 1 {
        part2_current_nodes.pop();
    }

    while !all_at_end_nodes {
        part2 += 1;

        for node in &part2_current_nodes {
            print!("{} ", name(*node));
        }
        println!();

        all_at_end_nodes = true;
        direction = directions.next().unwrap();
        println!("{direction}");

        for j in 0..part2_current_nodes.len() {
            current_node = part2_current_nodes[j];
            next_node = match direction {
                'L' => nodes[current_node].0,
                'R' => nodes[current_node].1,
                _ => panic!("the directions must only contain L and R"),
            };
            if all_at_end_nodes {
                all_at_end_nodes = nodes[next_node].2;
            }
            part2_current_nodes[j] = next_node;
        }
    }

    (0, part2)
}

fn position(node_name: &str) -> usize {
    let node_name = node_name.as_bytes();
    let ascii_a = "A".as_bytes()[0];
    (node_name[0] - ascii_a) as usize * 26 * 26
        + (node_name[1] - ascii_a) as usize * 26
        + (node_name[2] - ascii_a) as usize
}

fn name(mut position: usize) -> String {
    let third = position % 26;
    position -= third;
    let second = position % (26 * 26) / 26;
    position -= second;
    let first = position % (26 * 26 * 26) / (26 * 26);

    vec![first, second, third]
        .iter()
        .map(|&i| char::from(i as u8 + "A".as_bytes()[0]))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let (part1, part2) = run("input/examples/8/1.txt");
        assert_eq!(part1, 2);
        assert_eq!(part2, 2);
    }

    #[test]
    fn test_example2() {
        let (part1, part2) = run("input/examples/8/2.txt");
        assert_eq!(part1, 6);
        assert_eq!(part2, 6);
    }

    #[test]
    fn test_example3() {
        let (part1, part2) = run("input/examples/8/3.txt");
        assert_eq!(part1, 0);
        assert_eq!(part2, 6);
    }

    #[test]
    fn test_input() {
        let (part1, part2) = run("input/8.txt");
        assert_eq!(part1, 13_019);
        assert_eq!(part2, 246_894_760);
    }
}
