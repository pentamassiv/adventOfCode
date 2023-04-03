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
    // println!("({min}, {max})");

    let mut interesting_row = (
        vec![false; (max - min).unsigned_abs() as usize],
        line_of_interest,
    );
    sensor_beacon
        .iter()
        .filter(|&&((x, y), r, _)| {
            /*println!("sensor: {x},{y}");
            println!("range: {r}");
            println!("filter: {}", r >= (interesting_row.1 - y).abs());*/
            r >= (interesting_row.1 - y).abs()
        })
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
    /*  for no in min..max {
        if no % 5 == 0 {
            print!("{no:>5}");
        }
    }
    println!();*/

    // Remove all fields occupied by sensors and beacons
    for &((sensor_x, sensor_y), _, (beacon_x, beacon_y)) in &sensor_beacon {
        if beacon_y == interesting_row.1 {
            interesting_row.0[(min.abs() + beacon_x) as usize] = false;
        }
        if sensor_y == interesting_row.1 {
            interesting_row.0[(min.abs() + sensor_x) as usize] = false;
        }
    }
    //println!();
    interesting_row.0.into_iter().fold(0, |acc, x| {
        if x {
            //print!("#");
            acc + 1
        } else {
            //print!(".");
            acc
        }
    })
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
    // println!("({min}, {max})");

    for solution_y in 0..max_height {
        if solution_y % 10_000 == 0 {
            println!("{solution_y}");
        };
        let mut interesting_row = (vec![false; (max - min).unsigned_abs() as usize], solution_y);
        sensor_beacon
            .iter()
            .filter(|&&((_, y), r, _)| {
                /*println!("sensor: {x},{y}");
                println!("range: {r}");
                println!("filter: {}", r >= (interesting_row.1 - y).abs());*/
                r >= (interesting_row.1 - y).abs()
            })
            .for_each(|&(sensor, range, _)| {
                for i in 0..=range {
                    if sensor.1 - i == interesting_row.1 || sensor.1 + i == interesting_row.1 {
                        for j in 0..=(range - i) {
                            interesting_row.0[(min.abs() + sensor.0 - j) as usize] = true;
                            interesting_row.0[(min.abs() + sensor.0 + j) as usize] = true;
                        }
                        break;
                    }
                }
            });

        let interesting_part = &interesting_row.0
            [min.unsigned_abs() as usize..=min.unsigned_abs() as usize + max_width as usize];
        if let Some(solution_x) = interesting_part.iter().position(|&b| !b) {
            return solution_x * 4_000_000 + solution_y as usize;
        }
    }
    0
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

    /*println!("sensor: {sensor_x}, {sensor_y}");
    println!("beacon: {beacon_x}, {beacon_y}");
    println!("range:  {detection_range}");*/
    ((sensor_x, sensor_y), detection_range, (beacon_x, beacon_y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_example() {
        let part1 = run1("input/examples/15/1.txt", 10);
        assert_eq!(part1, 26);
        let part2 = run2("input/examples/15/1.txt", 20, 20);
        assert_eq!(part2, 56_000_011);
    }

    #[ignore]
    #[test]
    fn test_input() {
        let part1 = run1("input/15.txt", 2_000_000);
        assert_eq!(part1, 4_919_281);
        let part2 = run2("input/15.txt", 4_000_000, 4_000_000);
        assert_eq!(part2, 4_919_281);
    }
}
