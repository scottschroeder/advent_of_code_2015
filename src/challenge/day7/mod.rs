use anyhow::Result;
use std::collections::HashMap;


mod parser;
mod lang;

pub fn part1(input: &str) -> Result<String> {
    let x =parser::parse_input(input)?;
    let mut w = HashMap::new();
    signal_engine(&mut w, x);
    log::info!("{:#?}", w);
    Ok(format!("{:?}", w.get(&lang::Identifier("a")).unwrap()))
}

pub fn part2(input: &str) -> Result<String> {
    let x =parser::parse_input(input)?;
    let mut w = HashMap::new();
    signal_engine(&mut w, x.clone());
    let wire_a = *w.get(&lang::Identifier("a")).unwrap();
    w.clear();
    w.insert(lang::Identifier("b"), wire_a);
    signal_engine(&mut w, x);
    log::info!("{:#?}", w);
    let wire_a = *w.get(&lang::Identifier("a")).unwrap();
    Ok(format!("{:?}", wire_a))
}


fn signal_engine<'a>(complete: &mut HashMap<lang::Identifier<'a>, u16>,mut segments: Vec<lang::Segment<'a>>) {
    while !segments.is_empty() {
        segments.retain(|s| {
            if complete.contains_key(&s.output) {
                return false;
            }
            let r = s.circut.resolve(&complete);
            log::trace!("{:?} -> {:?}", s, r);
            if let Some(r) = r {
                complete.insert(s.output, r);
            }
            r.is_none()
        })
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day7");
    const EX_1: &str = include_str!("../../../input/day7_ex1");

    #[test]
    fn ex1() {
        assert_eq!(part1(EX_1).unwrap().as_str(), "65412")
    }

    #[test]
    fn verify_p1() {
        assert_eq!(part1(INPUT).unwrap().as_str(), "956")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(part2(INPUT).unwrap().as_str(), "40149")

    }
}
