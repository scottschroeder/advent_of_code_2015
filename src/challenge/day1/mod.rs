use anyhow::{anyhow as ah, Result};

pub fn part1(input: &str) -> Result<String> {
    let mut floor = 0;
    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => return Err(ah!("unknown input char: {}", c)),
        }
    }
    Ok(format!("{:?}", floor))
}

pub fn part2(input: &str) -> Result<String> {
    Ok(format!("{:?}", first_basement(input)?))
}

fn first_basement(input: &str) -> Result<usize> {
    let mut floor = 0;
    for (idx, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => return Err(ah!("unknown input char: {}", c)),
        }
        if floor == -1 {
            return Ok(idx + 1);
        }
    }
    Err(ah!("never entered basement"))
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day1");

    #[test]
    fn verify_p1() {
        assert_eq!(part1(INPUT).unwrap().as_str(), "232")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(part2(INPUT).unwrap().as_str(), "1783")
    }

    #[test]
    fn p1_basics() {
        assert_eq!(part1("(())").unwrap().as_str(), "0");
        assert_eq!(part1("()()").unwrap().as_str(), "0");
        assert_eq!(part1("(((").unwrap().as_str(), "3");
        assert_eq!(part1("(()(()(").unwrap().as_str(), "3");
        assert_eq!(part1("))(((((").unwrap().as_str(), "3");
        assert_eq!(part1("())").unwrap().as_str(), "-1");
        assert_eq!(part1(")))").unwrap().as_str(), "-3");
        assert_eq!(part1(")())())").unwrap().as_str(), "-3");
    }
}
