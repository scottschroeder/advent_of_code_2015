use anyhow::Result;
use rayon::prelude::*;

const PART1_MAX: u64 = 2000000;
const PART2_MAX: u64 = 2000000;

pub fn part1(input: &str) -> Result<String> {
    let input = input.trim();
    let n = scan(input, PART1_MAX, is_five_zeros)?;
    Ok(format!("{:?}", n))
}

pub fn part2(input: &str) -> Result<String> {
    let input = input.trim();
    let n = scan(input, PART2_MAX, is_six_zeros)?;
    Ok(format!("{:?}", n))
}

fn scan<F>(secret: &str, max: u64, f: F) -> Result<u64>
where
    F: Fn(md5::Digest) -> bool + Sync,
{
    (0u64..max)
        .into_par_iter()
        .find_first(|n| {
            let d = hash(secret, *n);
            f(d)
        })
        .ok_or_else(|| anyhow::anyhow!("could not find value under {}", max))
}

#[inline]
fn hash(secret: &str, n: u64) -> md5::Digest {
    let mut ctx = md5::Context::new();
    ctx.consume(secret.as_bytes());
    let ascii_number = n.to_string();
    ctx.consume(ascii_number.as_bytes());
    ctx.compute()
}

#[inline]
fn is_five_zeros(digest: md5::Digest) -> bool {
    let data: [u8; 16] = digest.into();
    data[0] == 0 && data[1] == 0 && (data[2] & 0xF0) == 0
}

#[inline]
fn is_six_zeros(digest: md5::Digest) -> bool {
    let data: [u8; 16] = digest.into();
    data[0] == 0 && data[1] == 0 && data[2] == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day4");

    #[test]
    fn ex1() {
        assert_eq!(part1("abcdef").unwrap().as_str(), "609043")
    }
    #[test]
    fn ex2() {
        assert_eq!(part1("pqrstuv").unwrap().as_str(), "1048970")
    }

    #[test]
    fn verify_p1() {
        assert_eq!(part1(INPUT).unwrap().as_str(), "254575")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(part2(INPUT).unwrap().as_str(), "1038736")
    }
}
