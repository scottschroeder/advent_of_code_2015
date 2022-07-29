use anyhow::{anyhow, Result};

fn next_letter(c: char) -> char {
    // Igore hij klm nop
    match c {
        'h' => 'j',
        'n' => 'p',
        'k' => 'm',
        'z' => 'a',
        _ => {
            let cid = c as u8;
            (cid + 1) as char
        }
    }
}

fn invalid_letter(c: char) -> bool {
    match c {
        'i' => true,
        'o' => true,
        'l' => true,
        _ => false,
    }
}

fn check_straight(a: char, b: char, c: char) -> bool {
    let ax = a as u8;
    let bx = b as u8;
    let cx = c as u8;
    ax + 1 == bx && bx + 1 == cx
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Password([char; 8]);

impl Password {
    fn new(passwd: &str) -> Result<Self> {
        let mut pw = Password([' '; 8]);
        let mut ok = false;
        for (idx, c) in passwd.chars().take(8).enumerate() {
            if idx == 7 {
                ok = true;
            }
            pw.0[idx] = c
        }
        if !ok {
            Err(anyhow::anyhow!("input did not have 8 chars"))
        } else {
            Ok(pw)
        }
    }
    fn increment(&mut self) {
        for place in 0..8 {
            let idx = 7 - place;
            let next_c = next_letter(self.0[idx]);
            self.0[idx] = next_c;
            if next_c != 'a' {
                break;
            }
        }
    }

    fn count_doubles(&self) -> usize {
        let mut count = 0;

        let mut last_match = false;
        for pair in self.0.windows(2) {
            if last_match {
                last_match = false;
                continue;
            }
            if pair[0] == pair[1] {
                last_match = true;
                count += 1;
            }
        }

        count
    }

    fn find_straight(&self) -> bool {
        self.0.windows(3).any(|w| check_straight(w[0], w[1], w[2]))
    }
    fn has_invalid_letter(&self) -> bool {
        self.0.iter().any(|c| invalid_letter(*c))
    }
}

impl std::fmt::Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.0.iter() {
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

fn valid_password(pw: &Password) -> bool {
    pw.count_doubles() >= 2 && pw.find_straight() && !pw.has_invalid_letter()
}

fn next_valid_password(start: &Password) -> Password {
    let mut new = *start;
    log::info!("starting with: {}", start);
    new.increment();
    log::trace!("first round: {}", new);

    while !valid_password(&new) {
        new.increment();
        log::trace!("next round: {}", new);
    }

    log::info!("result: {}", new);
    new
}

pub fn part1(input: &str) -> Result<String> {
    let pw = Password::new(input)?;
    let next = next_valid_password(&pw);
    Ok(format!("{}", next))
}

pub fn part2(input: &str) -> Result<String> {
    let pw = Password::new(input)?;
    let next = next_valid_password(&pw);
    let next2 = next_valid_password(&next);
    Ok(format!("{}", next2))
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day11");

    #[test]
    fn verify_p1() {
        assert_eq!(part1(INPUT).unwrap().as_str(), "vzbxxyzz")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(part2(INPUT).unwrap().as_str(), "vzcaabcc")
    }

    #[test]
    fn create_password() {
        let pw = Password::new("abcdefgh").unwrap();
        assert_eq!(pw.0, ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'])
    }

    #[test]
    fn password_too_short() {
        assert!(Password::new("abcdefg").is_err());
    }

    #[test]
    fn increment_normal() {
        let mut pw = Password::new("aaaaaaaa").unwrap();
        pw.increment();
        assert_eq!(pw, Password::new("aaaaaaab").unwrap());
    }

    #[test]
    fn increment_rollover() {
        let mut pw = Password::new("aaaaaaaz").unwrap();
        pw.increment();
        assert_eq!(pw, Password::new("aaaaaaba").unwrap());
    }

    #[test]
    fn increment_rollover_stack() {
        let mut pw = Password::new("aaaaazzz").unwrap();
        pw.increment();
        assert_eq!(pw, Password::new("aaaabaaa").unwrap());
    }

    #[test]
    fn increment_rollover_all() {
        let mut pw = Password::new("zzzzzzzz").unwrap();
        pw.increment();
        assert_eq!(pw, Password::new("aaaaaaaa").unwrap());
    }

    #[test]
    fn increment_skip_letter() {
        let mut pw = Password::new("aaaahzzz").unwrap();
        pw.increment();
        assert_eq!(pw, Password::new("aaaajaaa").unwrap());
    }
    #[test]
    fn count_zero_doubles() {
        let pw = Password::new("abcdefgh").unwrap();
        assert_eq!(pw.count_doubles(), 0);
    }
    #[test]
    fn count_single_doubles() {
        let pw = Password::new("aacdefgh").unwrap();
        assert_eq!(pw.count_doubles(), 1);
    }
    #[test]
    fn count_last_doubles() {
        let pw = Password::new("abcdefgg").unwrap();
        assert_eq!(pw.count_doubles(), 1);
    }

    #[test]
    fn count_two_doubles() {
        let pw = Password::new("aacdefgg").unwrap();
        assert_eq!(pw.count_doubles(), 2);
    }

    #[test]
    fn dont_count_overlapping() {
        let pw = Password::new("aaadefgh").unwrap();
        assert_eq!(pw.count_doubles(), 1);
    }

    #[test]
    fn simple_straight() {
        let pw = Password::new("abcxxxxx").unwrap();
        assert_eq!(pw.find_straight(), true);
    }

    #[test]
    fn straight_with_invalid_letter() {
        let pw = Password::new("xxxklmxx").unwrap();
        assert_eq!(pw.find_straight(), true);
    }
    #[test]
    fn wrapping_straight() {
        let pw = Password::new("gggyzagg").unwrap();
        assert_eq!(pw.find_straight(), false);
    }

    #[test]
    fn check_ex1() {
        let pw = Password::new("hijklmmn").unwrap();
        assert!(!valid_password(&pw));
    }
    #[test]
    fn check_ex2() {
        let pw = Password::new("abbceffg").unwrap();
        assert!(!valid_password(&pw));
    }
    #[test]
    fn check_ex3() {
        let pw = Password::new("abbcegjk").unwrap();
        assert!(!valid_password(&pw));
    }

    #[test]
    fn check_ex4() {
        let pw = Password::new("abcdffaa").unwrap();
        assert!(valid_password(&pw));
    }

    #[test]
    fn check_ex5() {
        let pw = Password::new("ghjaabcc").unwrap();
        assert!(valid_password(&pw));
    }

    #[test]
    fn check_only_invalid_letter() {
        let pw = Password::new("hijaabcc").unwrap();
        assert!(!valid_password(&pw));
    }

    #[test]
    fn check_part1_ex0() {
        let pw = Password::new("abcdfezz").unwrap();
        let next = next_valid_password(&pw);
        assert_eq!(next, Password::new("abcdffaa").unwrap());
    }

    #[test]
    fn check_part1_ex1() {
        let pw = Password::new("abcdefgh").unwrap();
        let next = next_valid_password(&pw);
        assert_eq!(next, Password::new("abcdffaa").unwrap());
    }
    #[test]
    fn check_part1_ex2() {
        let pw = Password::new("ghijklmn").unwrap();
        let next = next_valid_password(&pw);
        assert_eq!(next, Password::new("ghjaabcc").unwrap());
    }
}
