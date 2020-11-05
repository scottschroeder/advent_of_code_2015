use anyhow::Result;
use aoc::grid::{compass::Direction, point::Point};
use std::collections::HashMap;

type Loc = Point<i64>;

pub fn part1(input: &str) -> Result<String> {
    let instructions = parse_input(input)?;
    let mut count = HashMap::new();

    for p in houses(instructions.iter()) {
        let x = count.entry(p).or_insert(0);
        *x += 1;
    }
    Ok(format!("{:?}", count.len()))
}

pub fn part2(input: &str) -> Result<String> {
    let instructions = parse_input(input)?;
    let mut count = HashMap::new();

    for p in houses(instructions.iter().step_by(2)) {
        let x = count.entry(p).or_insert(0);
        *x += 1;
    }
    for p in houses(instructions.iter().skip(1).step_by(2)) {
        let x = count.entry(p).or_insert(0);
        *x += 1;
    }
    Ok(format!("{:?}", count.len()))
}

fn parse_input(s: &str) -> Result<Vec<Direction>> {
    s.chars().map(parse_direction).collect()
}

fn parse_direction(c: char) -> Result<Direction> {
    let d = match c {
        '^' => Direction::North,
        '>' => Direction::East,
        'v' => Direction::South,
        '<' => Direction::West,
        _ => anyhow::bail!("unknown direction: {:?}", c),
    };
    Ok(d)
}

fn houses<'a>(inst: impl Iterator<Item = &'a Direction> + 'a) -> impl Iterator<Item = Loc> + 'a {
    let mut p = Loc::new(0, 0);
    inst.map(move |d| {
        let last = p;
        p = p + d.delta();
        last
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day3");

    #[test]
    fn verify_p1() {
        assert_eq!(part1(INPUT).unwrap().as_str(), "2572")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(part2(INPUT).unwrap().as_str(), "2631")
    }
}
