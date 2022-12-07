use std::collections::HashMap;

pub fn run(path: &str) -> (u32, u32) {
    // Read the input file
    let input = std::fs::read_to_string(path).unwrap();

    let mut file_path: Vec<String> = vec![];
    let mut files = HashMap::new();
    let mut dirs = HashMap::new();
    dirs.insert(vec![], 0); // Insert root folder

    // Iterator over the lines
    for line in input.lines() {
        match parse_line(line) {
            Line::Command(command) => match command {
                Command::Root => file_path.clear(),
                Command::MoveOut => {
                    file_path.pop();
                }
                Command::List => {}
                Command::MoveIn(path) => file_path.push(path),
            },
            Line::Output(output) => match output {
                Output::File(size, file_name) => {
                    let mut path = file_path.clone();
                    path.push(file_name);
                    files.insert(path, size);
                    //dirs.insert(build_path(file_path.clone(), file_name), size);
                }
                Output::Dir(folder_name) => {
                    let mut path = file_path.clone();
                    path.push(folder_name);
                    dirs.insert(path, 0);
                    //dirs.insert(build_path(file_path.clone(), folder_name), 0);
                }
            },
        }
    }

    // Calculate folder sizes
    for (file_path, size) in files {
        for no in 0..file_path.len() {
            *dirs.get_mut(&file_path[0..no]).unwrap() += size;
        }
    }
    let solution1 = dirs.values().filter(|v| **v <= 100_000).sum::<u32>();

    let total_space = 70_000_000;
    let needed_space = 30_000_000;
    let free_space = total_space - dirs.get(&vec![]).unwrap();
    let mut candidate_folders = dirs
        .values()
        .filter(|v| **v >= needed_space - free_space)
        .collect::<Vec<_>>();
    candidate_folders.sort();

    let solution2 = *candidate_folders[0];
    (solution1, solution2)
}

fn parse_line(line: &str) -> Line {
    if line.starts_with("$ ") {
        Line::Command(parse_command(line))
    } else {
        Line::Output(parse_output(line))
    }
}

fn parse_command(line: &str) -> Command {
    match line {
        "$ cd /" => Command::Root,
        "$ cd .." => Command::MoveOut,
        "$ ls" => Command::List,
        &_ => Command::MoveIn(line.strip_prefix("$ cd ").unwrap().to_string()),
    }
}

fn parse_output(line: &str) -> Output {
    let l = line.split_ascii_whitespace().collect::<Vec<_>>();
    let left = l[0];
    let right = l[1];
    if left == "dir" {
        Output::Dir(right.to_string())
    } else {
        Output::File(left.parse().unwrap(), right.to_string())
    }
}

#[derive(Debug)]
enum Command {
    MoveIn(String),
    MoveOut,
    Root,
    List,
}

#[derive(Debug)]
enum Output {
    File(u32, String),
    Dir(String),
}

#[derive(Debug)]
enum Line {
    Command(Command),
    Output(Output),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let (part1, part2) = run("input/examples/7/1.txt");
        assert_eq!(part1, 95_437);
        assert_eq!(part2, 24_933_642);
    }

    #[test]
    fn test_input() {
        let (part1, part2) = run("input/7.txt");
        assert_eq!(part1, 1_582_412);
        assert_eq!(part2, 3_696_336);
    }
}
