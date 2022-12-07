use crate::util::PuzzleInput;
use std::collections::HashSet;
use std::iter::zip;

pub fn part1(puzzle_input: &PuzzleInput) -> i64 {
    let input: Vec<char> = puzzle_input[0].chars().collect();

    for (chars, count) in zip(input.windows(4), 4..) {
        if let [a, b, c, d] = chars {
            if a != b && a != c && a != d && b != c && b != d && c != d {
                return count;
            }
        }
    }

    0
}

pub fn part2(puzzle_input: &PuzzleInput) -> i64 {
    let input: Vec<char> = puzzle_input[0].chars().collect();

    for (chars, count) in zip(input.windows(14), 14..) {
        let characters: HashSet<&char> = HashSet::from_iter(chars.iter());

        if characters.len() == 14 {
            return count;
        }
    }

    0
}
