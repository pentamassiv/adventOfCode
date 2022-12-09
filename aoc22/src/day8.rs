use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

pub fn run<P>(path: P) -> (usize, usize)
where
    P: AsRef<Path>,
{
    let input = std::fs::read_to_string(path).unwrap();
    let input = input.trim();
    let grid_width = input.find('\n').unwrap();
    let input = input.as_bytes();
    let grid_height = input.len() / grid_width;

    let mut visible_trees: HashSet<(usize, usize)> = HashSet::new();
    let mut scenic_scores: HashMap<(usize, usize), usize> = HashMap::new();

    let mut tree;

    for row in 0..grid_height {
        for col in 0..grid_width {
            tree = input[row * (grid_width + 1) + col];
            add_scenic_score(
                &mut visible_trees,
                &mut scenic_scores,
                input,
                tree,
                row,
                col,
                grid_width,
                row_idx,
                (0..col).rev(),
            );
            add_scenic_score(
                &mut visible_trees,
                &mut scenic_scores,
                input,
                tree,
                row,
                col,
                grid_width,
                col_idx,
                (0..row).rev(),
            );
        }
    }

    for row in (0..grid_height).rev() {
        for col in (0..grid_width).rev() {
            tree = input[row * (grid_width + 1) + col];
            add_scenic_score(
                &mut visible_trees,
                &mut scenic_scores,
                input,
                tree,
                row,
                col,
                grid_width,
                row_idx,
                col + 1..grid_width,
            );
            add_scenic_score(
                &mut visible_trees,
                &mut scenic_scores,
                input,
                tree,
                row,
                col,
                grid_width,
                col_idx,
                row + 1..grid_height,
            );
        }
    }

    let solution1 = visible_trees.len();
    let solution2 = *scenic_scores.iter().max_by(|x, y| x.1.cmp(y.1)).unwrap().1;
    (solution1, solution2)
}

#[allow(clippy::too_many_arguments)]
fn add_scenic_score<I: Iterator<Item = usize>>(
    visible_trees: &mut HashSet<(usize, usize)>,
    scenic_scores: &mut HashMap<(usize, usize), usize>,
    input: &[u8],
    tree: u8,
    row: usize,
    col: usize,
    grid_width: usize,
    idx_fn: fn(usize, usize, usize, usize) -> usize,
    range: I,
) {
    let mut viewed_tree;
    let mut view_score = 0;

    for i in range {
        viewed_tree = input[idx_fn(row, col, grid_width, i)];
        view_score += 1;
        if viewed_tree >= tree {
            scenic_scores
                .entry((col, row))
                .and_modify(|old_score| *old_score *= view_score)
                .or_insert(view_score);
            return;
        }
    }
    scenic_scores
        .entry((col, row))
        .and_modify(|old_score| *old_score *= view_score)
        .or_insert(view_score);
    visible_trees.insert((col, row));
}
fn row_idx(row: usize, _: usize, grid_width: usize, i: usize) -> usize {
    row * (grid_width + 1) + i
}
fn col_idx(_: usize, col: usize, grid_width: usize, i: usize) -> usize {
    i * (grid_width + 1) + col
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let (part1, part2) = run("input/examples/8/1.txt");
        assert_eq!(part1, 21);
        assert_eq!(part2, 8);
    }

    #[test]
    fn test_input() {
        let (part1, part2) = run("input/8.txt");
        assert_eq!(part1, 1679);
        assert_eq!(part2, 536_625);
    }
}
