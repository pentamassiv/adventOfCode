#![warn(clippy::pedantic)]

fn main() {
    // Read the input file
    let input = std::fs::read_to_string("2022/aoc223/input").unwrap();
    let input = input.lines().collect::<Vec<&str>>();
    let mut sum_prio = 0;
    let mut sum_badge = 0;

    // Get groups of three rucksacks
    for group in input.chunks(3) {
        // Check each rucksack for the duplicate item type
        for &rucksack in group {
            let (l, r) = rucksack.split_at(rucksack.len() / 2);
            sum_prio += item_prio(duplicate_item(&[l, r][..]));
        }
        // Check the group of rucksacks for the badge
        sum_badge += item_prio(duplicate_item(group));
    }
    println!("Part 1:");
    println!("{sum_prio}");
    println!("Part 2:");
    println!("{sum_badge}");
}

// Calculate the prio of an item type
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

// Find the duplicate item type
fn duplicate_item(slice: &[&str]) -> char {
    for c in slice[0].chars() {
        if slice[1..].iter().all(|x| x.contains(c)) {
            return c;
        }
    }
    panic!("No duplicate found!");
}
