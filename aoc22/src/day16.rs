use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    path::Path,
};

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
    println!();
    println!("read:");
    print_valves(&valves);

    let useless_valves: Vec<_> = valves
        .iter()
        .filter_map(|(s, v)| {
            if v.flow_rate == 0 {
                Some(s.clone())
            } else {
                None
            }
        })
        .collect();
    println!();
    println!("useless: {useless_valves:?}");

    for useless_name in &useless_valves {
        let useless_leads_to = valves.get(useless_name).unwrap().leads_to.clone();
        for (valve_name, v) in &mut valves {
            println!("replacing {useless_name} in {valve_name}");
            v.replace(valve_name, useless_name, &useless_leads_to);
        }
        // Valve AA can never be removed, because it is the starting point
        if useless_name != "AA" {
            let _ = valves.remove(useless_name);
        }
    }
    println!();
    println!("reduced:");
    print_valves(&valves);

    /*
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

    println!("{valves:?}");*/
    (0, 0)
}

#[derive(Debug, Clone)]
struct Valve {
    flow_rate: usize,
    leads_to: Vec<(String, usize)>, // Vec<(Name of valve, cost to reach it)>
}

impl Valve {
    // Checks if any of the valves that self leads to is the useless valve that is replaced
    // If it found it, it replaces it with an equivalent direct connection
    fn replace(
        &mut self,
        name_self: &str,
        useless_name: &str,
        useless_leads_to: &Vec<(String, usize)>,
    ) {
        let mut cost = 1;
        let mut replace_idx = None;
        for (no, (valve_name, valve_cost)) in self.leads_to.iter().enumerate() {
            if valve_name == useless_name {
                replace_idx = Some(no);
                cost = *valve_cost;
                break;
            }
        }
        //println!("before: {:?}", self.leads_to);
        if let Some(idx) = replace_idx {
            self.leads_to.remove(idx);
        } else {
            return;
        }
        // println!("after: {:?}", self.leads_to);
        let mut new_paths = useless_leads_to.clone();
        // println!("new_paths: {new_paths:?}");
        for lead in &mut new_paths {
            lead.1 += cost;
        }
        //   println!("new_paths: {new_paths:?}");
        self.leads_to.append(&mut new_paths);

        // Sort the valves that can be reached alphabetically. Duplicates are sorted by the cost to reach them
        // It never makes sense to spend more time to reach the same valve, so those duplicates with a higher cost are removed
        self.leads_to.sort_by(|(sa, ca), (sb, cb)| {
            let str_ord = sa.cmp(sb);
            match str_ord {
                Ordering::Less => Ordering::Less,
                Ordering::Equal => ca.cmp(cb),
                Ordering::Greater => Ordering::Greater,
            }
        });
        self.leads_to.dedup_by_key(|(s, _)| s.clone());
        let _ = self
            .leads_to
            .drain_filter(|(name, _)| name_self == name)
            .collect::<Vec<_>>();
    }
}

struct TunnelRunner {
    time: usize,
    pressure: usize,
    max_pressure: usize,
    remaining_pressure: usize,
    opened_valves: HashSet<String>,
    valves: HashMap<String, Valve>,
    path: Vec<(Valve, bool, usize)>, // Path of crossed valves, if they were opened in that step and the number of the valve that was walked to next
}

impl TunnelRunner {
    fn new(time: usize, valves: HashMap<String, Valve>) -> Self {
        TunnelRunner {
            time,
            pressure: 0,
            max_pressure: 0,
            remaining_pressure: usize::MAX,
            opened_valves: HashSet::new(),
            valves,
            path: vec![], // Path of crossed valves, if they were opened in that step and the number of the valve that was walked to next
        }
    }
    fn run(&mut self) -> usize {
        while self.time > 0 {
            let current_valve = if self.path.is_empty() {
                self.valves.get("AA").expect("Valve AA is missing").clone()
            } else {
                let next = self.path[self.path.len() - 1].0.leads_to[0];
                self.valves.get("AA").expect("Valve AA is missing").clone()
            };
            self.path.push((current_valve, true, 0));
            self.opened_valves.insert("AA".to_string());
        }
        self.max_pressure
    }
}

fn print_valves(map: &HashMap<String, Valve>) {
    let mut vec: Vec<_> = map.iter().collect();
    vec.sort_by(|(na, _), (nb, _)| na.cmp(nb));
    for (n, v) in vec {
        println!(
            "Valve {} has flow rate={}; tunnels lead to valves{:?}",
            n, v.flow_rate, v.leads_to
        );
    }
    println!()
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
