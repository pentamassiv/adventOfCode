use std::{collections::HashMap, path::Path};

pub fn run<P>(path: P) -> (i32, i32)
where
    P: AsRef<Path> + std::fmt::Display,
{
    // Annoying helper to be able to run the tests in regular and debug mode (see https://github.com/rust-lang/rust-analyzer/issues/13208)
    let input = if let Ok(input) = std::fs::read_to_string(&path) {
        input
    } else {
        std::fs::read_to_string(format!("aoc22/{path}")).unwrap()
    };

    let mut valves = HashMap::new();
    input.lines().for_each(|l| {
        let name = l[6..8].to_string();
        let flow_rate = l[23..26].split(';').next().unwrap().parse().unwrap();
        let leads_to: Vec<(String, usize)> = l[48..]
            .split_once(" ")
            .unwrap()
            .1
            .split(", ")
            .map(|s| (s.to_string(), 1))
            .collect();
        valves.insert(
            name,
            Valve {
                flow_rate,
                leads_to,
            },
        );
    });
    println!("{valves:?}");
    println!();
    println!();
    let useless_name = "AA";
    let useless_valve = &Valve {
        flow_rate: 0,
        leads_to: vec![
            ("DD".to_string(), 1),
            ("II".to_string(), 1),
            ("BB".to_string(), 1),
        ],
    };
    let _ = valves
        .drain_filter(|name, v| {
            if name == useless_name {
                true
            } else {
                v.replace((useless_name, useless_valve));
                false
            }
        })
        .collect::<Vec<_>>();

    println!("{valves:?}");
    (0, 0)
}

#[derive(Debug)]
struct Valve {
    flow_rate: usize,
    leads_to: Vec<(String, usize)>, // Vec<(Name of valve, cost to reach it)>
}

impl Valve {
    fn replace(&mut self, useless_valve: (&str, &Valve)) {
        if useless_valve.1.flow_rate > 0 {
            panic!("You can't replace a valve that has a flow rate that is greater than 0");
        }
        let mut cost = 1;
        let mut replace_idx = None;
        for (no, (valve_name, valve_cost)) in self.leads_to.iter().enumerate() {
            if valve_name == useless_valve.0 {
                replace_idx = Some(no);
                cost = *valve_cost;
                break;
            }
        }
        println!("before: {:?}", self.leads_to);
        if let Some(idx) = replace_idx {
            self.leads_to.remove(idx);
        } else {
            return;
        }
        println!("after: {:?}", self.leads_to);
        let mut new_paths = useless_valve.1.leads_to.clone();
        println!("new_paths: {new_paths:?}");
        for lead in &mut new_paths {
            lead.1 += cost;
        }
        println!("new_paths: {new_paths:?}");
        self.leads_to.append(&mut new_paths);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_example() {
        let (part1, part2) = run("input/examples/16/1.txt");
        assert_eq!(part1, 1651);
        assert_eq!(part2, 45000);
    }

    #[ignore]
    #[test]
    fn test_input() {
        let (part1, part2) = run("input/16.txt");
        assert_eq!(part1, 75_622);
        assert_eq!(part2, 213_159);
    }
}
