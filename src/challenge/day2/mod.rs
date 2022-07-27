use std::str::FromStr;

use anyhow::{anyhow as ah, Context, Result};

pub fn part1(input: &str) -> Result<String> {
    let total = box_calc(input, |b| b.surface_area() + b.smallest_side())?;
    Ok(format!("{:?}", total))
}

pub fn part2(input: &str) -> Result<String> {
    let total = box_calc(input, |b| b.smallest_perimeter() + b.volume())?;
    Ok(format!("{:?}", total))
}

fn box_calc<F>(input: &str, f: F) -> Result<usize>
where
    F: Fn(&Box) -> usize,
{
    let boxes = parse_orders(input)?;
    let mut total = 0;
    for b in &boxes {
        let box_cost = f(b);
        total += box_cost;
        log::trace!("{:?} -> {} ({})", b, box_cost, total);
    }
    Ok(total)
}

#[derive(Debug)]
struct Box {
    length: usize,
    width: usize,
    height: usize,
}

impl Box {
    fn surface_area(&self) -> usize {
        2 * self.length * self.width + 2 * self.width * self.height + 2 * self.height * self.length
    }
    fn volume(&self) -> usize {
        self.length * self.width * self.height
    }
    fn smallest_side(&self) -> usize {
        let a = self.length * self.width;
        let b = self.width * self.height;
        let c = self.height * self.length;
        min3(a, b, c)
    }
    fn smallest_perimeter(&self) -> usize {
        let a = 2 * self.length + 2 * self.width;
        let b = 2 * self.width + 2 * self.height;
        let c = 2 * self.height + 2 * self.length;
        min3(a, b, c)
    }
}

fn min3(a: usize, b: usize, c: usize) -> usize {
    std::cmp::min(a, std::cmp::min(b, c))
}

fn parse_orders(s: &str) -> Result<Vec<Box>> {
    s.lines()
        .map(|order_line| {
            parse_box(order_line).with_context(|| format!("failed to parse: {}", order_line))
        })
        .collect::<Result<Vec<Box>>>()
}

fn parse_box(s: &str) -> Result<Box> {
    let mut sp = s.split('x');
    Ok(Box {
        length: parse_dimm(sp.next())?,
        width: parse_dimm(sp.next())?,
        height: parse_dimm(sp.next())?,
    })
}
fn parse_dimm(chunk: Option<&str>) -> Result<usize> {
    let s = chunk.ok_or_else(|| ah!("empty dimmension"))?;
    usize::from_str(s).map_err(|e| ah!("{}", e))
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day2");

    #[test]
    fn verify_p1() {
        assert_eq!(part1(INPUT).unwrap().as_str(), "1588178")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(part2(INPUT).unwrap().as_str(), "3783758")
    }
}
