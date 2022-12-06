#![feature(iter_array_chunks)]

fn main() {
    // Read the input file
    let input = std::fs::read_to_string("2022/aoc224/input").unwrap();

    let mut complete_overlaps = 0;
    let mut partial_overlaps = 0;
    // Iterator over the lines
    input.lines().for_each(|line| {
        line.splitn(4, |c| c == '-' || c == ',')
            .map(|x| x.parse::<i32>().unwrap())
            .array_chunks::<4>()
            .filter(|[start_a, end_a, start_b, end_b]| {
                (*start_a..=*end_a).contains(start_b)
                    || (*start_a..=*end_a).contains(end_b)
                    || (*start_b..=*end_b).contains(start_a)
                    || (*start_b..=*end_b).contains(end_a)
            })
            .inspect(|_| {
                partial_overlaps += 1;
            })
            .filter(|[start_a, end_a, start_b, end_b]| start_a.cmp(start_b) != end_a.cmp(end_b))
            .for_each(|_| {
                complete_overlaps += 1;
            })
    });

    println!("{complete_overlaps}");
    println!("{partial_overlaps}");
}
