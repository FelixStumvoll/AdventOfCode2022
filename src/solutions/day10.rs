use crate::solutions::day10::Instruction::{Add, ExecStart, Noop};
use crate::solutions::day10::Pixel::{Off, On};
use crate::util::PuzzleInput;
use std::fmt::{Display, Formatter};
use std::fmt::{Result, Write};
use std::iter::repeat;

enum Instruction {
    Noop,
    ExecStart,
    Add(i64),
}

#[derive(Copy, Clone)]
enum Pixel {
    On,
    Off,
}

impl Display for Pixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            On => write!(f, "█"),
            Off => write!(f, "."),
        }?;

        Ok(())
    }
}

fn parse_input(puzzle_input: &PuzzleInput) -> Vec<Instruction> {
    let mut result: Vec<Instruction> = Vec::new();

    for line in puzzle_input {
        let (op, amount) = line.split_at(4);

        match op {
            "noop" => result.push(Noop),
            "addx" => {
                result.push(ExecStart);
                result.push(Add(amount[1..].parse().unwrap()));
            }
            _ => unreachable!(),
        }
    }

    result
}

pub fn part1(puzzle_input: &PuzzleInput) -> i64 {
    let instructions = parse_input(puzzle_input);

    const CHECK_CYCLES: [i64; 6] = [20, 60, 100, 140, 180, 220];

    let mut signal_strength = 0;
    let mut x: i64 = 1;
    let mut cycle_count: i64 = 0;

    for instruction in &instructions {
        cycle_count += 1;
        if CHECK_CYCLES.contains(&cycle_count) {
            signal_strength += x * cycle_count;
        }

        if let Add(value) = instruction {
            x += value;
        }
    }

    signal_strength
}

fn zip3<A, B, C>(
    a: impl IntoIterator<Item = A>,
    b: impl IntoIterator<Item = B>,
    c: impl IntoIterator<Item = C>,
) -> impl Iterator<Item = (A, B, C)> {
    a.into_iter().zip(b).zip(c).map(|((a, b), c)| (a, b, c))
}

pub fn part2(puzzle_input: &PuzzleInput) -> String {
    let instructions = parse_input(puzzle_input);

    const SCREEN_HEIGHT: i64 = 6;
    const SCREEN_WIDTH: i64 = 40;
    const SCREEN_SIZE: i64 = SCREEN_HEIGHT * SCREEN_WIDTH;
    let mut screen = [Off; SCREEN_SIZE as usize];
    let mut x: i64 = 1;

    let rows_iter = (0i64..).flat_map(|i| repeat(i).take(SCREEN_WIDTH as usize));

    for (instruction, crt_head, row) in zip3(&instructions, 0..SCREEN_SIZE, rows_iter) {
        let x_in_row = x + row * SCREEN_WIDTH;

        if crt_head.abs_diff(x_in_row) <= 1 {
            screen[crt_head as usize] = On;
        }

        if let Add(value) = instruction {
            x += value
        }
    }

    let mut result = String::new();
    writeln!(result).unwrap();

    for i in 0..SCREEN_SIZE {
        if i != 0 && i % SCREEN_WIDTH == 0 {
            writeln!(result).unwrap();
        }
        write!(result, "{}", screen[i as usize]).unwrap();
    }

    result
}
