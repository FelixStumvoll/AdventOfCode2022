use crate::util::PuzzleInput;
use std::collections::HashSet;

pub fn part1(puzzle_input: &PuzzleInput) -> i64 {
    fn char_points(item: char) -> i64 {
        match item {
            c @ 'a'..='z' => c.to_digit(36).map(|p| p - 9),
            c @ 'A'..='Z' => c.to_digit(36).map(|p| p + 17),
            _ => None,
        }
        .map_or(0, |p| p.into())
    }

    let mut points: i64 = 0;

    for line in puzzle_input {
        let (first, second) = line.split_at(line.len() / 2);
        let mut lookup: HashSet<char> = HashSet::from_iter(first.chars());

        for second_char in second.chars() {
            if lookup.contains(&second_char) {
                points += char_points(second_char);
                lookup.remove(&second_char);
            }
        }
    }

    points
}
