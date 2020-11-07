use anyhow::Result;
use aoc::Point;

const GRID_SIZE: usize = 1000;

type Grid<T> = aoc::grid::fixed_grid::FixedGrid<T>;

pub fn part1(input: &str) -> Result<String> {
    let mut grid = Grid::<Bulb>::from_dimm(GRID_SIZE, GRID_SIZE);
    for instr in input.lines().map(parse_line) {
        let instr = instr?;
        grid_apply(instr.p1, instr.p2, &mut grid, |b: &mut Bulb| {
            instr.operation.op(b)
        });
    }
    let x = grid.raw_iter().filter(|b| **b == Bulb::On).count();
    Ok(format!("{:?}", x))
}

pub fn part2(input: &str) -> Result<String> {
    let mut grid = Grid::<ValueBulb>::from_dimm(GRID_SIZE, GRID_SIZE);
    for instr in input.lines().map(parse_line) {
        let instr = instr?;
        grid_apply(instr.p1, instr.p2, &mut grid, |b: &mut ValueBulb| {
            instr.operation.value(b)
        });
    }
    let x: i64 = grid.raw_iter().map(|b| b.0).sum();
    Ok(format!("{:?}", x))
}

#[derive(Debug)]
enum Operation {
    On,
    Off,
    Toggle,
}
impl Operation {
    fn op(&self, b: &mut Bulb) {
        match self {
            Operation::On => b.turn_on(),
            Operation::Off => b.turn_off(),
            Operation::Toggle => b.toggle(),
        }
    }
    fn value(&self, b: &mut ValueBulb) {
        match self {
            Operation::On => b.turn_on(),
            Operation::Off => b.turn_off(),
            Operation::Toggle => b.toggle(),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    operation: Operation,
    p1: Point<i64>,
    p2: Point<i64>,
}

fn parse_line(s: &str) -> Result<Instruction> {
    let mut operation = None;
    let mut point = vec![];
    for word in s.split_ascii_whitespace() {
        match word {
            "turn" | "through" => {}
            "on" => operation = Some(Operation::On),
            "off" => operation = Some(Operation::Off),
            "toggle" => operation = Some(Operation::Toggle),
            _ => point.push(parse_point(word)?),
        }
    }
    if point.len() != 2 {
        anyhow::bail!("could not parse two points from: {:?}", s)
    }
    let operation =
        operation.ok_or_else(|| anyhow::anyhow!("could not parse operation from: {:?}", s))?;
    Ok(Instruction {
        operation,
        p1: point[0],
        p2: point[1],
    })
}
fn parse_point(s: &str) -> Result<Point<i64>> {
    let mut read = s.split(',');
    Ok(Point::new(
        read.next().unwrap().parse::<i64>()?,
        read.next().unwrap().parse::<i64>()?,
    ))
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Bulb {
    On,
    Off,
}

impl Bulb {
    fn turn_on(&mut self) {
        *self = Bulb::On
    }
    fn turn_off(&mut self) {
        *self = Bulb::Off
    }
    fn toggle(&mut self) {
        *self = match self {
            Bulb::On => Bulb::Off,
            Bulb::Off => Bulb::On,
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
struct ValueBulb(i64);

impl ValueBulb {
    fn turn_on(&mut self) {
        self.0 += 1
    }
    fn turn_off(&mut self) {
        self.0 -= 1;
        if self.0 < 0 {
            self.0 = 0;
        }
    }
    fn toggle(&mut self) {
        self.0 += 2
    }
}

impl Default for Bulb {
    fn default() -> Self {
        Bulb::Off
    }
}

fn grid_apply<T, F>(p1: Point<i64>, p2: Point<i64>, grid: &mut Grid<T>, f: F)
where
    F: Fn(&mut T),
{
    let x1 = std::cmp::min(p1.x, p2.x);
    let x2 = std::cmp::max(p1.x, p2.x);
    let stride = (x2 - x1) as usize + 1;

    let y1 = std::cmp::min(p1.y, p2.y);
    let y2 = std::cmp::max(p1.y, p2.y);

    for y in y1..y2 + 1 {
        let start = Point::new(x1, y);
        let bulbs = grid.get_mut_range(start, stride);
        for b in bulbs {
            f(b)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day6");

    #[test]
    fn verify_p1() {
        assert_eq!(part1(INPUT).unwrap().as_str(), "400410")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(part2(INPUT).unwrap().as_str(), "15343601")
    }
}
