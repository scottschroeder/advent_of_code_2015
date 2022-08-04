use anyhow::{anyhow, Context, Result};

const RACE_DURATION: u32 = 2503;

#[derive(Debug, Clone, Copy)]
struct Reindeer {
    speed: u32,
    fly: u32,
    rest: u32,
}

impl Reindeer {
    fn distance_during_time(&self, total_seconds: u32) -> u32 {
        let cycle = self.fly + self.rest;
        let interval = total_seconds % cycle;
        if interval >= self.fly {
            0
        } else {
            self.speed
        }
    }

    fn distance(&self, seconds: u32) -> u32 {
        let cycle = self.fly + self.rest;
        let full_cycles = seconds / cycle;
        let last_cycle = seconds % cycle;
        let seconds_flying = if last_cycle < self.fly {
            full_cycles * self.fly + last_cycle
        } else {
            (full_cycles + 1) * self.fly
        };
        seconds_flying * self.speed
    }
}

fn parse_single_reindeer(input: &str) -> Result<Reindeer> {
    let mut words = input.split(' ');
    let mut parse_number = |n: usize| -> Result<u32> {
        let s = words
            .nth(n)
            .ok_or_else(|| anyhow::anyhow!("not enough elements in input"))?;
        let x = s
            .parse::<u32>()
            .with_context(|| format!("convert `{}` to number", s))?;
        Ok(x)
    };

    let speed = parse_number(3).context("parse speed")?;
    let fly = parse_number(2).context("parse fly")?;
    let rest = parse_number(6).context("parse rest")?;

    Ok(Reindeer { speed, fly, rest })
}

fn parse_input(input: &str) -> Result<Vec<Reindeer>> {
    let mut res = Vec::new();
    for line in input.lines() {
        res.push(parse_single_reindeer(line)?);
    }
    Ok(res)
}

fn winning_distance_after_time(input: &str, time: u32) -> Result<Option<u32>> {
    let racers = parse_input(input)?;
    log::trace!("{:?}", racers);
    Ok(racers.iter().map(|r| r.distance(time)).max())
}

fn winning_points_after_time(input: &str, time: u32) -> Result<Option<u32>> {
    let racers = parse_input(input)?;
    log::trace!("{:?}", racers);

    let mut points = vec![0u32; racers.len()];
    let mut distances = vec![0u32; racers.len()];
    let mut max_distance = 0;

    for t in 0..time {
        if t < 100 {
            log::trace!("t={}, Distance: {:?} (max={})", t, distances, max_distance);
            log::trace!("t={}, Points:   {:?}", t, points);
        }
        for (r, dist) in racers.iter().zip(&mut distances) {
            *dist += r.distance_during_time(t);
            max_distance = std::cmp::max(max_distance, *dist);
        }
        for (p, dist) in points.iter_mut().zip(&distances) {
            if *dist >= max_distance {
                *p += 1
            }
        }
    }

    Ok(points.iter().max().cloned())
}

pub fn part1(input: &str) -> Result<String> {
    let distance = winning_distance_after_time(input, RACE_DURATION)?
        .ok_or_else(|| anyhow::anyhow!("there were no racers"))?;
    Ok(format!("{:?}", distance))
}

pub fn part2(input: &str) -> Result<String> {
    let points = winning_points_after_time(input, RACE_DURATION)?
        .ok_or_else(|| anyhow::anyhow!("there were no racers"))?;
    Ok(format!("{:?}", points))
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day14");
    const EX: &str = include_str!("../../../input/day14_ex");

    #[test]
    fn verify_p1() {
        assert_eq!(part1(INPUT).unwrap().as_str(), "2640")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(part2(INPUT).unwrap().as_str(), "1102")
    }

    #[test]
    fn check_p1_example() {
        assert_eq!(winning_distance_after_time(EX, 1000).unwrap(), Some(1120))
    }
    #[test]
    fn check_p2_example() {
        assert_eq!(winning_points_after_time(EX, 1000).unwrap(), Some(689))
    }
}
