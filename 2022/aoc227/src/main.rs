fn main() {
    // Read the input file
    let input = std::fs::read_to_string("2022/aoc227/input").unwrap();

    // Iterator over the lines
    for line in input.lines() {
        println!("{line}");
    }
}
