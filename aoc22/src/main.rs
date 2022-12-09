#![warn(clippy::pedantic)]
#![feature(iter_array_chunks)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
/*mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
*/

fn main() {
    println!("Day 1:");
    let (part1, part2) = day1::run("aoc22/input/1.txt");
    println!("  {part1}");
    println!("  {part2}");

    println!("Day 2:");
    let (part1, part2) = day2::run("aoc22/input/2.txt");
    println!("  {part1}");
    println!("  {part2}");

    println!("Day 3:");
    let (part1, part2) = day3::run("aoc22/input/3.txt");
    println!("  {part1}");
    println!("  {part2}");

    println!("Day 4:");
    let (part1, part2) = day4::run("aoc22/input/4.txt");
    println!("  {part1}");
    println!("  {part2}");

    println!("Day 5:");
    let (part1, part2) = day5::run("aoc22/input/5.txt");
    println!("  {part1}");
    println!("  {part2}");

    println!("Day 6:");
    let (part1, part2) = day6::run("aoc22/input/6.txt");
    println!("  {part1}");
    println!("  {part2}");

    println!("Day 7:");
    let (part1, part2) = day7::run("aoc22/input/7.txt");
    println!("  {part1}");
    println!("  {part2}");

    println!("Day 8:");
    let start = std::time::Instant::now();
    let (part1, part2) = day8::run("aoc22/input/8.txt");
    println!("  {part1}");
    println!("  {part2}");
    println!("Elapsed: {:.2?}", start.elapsed());

    /*
    println!("Day 9:");
    let (part1, part2) = day9::run("aoc22/input/9.txt");
    println!("  {part1}");
    println!("  {part2}");

    println!("Day 10:");
    let (part1, part2) = day10::run("aoc22/input/10.txt");
    println!("  {part1}");
    println!("  {part2}");
    */
}
