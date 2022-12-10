use crate::solutions::day9::Direction::{Down, Left, Right, Up};
use crate::util::PuzzleInput;
use std::collections::HashSet;
use std::iter::zip;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn apply(&self, (row, col): Pos) -> Pos {
        match self {
            Up => (row - 1, col),
            Down => (row + 1, col),
            Left => (row, col - 1),
            Right => (row, col + 1),
        }
    }
}

type Pos = (i64, i64);

type Move = (Direction, usize);

fn parse_input(puzzle_input: &PuzzleInput) -> Vec<Move> {
    puzzle_input
        .iter()
        .map(|line| {
            let (dir, amount) = line.split_at(1);
            let amount: usize = amount[1..].parse().unwrap();

            let dir = match dir {
                "U" => Up,
                "D" => Down,
                "L" => Left,
                "R" => Right,
                _ => unreachable!(),
            };

            (dir, amount)
        })
        .collect()
}

fn adjust_tail((head_x, head_y): &Pos, tail @ (tail_x, tail_y): &Pos) -> Pos {
    let x_diff = head_x - *tail_x;
    let y_diff = head_y - *tail_y;

    if x_diff.abs() <= 1 && y_diff.abs() <= 1 {
        *tail
    } else {
        (tail_x + x_diff.signum(), tail_y + y_diff.signum())
    }
}

pub fn part1(puzzle_input: &PuzzleInput) -> usize {
    let moves = parse_input(puzzle_input);

    let mut head_pos: Pos = (0, 0);
    let mut tail_pos: Pos = (0, 0);

    let mut visited_positions: HashSet<Pos> = HashSet::new();

    visited_positions.insert(tail_pos);

    for (direction, amount) in &moves {
        for _ in 0..*amount {
            head_pos = direction.apply(head_pos);
            tail_pos = adjust_tail(&head_pos, &tail_pos);
            visited_positions.insert(tail_pos);
        }
    }

    visited_positions.len()
}

pub fn part2(puzzle_input: &PuzzleInput) -> usize {
    let moves = parse_input(puzzle_input);

    let mut rope: [Pos; 10] = [(0, 0); 10];

    let mut visited_positions: HashSet<Pos> = HashSet::new();
    visited_positions.insert(rope[9]);

    for (direction, amount) in &moves {
        for _ in 0..*amount {
            rope[0] = direction.apply(rope[0]);

            for (head_idx, tail_idx) in zip(0..rope.len(), 1..rope.len()) {
                rope[tail_idx] = adjust_tail(&rope[head_idx], &rope[tail_idx]);
            }

            visited_positions.insert(rope[9]);
        }
    }

    visited_positions.len()
}
