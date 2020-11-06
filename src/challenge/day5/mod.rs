use anyhow::Result;
use std::collections::HashMap;

pub fn part1(input: &str) -> Result<String> {
    let x = input.lines().filter(|l| check_str(l)).count();
    Ok(format!("{:?}", x))
}

pub fn part2(input: &str) -> Result<String> {
    let x = input.lines().filter(|l| check_str_2(l)).count();
    Ok(format!("{:?}", x))
}

#[inline]
fn is_vowel(c: char) -> bool {
    match c {
        'a' => true,
        'e' => true,
        'i' => true,
        'o' => true,
        'u' => true,
        _ => false,
    }
}

#[inline]
fn is_double(c1: char, c2: char) -> bool {
    c1 == c2
}

#[inline]
fn is_banned(c1: char, c2: char) -> bool {
    match (c1, c2) {
        ('a', 'b') => true,
        ('c', 'd') => true,
        ('p', 'q') => true,
        ('x', 'y') => true,
        _ => false,
    }
}

fn check_str(s: &str) -> bool {
    let mut vowels = 0;
    let mut double = false;
    let mut prev = None;
    for c in s.chars() {
        if is_vowel(c) {
            vowels += 1;
        }
        if let Some(c1) = prev {
            if is_banned(c1, c) {
                log::trace!("{:?} ban: {}{}", s, c1, c);
                return false;
            }
            double = double || is_double(c1, c);
        }
        prev = Some(c);
    }
    log::trace!("{:?} double: {:?} vowels: {}", s, double, vowels);
    double && vowels >= 3
}

#[derive(Default, Debug)]
struct PairKeeper {
    inner: HashMap<(char, char), usize>,
}

impl PairKeeper {
    fn check(&mut self, pos: usize, c1: char, c2: char) -> bool {
        let key = (c1, c2);
        match self.inner.entry(key) {
            std::collections::hash_map::Entry::Occupied(o) => o.get() + 1 < pos,
            std::collections::hash_map::Entry::Vacant(v) => {
                v.insert(pos);
                false
            }
        }
    }
}

fn check_str_2(s: &str) -> bool {
    let mut n1 = None;
    let mut n2 = None;
    let mut pair = false;
    let mut skip = false;
    let mut pk = PairKeeper::default();
    for (pos, c) in s.chars().enumerate() {
        if let Some(c1) = n1 {
            pair = pair || pk.check(pos, c, c1);
            skip = skip || if let Some(c2) = n2 { c2 == c } else { false }
        }
        if pair && skip {
            return true;
        }
        n2 = n1.take();
        n1 = Some(c);
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day5");

    #[test]
    fn pair_exact() {
        let mut pk = PairKeeper::default();
        assert_eq!(pk.check(0, 'a', 'b'), false);
        assert_eq!(pk.check(1, 'b', 'a'), false);
        assert_eq!(pk.check(2, 'a', 'b'), true);
    }

    #[test]
    fn pair_triple() {
        let mut pk = PairKeeper::default();
        assert_eq!(pk.check(0, 'a', 'a'), false);
        assert_eq!(pk.check(1, 'a', 'a'), false);
        assert_eq!(pk.check(2, 'a', 'a'), true);
    }

    #[test]
    fn ex1() {
        assert_eq!(part1("ugknbfddgicrmopn").unwrap().as_str(), "1")
    }
    #[test]
    fn ex2() {
        assert_eq!(part1("aaa").unwrap().as_str(), "1")
    }
    #[test]
    fn ex3() {
        assert_eq!(part1("jchzalrnumimnmhp").unwrap().as_str(), "0")
    }
    #[test]
    fn ex4() {
        assert_eq!(part1("haegwjzuvuyypxyu").unwrap().as_str(), "0")
    }
    #[test]
    fn ex5() {
        assert_eq!(part1("dvszwmarrgswjxmb").unwrap().as_str(), "0")
    }

    #[test]
    fn p2_ex1() {
        assert_eq!(part2("qjhvhtzxzqqjkmpb").unwrap().as_str(), "1")
    }
    #[test]
    fn p2_ex2() {
        assert_eq!(part2("xxyxx").unwrap().as_str(), "1")
    }
    #[test]
    fn p2_ex3() {
        assert_eq!(part2("uurcxstgmygtbstg").unwrap().as_str(), "0")
    }
    #[test]
    fn p2_ex4() {
        assert_eq!(part2("ieodomkazucvgmuy").unwrap().as_str(), "0")
    }

    #[test]
    fn verify_p1() {
        assert_eq!(part1(INPUT).unwrap().as_str(), "258")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(part2(INPUT).unwrap().as_str(), "53")
    }
}
