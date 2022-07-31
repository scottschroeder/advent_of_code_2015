use std::{any, collections::HashMap};

use anyhow::{Context, Result};
use itertools::Itertools;
use matrix::Matrix;
mod matrix;

fn parse_amount(input: &str) -> Result<i32> {
    let mut chunks = input.split(' ');
    let dir = chunks
        .next()
        .ok_or_else(|| anyhow::anyhow!("could not get gain/lose"))?;
    let scalar_str = chunks
        .next()
        .ok_or_else(|| anyhow::anyhow!("could not get number text"))?;
    let v = scalar_str
        .parse::<i32>()
        .with_context(|| format!("could not parse number: {:?}", scalar_str))?;
    let d = match dir {
        "gain" => 1,
        "lose" => -1,
        _ => anyhow::bail!("direction was not gain or lose: {:?}", dir),
    };
    Ok(d * v)
}

fn parse_line(input: &str) -> Result<(&str, &str, i32)> {
    let input = input.trim_end_matches('.');
    let mut chunks = input
        .split(" happiness units by sitting next to ")
        .flat_map(|s| s.split(" would "));

    let src = chunks
        .next()
        .ok_or_else(|| anyhow::anyhow!("could not get src"))?;
    let amount = chunks
        .next()
        .ok_or_else(|| anyhow::anyhow!("could not get gain/lose amount"))?;
    let dst = chunks
        .next()
        .ok_or_else(|| anyhow::anyhow!("could not get gain/lose amount"))?;
    let h = parse_amount(amount).context("parse number from amount")?;
    Ok((src, dst, h))
}

fn parse_input(input: &str, add_extra: bool) -> Result<Matrix> {
    let mut loc_map = HashMap::new();
    let mut loc_to_id = |loc: &str| {
        let seen = loc_map.len();
        *loc_map.entry(loc.to_string()).or_insert(seen)
    };
    let mut buf = Vec::new();

    for line in input.lines() {
        let (src, dst, h) =
            parse_line(line).with_context(|| format!("unable to parse line: {:?}", line))?;
        log::trace!("src: {} dst: {} dist: {}", src, dst, h);
        buf.push((loc_to_id(src), loc_to_id(dst), h))
    }
    let host_room = if add_extra { 1 } else { 0 };
    let mut m = Matrix::new(loc_map.len() + host_room);
    for (x, y, v) in buf {
        m.set(x, y, v);
    }
    Ok(m)
}

fn permute_distances(h_matrix: &Matrix) -> impl Iterator<Item = i32> + '_ {
    (0..h_matrix.size())
        .permutations(h_matrix.size())
        .map(move |p| {
            p.iter()
                .cycle()
                .take(h_matrix.size() + 1)
                .tuple_windows()
                .map(move |(s, d)| h_matrix.get(*s, *d) + h_matrix.get(*d, *s))
                .sum::<i32>()
        })
}

fn best_seating(input: &str, add_host: bool) -> Result<i32> {
    let m = parse_input(input, add_host)?;
    permute_distances(&m)
        .max()
        .ok_or_else(|| anyhow::anyhow!("there were no seating arrangements"))
}

pub fn part1(input: &str) -> Result<String> {
    let x = best_seating(input, false)?;
    Ok(format!("{:?}", x))
}

pub fn part2(input: &str) -> Result<String> {
    let x = best_seating(input, true)?;
    Ok(format!("{:?}", x))
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day13");
    const EX: &str = include_str!("../../../input/day13_ex");

    #[test]
    fn verify_p1() {
        assert_eq!(part1(INPUT).unwrap().as_str(), "733")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(part2(INPUT).unwrap().as_str(), "725")
    }

    #[test]
    fn check_example() {
        assert_eq!(part1(EX).unwrap().as_str(), "330")
    }
}
