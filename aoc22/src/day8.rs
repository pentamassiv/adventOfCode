use std::{
    collections::{HashMap, HashSet},
    path::Path,
    vec,
};

pub fn run<P>(path: P) -> (usize, i32)
where
    P: AsRef<Path>,
{
    let input = std::fs::read_to_string(path).unwrap();
    let grid_width = input.find('\n').unwrap();
    let input = input.as_bytes();
    let grid_height = input.len() / grid_width;

    let mut max_v = vec![0; grid_width];
    let mut max_h = 0;

    let mut visible_trees: HashSet<(usize, usize)> = HashSet::new();
    let mut scenic_scores: HashMap<(usize, usize), usize> = HashMap::new();
    // insert_border_trees(&mut visible_trees, grid_width, grid_height);

    let mut tree;
    for row in 0..grid_height {
        for col in 0..grid_width {
            tree = input[row * (grid_width + 1) + col];
            check_visibility(
                input,
                &mut tree,
                (row, col),
                &mut max_h,
                &mut max_v,
                grid_width,
                &mut visible_trees,
            );
        }
        max_h = 0;
    }

    visible_trees.len();

    let mut max_h = 0;
    let mut max_v = vec![0; grid_width];

    for row in (0..grid_height).rev() {
        for col in (0..grid_width).rev() {
            tree = input[row * (grid_width + 1) + col];
            check_visibility(
                input,
                &mut tree,
                (row, col),
                &mut max_h,
                &mut max_v,
                grid_width,
                &mut visible_trees,
            );
        }
        max_h = 0;
    }

    let solution1 = visible_trees.len();
    println!("{solution1:?}");
    (solution1, 0)
}

fn check_visibility(
    input: &[u8],
    tree: &mut u8,
    position: (usize, usize),
    max_h: &mut u8,
    max_v: &mut [u8],
    grid_width: usize,
    visible_trees: &mut HashSet<(usize, usize)>,
) {
    *tree = input[position.0 * (grid_width + 1) + position.1];
    if *tree > *max_h {
        visible_trees.insert(position);
        *max_h = *tree;
    }
    if *tree > max_v[position.1] {
        visible_trees.insert(position);
        max_v[position.1] = *tree;
    }
}

fn add_scenic_score(
    scenic_scores: &mut HashMap<(usize, usize), usize>,
    input: &[u8],
    tree: u8,
    row: usize,
    col: usize,
    grid_width: usize,
    rev: bool,
) {
    let mut viewed_tree;
    let mut view_score = 0;

    for i in (0..col).rev() {
        viewed_tree = input[row * (grid_width + 1) + col - i];
        view_score += 1;
        if tree >= viewed_tree {
            scenic_scores
                .entry((col, row))
                .and_modify(|old_score| *old_score += view_score)
                .or_insert(view_score);
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let (part1, part2) = run("input/examples/8/1.txt");
        assert_eq!(part1, 21);
        //assert_eq!(part2, 0);
    }

    #[test]
    fn test_input() {
        let (part1, part2) = run("input/8.txt");
        assert_eq!(part1, 1679);
        /* assert_eq!(part2, 213_159);
         */
    }
}
