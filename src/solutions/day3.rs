use crate::util::PuzzleInput;
use std::collections::{HashMap, HashSet};
use std::iter::{repeat, zip};

fn item_points(item: char) -> i64 {
    match item {
        c @ 'a'..='z' => c.to_digit(36).map(|p| p - 9),
        c @ 'A'..='Z' => c.to_digit(36).map(|p| p + 17),
        _ => None,
    }
    .map_or(0, |p| p.into())
}

pub fn part1(puzzle_input: &PuzzleInput) -> i64 {
    let mut points: i64 = 0;

    for line in puzzle_input {
        let (first, second) = line.split_at(line.len() / 2);
        let mut lookup: HashSet<char> = HashSet::from_iter(first.chars());

        for second_char in second.chars() {
            if lookup.contains(&second_char) {
                points += item_points(second_char);
                lookup.remove(&second_char);
            }
        }
    }

    points
}

pub fn part2(puzzle_input: &PuzzleInput) -> i64 {
    let mut points: i64 = 0;

    for group in puzzle_input.chunks(3) {
        if let [a, b, c] = group {
            let mut map: HashMap<char, bool> = HashMap::from_iter(zip(a.chars(), repeat(false)));

            for b_item in b.chars() {
                if map.get(&b_item).is_some() {
                    map.insert(b_item, true);
                }
            }

            for c_item in c.chars() {
                if let Some(true) = map.get(&c_item) {
                    points += item_points(c_item);
                    map.remove(&c_item);
                }
            }
        }
    }

    points
}
