use crate::solutions::day8::Direction::{Down, Left, Right, Up};
use crate::util::PuzzleInput;
use std::cmp::max;

type Map = Vec<Vec<i64>>;
type Pos = (usize, usize);

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn apply(&self, (row, col): Pos) -> Option<Pos> {
        match self {
            Up if row > 0 => Some((row - 1, col)),
            Down => Some((row + 1, col)),
            Left if col > 0 => Some((row, col - 1)),
            Right => Some((row, col + 1)),
            _ => None,
        }
    }

    fn all() -> [Direction; 4] {
        [Up, Down, Left, Right]
    }
}

fn map_get(map: &Map, pos: Pos) -> Option<&i64> {
    map.get(pos.0).and_then(|row| row.get(pos.1))
}

fn parse_input(puzzle_input: &PuzzleInput) -> Map {
    puzzle_input
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap().into())
                .collect::<Vec<i64>>()
        })
        .collect()
}

pub fn part1(puzzle_input: &PuzzleInput) -> i64 {
    fn tree_visible_from_edge(
        map: &Map,
        tree_height: i64,
        tree_pos: Pos,
        direction: &Direction,
    ) -> bool {
        let Some(mut curr_pos) = direction.apply(tree_pos) else {
            return true;
        };

        while let Some(cell) = map_get(map, curr_pos) {
            if *cell >= tree_height {
                return false;
            }

            let Some(new_pos) = direction.apply(curr_pos)  else {
                break;
            };

            curr_pos = new_pos;
        }

        true
    }

    let map = parse_input(puzzle_input);
    let mut visible_trees: i64 = 0;

    for (row_index, line) in map.iter().enumerate() {
        for (col_index, cell) in line.iter().enumerate() {
            if Direction::all()
                .iter()
                .any(|d| tree_visible_from_edge(&map, *cell, (row_index, col_index), d))
            {
                visible_trees += 1;
            };
        }
    }

    visible_trees
}

pub fn part2(puzzle_input: &PuzzleInput) -> i64 {
    fn calc_scenic_score(map: &Map, tree_height: i64, tree_pos: Pos, direction: &Direction) -> i64 {
        let Some(mut curr_pos) = direction.apply(tree_pos) else {
            return 0;
        };

        let mut distance: i64 = 0;

        while let Some(cell) = map_get(map, curr_pos) {
            distance += 1;

            if *cell >= tree_height {
                break;
            }

            let Some(new_pos) = direction.apply(curr_pos) else {
                break;
            };

            curr_pos = new_pos;
        }

        distance
    }

    let map = parse_input(puzzle_input);
    let mut max_scenic_score: i64 = 0;

    for (row_index, line) in map.iter().enumerate() {
        for (col_index, cell) in line.iter().enumerate() {
            let scenic_score = Direction::all().iter().fold(1, |acc, dir| {
                acc * calc_scenic_score(&map, *cell, (row_index, col_index), dir)
            });

            max_scenic_score = max(max_scenic_score, scenic_score)
        }
    }

    max_scenic_score
}
