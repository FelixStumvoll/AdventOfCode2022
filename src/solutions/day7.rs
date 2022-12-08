use crate::util::PuzzleInput;
use std::collections::HashMap;

fn parse_file_tree(puzzle_input: &PuzzleInput) -> HashMap<String, i128> {
    let mut path_map: HashMap<String, i128> = HashMap::new();
    let mut current_path: Vec<String> = Vec::new();

    for line in puzzle_input {
        match &line[..] {
            "$ cd /" => {
                current_path.clear();
                current_path.push("".into());
            }
            "$ cd .." => {
                current_path.pop();
            }
            "$ ls" => {}
            l if l.starts_with('$') => {
                let file = &l[5..];
                current_path.push(file.into());
            }
            l if l.starts_with("dir") => {}
            size_info => {
                if let Some(size) = size_info.split(' ').next() {
                    let size: i128 = size.parse().unwrap();

                    let mut temp_path = current_path.clone();

                    for _ in 0..temp_path.len() {
                        let map_v = path_map.entry(temp_path.join("/")).or_default();
                        *map_v += size;
                        temp_path.pop();
                    }
                }
            }
        }
    }

    path_map
}

pub fn part1(puzzle_input: &PuzzleInput) -> i128 {
    parse_file_tree(puzzle_input)
        .into_values()
        .filter(|v| *v <= 100000)
        .sum()
}

pub fn part2(puzzle_input: &PuzzleInput) -> i128 {
    let dir_sizes = parse_file_tree(puzzle_input);
    let root_size = dir_sizes[""];

    const FULL_SPACE: i128 = 70_000_000;
    const REQUIRED_SPACE: i128 = 30_000_000;
    let required_space = REQUIRED_SPACE - (FULL_SPACE - root_size);

    let mut paths: Vec<i128> = Vec::from_iter(dir_sizes.into_values());
    paths.sort();

    for size in &paths {
        if *size > required_space {
            return *size;
        }
    }

    unreachable!()
}
