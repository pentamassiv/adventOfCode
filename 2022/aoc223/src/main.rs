#![feature(drain_filter)]

fn main() {
    // Read the input file
    let input = std::fs::read_to_string("2022/aoc223/input").unwrap();

    let mut compartments;
    let mut sum_prio = 0;
    let mut sum_badge = 0;
    let mut badge_candidates = vec![];
    // Iterator over the lines
    for (no, rucksack) in input.lines().enumerate() {
        compartments = rucksack.split_at(rucksack.len() / 2);
        if no % 3 == 0 {
            for item_type in rucksack.chars() {
                badge_candidates.push(item_type);
            }
        } else {
            badge_candidates = badge_candidates
                .drain_filter(|item_type| rucksack.contains(*item_type))
                .collect::<Vec<_>>();
        }
        if no % 3 == 2 {
            sum_badge += item_prio(badge_candidates[0]);
            badge_candidates.clear();
        }

        //println!("{} {}", compartments.0, compartments.1);
        for item_type in compartments.0.chars() {
            if compartments.1.contains(item_type) {
                //println!("{}({})", item_prio(item_type), item_type);
                sum_prio += item_prio(item_type);
                break;
            }
        }
    }
    println!("{sum_prio}");
    println!("{sum_badge}");
}

fn item_prio(item_type: char) -> u32 {
    if item_type.is_lowercase() {
        u32::from(item_type) - u32::from('a') + 1
    } else if item_type.is_uppercase() {
        u32::from(item_type) - u32::from('A') + 27
    } else {
        print!("Found weird item_type: {item_type}");
        0
    }
}
