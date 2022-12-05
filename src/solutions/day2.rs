use crate::solutions::day2::Outcome::{Draw, Loss, Win};
use crate::solutions::day2::Symbol::{Paper, Rock, Scissors};
use crate::util::PuzzleInput;

enum Outcome {
    Win,
    Loss,
    Draw,
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Symbol {
    Rock,
    Paper,
    Scissors,
}

impl Symbol {
    fn points(&self) -> i64 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn beats(&self) -> Symbol {
        match self {
            Rock => Scissors,
            Scissors => Paper,
            Paper => Rock,
        }
    }

    fn loses_to(&self) -> Symbol {
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }
}

fn parse_input_part1(lines: &[String]) -> Vec<(Symbol, Symbol)> {
    fn to_symbol(symbol: &str) -> Symbol {
        match symbol {
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissors,
            _ => panic!(),
        }
    }

    lines
        .iter()
        .filter_map(|l| match l.split(' ').collect::<Vec<&str>>()[..] {
            [player1, player2] => Some((to_symbol(player1), to_symbol(player2))),
            _ => None,
        })
        .collect()
}

pub fn part1(input: &[String]) -> i64 {
    parse_input_part1(input)
        .iter()
        .map(|game| {
            (match game {
                (player1, player2) if *player1 == *player2 => 3,
                (player1, player2) if player2.beats() == *player1 => 6,
                _ => 0,
            }) + game.1.points()
        })
        .sum()
}

fn parse_input_part2(lines: &[String]) -> Vec<(Symbol, Outcome)> {
    fn to_symbol(symbol: &str) -> Symbol {
        match symbol {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            _ => panic!(),
        }
    }

    fn to_outcome(outcome: &str) -> Outcome {
        match outcome {
            "X" => Loss,
            "Y" => Draw,
            "Z" => Win,
            _ => panic!(),
        }
    }

    lines
        .iter()
        .filter_map(|line| match line.split(' ').collect::<Vec<&str>>()[..] {
            [player1, outcome] => Some((to_symbol(player1), to_outcome(outcome))),
            _ => None,
        })
        .collect()
}

pub fn part2(input: &[String]) -> i64 {
    parse_input_part2(input)
        .iter()
        .map(|game| match game {
            (player1, Draw) => 3 + player1.points(),
            (player1, Win) => 6 + player1.loses_to().points(),
            (player1, Loss) => player1.beats().points(),
        })
        .sum()
}
