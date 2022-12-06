use crate::util::PuzzleInput;
use regex::Regex;

fn check_overlap(
    puzzle_input: &PuzzleInput,
    overlap_condition: impl Fn(i64, i64, i64, i64) -> bool,
) -> i64 {
    fn to_i64(str: &str) -> i64 {
        str.parse().unwrap()
    }

    let line_regex = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();

    let mut counter: i64 = 0;

    for line in puzzle_input {
        if let Some(captures) = line_regex.captures(line) {
            let start1: i64 = to_i64(&captures[1]);
            let end1: i64 = to_i64(&captures[2]);
            let start2: i64 = to_i64(&captures[3]);
            let end2: i64 = to_i64(&captures[4]);

            if overlap_condition(start1, end1, start2, end2) {
                counter += 1;
            }
        }
    }

    counter
}

pub fn part1(puzzle_input: &PuzzleInput) -> i64 {
    check_overlap(puzzle_input, |start1, end1, start2, end2| {
        (start1 <= start2 && end1 >= end2) || (start2 <= start1 && end2 >= end1)
    })
}

pub fn part2(puzzle_input: &PuzzleInput) -> i64 {
    check_overlap(puzzle_input, |start1, end1, start2, end2| {
        (start1..=end1).contains(&start2) || (start2..=end2).contains(&start1)
    })
}
