use anyhow::{anyhow as ah, Result};

pub fn part1(input: &str) -> Result<String> {
    let mut floor = 0;
    Ok(format!("{:?}", floor))
}

pub fn part2(input: &str) -> Result<String> {
    let mut floor = 0;
    Ok(format!("{:?}", floor))
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day3");

    #[test]
    fn verify_p1() {
        assert_eq!(part1(INPUT).unwrap().as_str(), "0")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(part2(INPUT).unwrap().as_str(), "0")
    }

}