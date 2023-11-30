#![allow(clippy::cast_sign_loss)]
use std::path::Path;

pub fn run1<P>(path: P, line_of_interest: i32) -> usize
where
    P: AsRef<Path> + std::fmt::Display,
{
    // Annoying helper to be able to run the tests in regular and debug mode (see https://github.com/rust-lang/rust-analyzer/issues/13208)
    let input = if let Ok(input) = std::fs::read_to_string(&path) {
        input
    } else {
        std::fs::read_to_string(format!("aoc22/{path}")).unwrap()
    };

    let (mut min, mut max) = (0, 0);
    let sensor_beacon = input
        .lines()
        .map(|line| parse_sensor(line, &mut min, &mut max))
        .collect::<Vec<_>>();

    let mut interesting_row = (
        vec![false; (max - min).unsigned_abs() as usize],
        line_of_interest,
    );
    sensor_beacon
        .iter()
        .filter(|&&((_, y), r, _)| r >= (interesting_row.1 - y).abs())
        .for_each(|&(sensor, range, _)| {
            for i in 1..=range {
                if sensor.1 - i == interesting_row.1 || sensor.1 + i == interesting_row.1 {
                    for j in 0..=(range - i) {
                        interesting_row.0[(min.abs() + sensor.0 - j) as usize] = true;
                        interesting_row.0[(min.abs() + sensor.0 + j) as usize] = true;
                    }
                    break;
                }
            }
        });

    // Remove all fields occupied by sensors and beacons
    for &((sensor_x, sensor_y), _, (beacon_x, beacon_y)) in &sensor_beacon {
        if beacon_y == interesting_row.1 {
            interesting_row.0[(min.abs() + beacon_x) as usize] = false;
        }
        if sensor_y == interesting_row.1 {
            interesting_row.0[(min.abs() + sensor_x) as usize] = false;
        }
    }
    interesting_row
        .0
        .into_iter()
        .fold(0, |acc, x| if x { acc + 1 } else { acc })
}

pub fn run2<P>(path: P, max_width: i32, max_height: i32) -> usize
where
    P: AsRef<Path> + std::fmt::Display,
{
    // Annoying helper to be able to run the tests in regular and debug mode (see https://github.com/rust-lang/rust-analyzer/issues/13208)
    let input = if let Ok(input) = std::fs::read_to_string(&path) {
        input
    } else {
        std::fs::read_to_string(format!("aoc22/{path}")).unwrap()
    };

    let (mut min, mut max) = (0, 0);
    let sensor_beacon = input
        .lines()
        .map(|line| parse_sensor(line, &mut min, &mut max))
        .collect::<Vec<_>>();

    for row_idx in 0..=max_height {
        let mut detection_ranges: Vec<Range<i32>> = sensor_beacon
            .iter()
            .filter(|&&((_, y), r, _)| r >= (row_idx - y).abs())
            .map(|&((sensor_x, sensor_y), range, _)| {
                let detection_dist = range - (row_idx - sensor_y).abs();
                sensor_x - detection_dist..sensor_x + detection_dist + 1
            })
            .collect();

        let mut sensors: Vec<Range<i32>> = sensor_beacon
            .iter()
            .filter(|&&((_, y), _, _)| row_idx == y)
            .map(|&((sensor_x, _), _, _)| sensor_x..sensor_x + 1)
            .collect();
        let mut beacons: Vec<Range<i32>> = sensor_beacon
            .iter()
            .filter(|&&(_, _, (_, y))| row_idx == y)
            .map(|&((beacon_x, _), _, _)| beacon_x..beacon_x + 1)
            .collect();

        detection_ranges.append(&mut sensors);
        detection_ranges.append(&mut beacons);
        detection_ranges.sort_by(|left, right| left.start.cmp(&right.start));
        let merged_ranges: Vec<_> = merge_ranges(detection_ranges).collect();
        // Check if the gap is at the beginning of the row
        if merged_ranges[0].start <= 0 && merged_ranges[0].end < max_width {
            println!("x: {}, y: {}", merged_ranges[0].end, row_idx);
            return 4_000_000 * merged_ranges[0].end as usize + row_idx as usize;
        }
        // Check if the gap is at the end of the row
        if merged_ranges[0].start > 0 && merged_ranges[0].end >= max_width {
            println!("x: {}, y: {}", merged_ranges[0].start, row_idx);
            return 4_000_000 * merged_ranges[0].start as usize + row_idx as usize;
        }
        // Check if the gap is somewhere in the middle of the row
        if merged_ranges.len() > 1 {
            println!("x: {}, y: {}", merged_ranges[0].end, row_idx);
            return 4_000_000 * merged_ranges[0].end as usize + row_idx as usize;
        }
    }
    panic!("No empty space was found");
}

