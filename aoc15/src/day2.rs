use std::path::Path;

pub fn run<P>(path: P) -> (u32, u32)
where
    P: AsRef<Path>,
{
    let input = std::fs::read_to_string(path).unwrap();

    let (solution1, solution2) = input
        .lines()
        .map(|l| l.split('x').collect::<Vec<_>>())
        .map(|l| {
            (
                l[0].parse::<u32>().unwrap(),
                l[1].parse::<u32>().unwrap(),
                l[2].parse::<u32>().unwrap(),
            )
        })
        .map(|(l, w, h)| {
            let (a, b, c) = (l * w, w * h, h * l);
            let paper = 2 * a + 2 * b + 2 * c + a.min(b).min(c);
            let bow = 2 * ((l + w + h) - l.max(w).max(h)) + l * w * h;
            (paper, bow)
        })
        .reduce(|(acc_p, acc_b), (p, b)| (acc_p + p, acc_b + b))
        .unwrap();
    (solution1, solution2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let (part1, part2) = run("input/examples/2/1.txt");
        assert_eq!(part1, 58);
        assert_eq!(part2, 34);
        let (part1, part2) = run("input/examples/2/2.txt");
        assert_eq!(part1, 43);
        assert_eq!(part2, 14);
    }

    #[test]
    fn test_input() {
        let (part1, part2) = run("input/2.txt");
        assert_eq!(part1, 1_606_483);
        assert_eq!(part2, 3_842_356);
    }
}
