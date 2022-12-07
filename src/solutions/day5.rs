use crate::util::PuzzleInput;
use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::iter::zip;

type Stacks = HashMap<usize, VecDeque<char>>;
type StackPos = usize;

struct MoveInstruction {
    from: StackPos,
    to: StackPos,
    amount: usize,
}

fn parse_input(puzzle_input: &PuzzleInput) -> (Stacks, Vec<MoveInstruction>) {
    let mut stacks: Stacks = HashMap::new();

    let mut lines_to_skip = 1;

    'stack_parsing: for line in puzzle_input {
        lines_to_skip += 1;

        for (char, stack) in zip(line.chars().skip(1).step_by(4), 1..) {
            match char {
                'A'..='Z' => stacks.entry(stack).or_default().push_back(char),
                '1' => break 'stack_parsing,
                _ => {}
            }
        }
    }

    let mut moves: Vec<MoveInstruction> = Vec::new();
    let move_regex = Regex::new(r"move (\d*) from (\d*) to (\d*)").unwrap();

    fn to_usize(str: &str) -> usize {
        str.parse().unwrap()
    }

    for line in puzzle_input.iter().skip(lines_to_skip) {
        if let Some(captures) = move_regex.captures(line) {
            moves.push(MoveInstruction {
                amount: to_usize(&captures[1]),
                from: to_usize(&captures[2]),
                to: to_usize(&captures[3]),
            });
        }
    }

    (stacks, moves)
}

fn apply_moves(stacks: &mut Stacks, moves: &Vec<MoveInstruction>, keep_order: bool) -> String {
    let mut cache: Vec<char> = Vec::new();

    for MoveInstruction { from, to, amount } in moves {
        let from_stack = stacks.get_mut(from).unwrap();

        for _ in 1..=*amount {
            if let Some(top_item) = from_stack.pop_front() {
                if keep_order {
                    cache.push(top_item);
                } else {
                    cache.insert(0, top_item);
                }
            }
        }

        let to_stack = stacks.get_mut(to).unwrap();

        while let Some(cache_entry) = cache.pop() {
            to_stack.push_front(cache_entry);
        }
    }

    let mut result = String::new();

    for i in 1..=stacks.len() {
        if let Some(stack) = stacks.get(&i) {
            if let Some(top) = stack.front() {
                result.push(*top);
            }
        }
    }

    result
}

pub fn part1(puzzle_input: &PuzzleInput) -> String {
    let (mut stacks, moves) = parse_input(puzzle_input);
    apply_moves(&mut stacks, &moves, false)
}

pub fn part2(puzzle_input: &PuzzleInput) -> String {
    let (mut stacks, moves) = parse_input(puzzle_input);
    apply_moves(&mut stacks, &moves, true)
}
