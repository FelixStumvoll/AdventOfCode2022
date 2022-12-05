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
    ($day:expr) => {
        println!("***** Day {} *****", $day);
        paste::paste! {
            crate::solutions::[<day $day>]::[<day $day>](&crate::util::read_input_for_day($day));
        }
    };
}
