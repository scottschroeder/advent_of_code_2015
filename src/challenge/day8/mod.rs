use anyhow::Result;


pub fn part1(input: &str) -> Result<String> {
    let x = input.lines().map(delta).sum::<usize>();
    Ok(format!("{:?}", x))
}

pub fn part2(input: &str) -> Result<String> {
    let x = input.lines().map(delta_encode).sum::<usize>();
    Ok(format!("{:?}", x))
}

fn delta(s: &str) -> usize {
    s.len() - count_quoted(s)
}

fn delta_encode(s: &str) -> usize {
    escape_len(s) - s.len()
}

fn escape_len(s: &str) -> usize {
    s.chars().map(|c| {
        match c {
            '"' => 2,
            '\\' => 2,
            _ => 1,
        }
    }).sum::<usize>() + 2
}

fn count_quoted(s: &str) -> usize {
    let mut escape = false;
    let mut hex = None;
    let mut count = 0;
    for (idx, c) in s.chars().enumerate() {
        if idx == 0 {
            assert_eq!(c, '"');
            continue;
        }
        let last_escape = escape;
        escape = false;
        if let Some(skip_more) = hex {
            if skip_more {
                hex = Some(false);
            } else {
                hex = None;
            }
            continue
        }
        match (last_escape, c) {
            (true, 'x') => {
                count += 1;
                hex = Some(true)
            }
            (true, '\\') => count +=1,
            (true, '"') => count +=1,
            (false, '"') => return count,
            (false, '\\') => escape = true,
            (true, _) => panic!("unknown escape char: {:?} in {:?}", c, s),
            (false, _) => count += 1,
        }
    }
    panic!("string did not end in quotation: {:?}", s)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day8");

    #[test]
    fn ex_1() {
        assert_eq!(count_quoted(r##""""##), 0);
    }
    #[test]
    fn ex_2() {
        assert_eq!(count_quoted(r##""abc""##), 3);
    }
    #[test]
    fn ex_3() {
        assert_eq!(count_quoted(r##""aaa\"aaa""##), 7);
    }
    #[test]
    fn ex_4() {
        assert_eq!(count_quoted(r##""\x27""##), 1);
    }

    #[test]
    fn pt2_ex_1() {
        assert_eq!(escape_len(r##""""##), 6);
    }
    #[test]
    fn pt2_ex_2() {
        assert_eq!(escape_len(r##""abc""##), 9);
    }
    #[test]
    fn pt2_ex_3() {
        assert_eq!(escape_len(r##""aaa\"aaa""##), 16);
    }
    #[test]
    fn pt2_ex_4() {
        assert_eq!(escape_len(r##""\x27""##), 11);
    }

    #[test]
    fn verify_p1() {
        assert_eq!(part1(INPUT).unwrap().as_str(), "1342")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(part2(INPUT).unwrap().as_str(), "2074")

    }
}
