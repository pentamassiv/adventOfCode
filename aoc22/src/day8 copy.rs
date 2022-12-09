use std::{collections::HashSet, path::Path, vec};

pub fn run<P>(path: P) -> (usize, i32)
where
    P: AsRef<Path>,
{
    let input = std::fs::read_to_string(path).unwrap();
    let grid_width = input.find('\n').unwrap();
    let mut grid_height = 0;
    let mut max_v = vec![0; grid_width];
    let mut visible_trees = HashSet::new();
    /*let tree_grid = input.lines().map(|l| {
        l.chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<usize>>()
    });*/
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as usize))
        .enumerate()
        .for_each(|(r_no, r)| {
            insert_visible_trees(&mut visible_trees, r.clone().enumerate(), r_no);
            for (col_no, tree) in r.enumerate() {
                if tree > max_v[col_no] {}
            }
            grid_height = r_no;
        });

    insert_border_trees(&mut visible_trees, grid_width, grid_height);

    let solution1 = visible_trees.len();
    println!("{solution1:?}");
    (solution1, 0)
}

fn insert_visible_trees<S: Iterator<Item = (usize, usize)>>(
    visible_trees: &mut HashSet<(usize, usize)>,
    sequence: S,
    row_no: usize,
) {
    let mut max_height = 0;
    sequence
        .skip_while(|(_, x)| {
            if *x <= max_height {
                true
            } else {
                max_height = *x;
                false
            }
        })
        .for_each(|(column_no, _)| {
            visible_trees.insert((column_no, row_no));
        });
}

fn insert_border_trees(
    visible_trees: &mut HashSet<(usize, usize)>,
    grid_width: usize,
    grid_height: usize,
) {
    for i in 0..grid_width {
        visible_trees.insert((0, i));
        visible_trees.insert((grid_height, i));
    }
    for i in 0..grid_height {
        visible_trees.insert((i, 0));
        visible_trees.insert((i, grid_width));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let (part1, part2) = run("input/examples/8/1.txt");
        assert_eq!(part1, 21);
        //assert_eq!(part2, 45000);
    }

    #[test]
    fn test_input() {
        let (part1, part2) = run("input/8.txt");
        assert!(part1 != 1153);
        /* assert_eq!(part2, 213_159);
         */
    }
}
