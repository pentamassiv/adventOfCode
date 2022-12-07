pub fn run(path: &str) -> (i32, i32) {
    // Read the input file
    let input = std::fs::read_to_string(path).unwrap();

    let mut complete_overlaps = 0;
    let mut partial_overlaps = 0;
    // Iterator over the lines
    input.lines().for_each(|line| {
        line.splitn(4, |c| c == '-' || c == ',')
            .map(|x| x.parse::<i32>().unwrap())
            .array_chunks::<4>()
            .filter(|[start_a, end_a, start_b, end_b]| end_a >= start_b && end_b >= start_a)
            .inspect(|_| {
                partial_overlaps += 1;
            })
            .filter(|[start_a, end_a, start_b, end_b]| start_a.cmp(start_b) != end_a.cmp(end_b))
            .for_each(|_| {
                complete_overlaps += 1;
            });
    });

    (complete_overlaps, partial_overlaps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let (part1, part2) = run("input/examples/4/1.txt");
        assert_eq!(part1, 2);
        assert_eq!(part2, 4);
    }

    #[test]
    fn test_input() {
        let (part1, part2) = run("input/4.txt");
        assert_eq!(part1, 538);
        assert_eq!(part2, 792);
    }
}
