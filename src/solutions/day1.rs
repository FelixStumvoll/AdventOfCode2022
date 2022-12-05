use crate::util::PuzzleInput;

fn parse_input(lines: &[String]) -> Vec<Option<i128>> {
    lines.iter().map(|l| l.parse::<i128>().ok()).collect()
}

pub fn day1(puzzle_input: &PuzzleInput) {
    let input = parse_input(puzzle_input);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input))
}

fn part1(lines: &[Option<i128>]) -> i128 {
    let mut max = None;
    let mut acc = 0;

    for cal in lines {
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

    max.unwrap_or(0)
}

fn part2(lines: &[Option<i128>]) -> i128 {
    let mut res = lines
        .split(Option::is_none)
        .map(|elf| elf.iter().flatten().sum())
        .collect::<Vec<_>>();

    res.sort();
    res.iter().rev().take(3).sum()
}
