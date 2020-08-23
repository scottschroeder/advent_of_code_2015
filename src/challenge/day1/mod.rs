use anyhow::Result;

pub fn part1(input: &str) -> Result<String> {
    Ok(format!("{:?}", 0))
}

pub fn part2(input: &str) -> Result<String> {
    Ok(format!("{:?}", 0))
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day1");

    #[test]
    fn verify_p1() {
        assert_eq!(part1(INPUT).unwrap().as_str(), "0")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(part2(INPUT).unwrap().as_str(), "0")
    }
}