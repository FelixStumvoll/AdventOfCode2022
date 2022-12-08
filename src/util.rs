use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub type PuzzleInput = Vec<String>;

pub fn read_input_for_day(day: usize) -> PuzzleInput {
    let path = format!("./input/day{}.txt", day);
    read_as_lines(Path::new(&path))
}

fn read_as_lines(path: &Path) -> PuzzleInput {
    let file = File::open(path).unwrap();
    io::BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .collect()
}

macro_rules! run_day {
    ($day:literal) => {
        println!("***** Day {} *****", $day);
        paste::paste! {
            let input = &util::read_input_for_day($day);
            let part1 = solutions::[<day $day>]::part1(&input);
            let part2 = solutions::[<day $day>]::part2(&input);
            println!("Part 1: {}", part1);
            println!("Part 2: {}", part2);
        }
    };

    ($day:expr, 1) => {
        println!("***** Day {} *****", $day);
        paste::paste! {
            let input = &util::read_input_for_day($day);
            let part1 = solutions::[<day $day>]::part1(&input);
            println!("Part 1: {}", part1);
        }
    };

    ($day:expr, 2) => {
        println!("***** Day {} *****", $day);
        paste::paste! {
            let input = &util::read_input_for_day($day);
            let part2 = solutions::[<day $day>]::part2(&input);
            println!("Part 2: {}", part2);
        }
    };
}