// Parses a line and returns the x,y coordinates of the sensor and the beacon
fn parse_sensor(line: &str, min: &mut i32, max: &mut i32) -> ((i32, i32), i32, (i32, i32)) {
    let mut chars = line.chars().enumerate().skip(13);
    let idx_comma = chars.find(|&(_, c)| c == ',').unwrap().0;
    let mut chars = chars.skip(4); // Skip ", y=" and the first digit
    let idx_colon = chars.find(|&(_, c)| c == ':').unwrap().0;
    let mut chars = chars.skip(25); // Skip ": closest beacon is at x=" and the first digit
    let idx_last_comma = chars.find(|&(_, c)| c == ',').unwrap().0;

    let sensor_x = line[12..idx_comma].parse::<i32>().unwrap();
    let sensor_y = line[idx_comma + 4..idx_colon].parse::<i32>().unwrap();
    let beacon_x = line[idx_colon + 25..idx_last_comma].parse::<i32>().unwrap();
    let beacon_y = line[idx_last_comma + 4..].parse::<i32>().unwrap();

    let detection_range = (sensor_x - beacon_x).abs() + (sensor_y - beacon_y).abs();

    *min = (*min).min(sensor_x - detection_range);
    *max = (*max).max(sensor_x + detection_range);

    ((sensor_x, sensor_y), detection_range, (beacon_x, beacon_y))
}

use std::cmp;
use std::ops::Range;

struct MergedRanges<I> {
    values: I,
    last: Option<Range<i32>>,
}

fn merge_ranges<I>(iterator: I) -> MergedRanges<I::IntoIter>
where
    I: IntoIterator<Item = Range<i32>>,
{
    let mut values = iterator.into_iter();
    let last = values.next();

    MergedRanges { values, last }
}

impl<I> Iterator for MergedRanges<I>
where
    I: Iterator<Item = Range<i32>>,
{
    type Item = Range<i32>;

    fn next(&mut self) -> Option<Range<i32>> {
        if let Some(mut last) = self.last.clone() {
            for new in &mut self.values {
                if last.end < new.start {
                    self.last = Some(new);
                    return Some(last);
                }

                last.end = cmp::max(last.end, new.end);
            }

            self.last = None;
            return Some(last);
        }

        None
    }
}

/*
fn main() {
    let mut v = vec![3..6, 1..5, 7..11, 9..12, 4..8];

    v.sort_by(|left, right| left.start.cmp(&right.start));
    let merged: Vec<_> = merge_ranges(v).collect();

    for range in &merged {
        print!(" {:?}", range);
    }
    println!("");
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let part1 = run1("input/examples/15/1.txt", 10);
        assert_eq!(part1, 26);
        let part2 = run2("input/examples/15/1.txt", 20, 20);
        assert_eq!(part2, 56_000_011);
    }

    #[test]
    fn test_input() {
        let part1 = run1("input/15.txt", 2_000_000);
        assert_eq!(part1, 4_919_281);
        let part2 = run2("input/15.txt", 4_000_000, 4_000_000);
        assert_eq!(part2, 12_630_143_363_767);
    }
}
