use crate::util::PuzzleInput;

fn parse_input(lines: &[String]) -> Vec<Option<i64>> {
    lines.iter().map(|l| l.parse::<i64>().ok()).collect()
}

pub fn part1(puzzle_input: &PuzzleInput) -> i64 {
    let puzzle_input = parse_input(puzzle_input);

    let mut max = None;
    let mut acc = 0;

    for cal in puzzle_input {
        match cal {
            Some(num) => acc += num,
            None => {
                if max.is_none() || max.filter(|curr_max| *curr_max < acc).is_some() {
                    max = Some(acc);
                }
                acc = 0
            }
        }
    }

    max.unwrap()
}

pub fn part2(puzzle_input: &PuzzleInput) -> i64 {
    let mut res = parse_input(puzzle_input)
        .split(Option::is_none)
        .map(|elf| elf.iter().flatten().sum())
        .collect::<Vec<_>>();

    res.sort();
    res.iter().rev().take(3).sum()
}
