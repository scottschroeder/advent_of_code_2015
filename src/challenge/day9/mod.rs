use std::collections::HashMap;

use anyhow::{Context, Result};
use itertools::Itertools;
use upper_triangular::UpperTriangular;

/*
    L   D   B
L   0   464 518
D   464 0   141
B   518 141 0
*/

mod upper_triangular {
    /// Store only the upper triangular portion of the matrix
    ///
    /// Assume any query for the lower triangle is the transpose
    /// Assume anything along the center diagonal is the default value.
    #[derive(Debug)]
    pub struct UpperTriangular<T> {
        inner: Vec<T>,
        size: usize,
    }

    #[inline]
    fn sum_to_n(x: usize) -> usize {
        x * (x + 1) / 2
    }

    impl<T: Default + Clone> UpperTriangular<T> {
        pub fn new(size: usize) -> UpperTriangular<T> {
            let storage = sum_to_n(size - 1);
            UpperTriangular {
                inner: vec![T::default(); storage],
                size,
            }
        }

        pub fn size(&self) -> usize {
            self.size
        }

        #[inline]
        fn idx(&self, x: usize, y: usize) -> usize {
            if x == y {
                panic!("don't ask when x==y, these values do not appear");
            }
            let (x, y) = if x < y { (y, x) } else { (x, y) };

            let idx_2d = y * self.size + x;
            let skipped = sum_to_n(y + 1);
            let idx = idx_2d - skipped;
            log::trace!(
                "X:{} Y:{} idx:{} skipped:{} => {}",
                x,
                y,
                idx_2d,
                skipped,
                idx
            );

            idx
        }

        pub fn get(&self, x: usize, y: usize) -> T {
            if x == y {
                return T::default();
            }
            let idx = self.idx(x, y);
            self.inner[idx].clone()
        }

        pub fn set(&mut self, x: usize, y: usize, value: T) {
            let idx = self.idx(x, y);
            self.inner[idx] = value
        }
    }
}

fn parse_line(input: &str) -> Result<(&str, &str, u32)> {
    let mut segments = input.split('=');
    let src_dst = segments
        .next()
        .ok_or_else(|| anyhow::anyhow!("line could not be split on '='"))?
        .trim();
    let distance_str = segments
        .next()
        .ok_or_else(|| anyhow::anyhow!("line could not be split on '='"))?
        .trim();

    let distance = distance_str
        .parse::<u32>()
        .map_err(|e| anyhow::anyhow!("could not parse distance: {}", e))?;

    let mut src_dst_segments = src_dst.split("to");
    let src = src_dst_segments
        .next()
        .ok_or_else(|| anyhow::anyhow!("could not split directon on 'to'"))?
        .trim();
    let dst = src_dst_segments
        .next()
        .ok_or_else(|| anyhow::anyhow!("could not split directon on 'to'"))?
        .trim();
    Ok((src, dst, distance))
}

fn parse_input(input: &str) -> Result<UpperTriangular<u32>> {
    let mut loc_map = HashMap::new();
    let mut loc_to_id = |loc: &str| {
        let seen = loc_map.len();
        *loc_map.entry(loc.to_string()).or_insert(seen)
    };

    let mut buf = Vec::new();

    for line in input.lines() {
        let (src, dst, dist) =
            parse_line(line).with_context(|| format!("unable to parse line: {:?}", line))?;
        log::trace!("src: {} dst: {} dist: {}", src, dst, dist);
        buf.push((loc_to_id(src), loc_to_id(dst), dist))
    }
    let mut ut = UpperTriangular::new(loc_map.len());
    log::trace!("{:#?}", loc_map);
    for (x, y, v) in buf {
        ut.set(x, y, v);
    }
    Ok(ut)
}

fn permute_distances(ut: &UpperTriangular<u32>) -> impl Iterator<Item = u32> + '_ {
    (0..ut.size()).permutations(ut.size()).map(move |p| {
        p.iter()
            .tuple_windows()
            .map(|(s, e)| ut.get(*s, *e))
            .sum::<u32>()
    })
}

pub fn part1(input: &str) -> Result<String> {
    let ut = parse_input(input).context("unable to parse input")?;

    let path = permute_distances(&ut)
        .min()
        .ok_or_else(|| anyhow::anyhow!("there were no possible paths in input"))?;

    Ok(format!("{:?}", path))
}

pub fn part2(input: &str) -> Result<String> {
    let ut = parse_input(input).context("unable to parse input")?;

    let path = permute_distances(&ut)
        .max()
        .ok_or_else(|| anyhow::anyhow!("there were no possible paths in input"))?;

    Ok(format!("{:?}", path))
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day9");

    #[test]
    fn verify_p1() {
        assert_eq!(part1(INPUT).unwrap().as_str(), "251")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(part2(INPUT).unwrap().as_str(), "898")
    }

    #[test]
    fn parse_dist_line() {
        assert_eq!(
            parse_line("London to Dublin = 464").unwrap(),
            ("London", "Dublin", 464)
        );
    }
}
