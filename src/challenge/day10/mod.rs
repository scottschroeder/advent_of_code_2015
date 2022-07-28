use anyhow::Result;

mod look_and_say {
    pub fn look_and_say<I: Iterator<Item = u32>>(
        input: I,
        rounds: usize,
    ) -> impl Iterator<Item = u32> {
        enum InnerIter<I> {
            Base(I),
            Acc(Accumulator<Box<InnerIter<I>>>),
        }
        impl<X: Iterator<Item = u32>> Iterator for InnerIter<X> {
            type Item = u32;

            fn next(&mut self) -> Option<Self::Item> {
                match self {
                    InnerIter::Base(b) => b.next(),
                    InnerIter::Acc(a) => a.next(),
                }
            }
        }

        let mut iter = InnerIter::Base(input);
        for _ in 0..rounds {
            iter = InnerIter::Acc(Accumulator::new(Box::new(iter)));
        }
        iter
    }

    #[derive(Debug, Clone, Copy)]
    enum State {
        Empty,
        Counting(u32, usize),
        Swap(u32, u32),
        Flush(u32),
    }

    struct Accumulator<I> {
        iter: I,
        state: State,
    }

    impl<I: Iterator<Item = u32>> Accumulator<I> {
        fn new(iter: I) -> Accumulator<I> {
            Accumulator {
                iter,
                state: State::Empty,
            }
        }
    }

    impl<I: Iterator<Item = u32>> Iterator for Accumulator<I> {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            loop {
                match self.state {
                    State::Empty => match self.iter.next() {
                        Some(v) => self.state = State::Counting(v, 1),
                        None => return None,
                    },
                    State::Counting(prev, count) => match self.iter.next() {
                        Some(v) => {
                            if v == prev {
                                self.state = State::Counting(v, count + 1);
                            } else {
                                self.state = State::Swap(prev, v);
                                return Some(count as u32);
                            }
                        }
                        None => {
                            self.state = State::Flush(prev);
                            return Some(count as u32);
                        }
                    },
                    State::Swap(prev, next) => {
                        self.state = State::Counting(next, 1);
                        return Some(prev);
                    }
                    State::Flush(v) => {
                        self.state = State::Empty;
                        return Some(v);
                    }
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn check_in_out(input: &[u32], expected: &[u32]) {
            let acc = Accumulator::new(input.iter().cloned());
            let out = acc.collect::<Vec<_>>();
            assert_eq!(&out, expected);
        }

        #[test]
        fn empty_iter() {
            check_in_out(&[], &[])
        }

        #[test]
        fn single_one() {
            check_in_out(&[1], &[1, 1])
        }
        #[test]
        fn double_one() {
            check_in_out(&[1, 1], &[2, 1])
        }
        #[test]
        fn two_one() {
            check_in_out(&[2, 1], &[1, 2, 1, 1])
        }
        #[test]
        fn one_two_one_one() {
            check_in_out(&[1, 2, 1, 1], &[1, 1, 1, 2, 2, 1])
        }
        #[test]
        fn acc_double() {
            let acc = look_and_say([1].iter().cloned(), 2);
            let out = acc.collect::<Vec<_>>();
            assert_eq!(&out, &[2, 1]);
        }
        #[test]
        fn acc_five() {
            let acc = look_and_say([1].iter().cloned(), 5);
            let out = acc.collect::<Vec<_>>();
            assert_eq!(&out, &[3, 1, 2, 2, 1, 1]);
        }
    }
}

fn parse_input_as_ints(input: &str) -> Result<Vec<u32>> {
    input
        .trim()
        .chars()
        .map(|c| {
            c.to_digit(10)
                .ok_or_else(|| anyhow::anyhow!("char {:?} was not a base-10 digit", c))
        })
        .collect()
}

pub fn part1(input: &str) -> Result<String> {
    let digits = parse_input_as_ints(input)?;
    let x = look_and_say::look_and_say(digits.into_iter(), 40).count();
    Ok(format!("{:?}", x))
}

pub fn part2(input: &str) -> Result<String> {
    let digits = parse_input_as_ints(input)?;
    let x = look_and_say::look_and_say(digits.into_iter(), 50).count();
    Ok(format!("{:?}", x))
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day10");

    #[test]
    fn verify_p1() {
        assert_eq!(part1(INPUT).unwrap().as_str(), "252594")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(part2(INPUT).unwrap().as_str(), "3579328")
    }
}
