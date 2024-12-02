use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

use itertools::Itertools;
use nom::IResult;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pair(u64, u64);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Puzzle(Vec<Pair>);

impl Puzzle {
    pub fn into_ord_pairs(self) -> impl Iterator<Item = (u64, u64)> {
        let mut h1 = BinaryHeap::<Reverse<u64>>::new();
        let mut h2 = BinaryHeap::<Reverse<u64>>::new();
        for pair in self.0 {
            h1.push(Reverse(pair.0));
            h2.push(Reverse(pair.1));
        }

        let it1 = std::iter::from_fn(move || h1.pop().map(|x| x.0));
        let it2 = std::iter::from_fn(move || h2.pop().map(|x| x.0));
        std::iter::zip(it1, it2)
    }

    pub fn part2(self) -> u64 {
        let (left, right): (Vec<_>, Vec<_>) = self.0.into_iter().map(|x| (x.0, x.1)).unzip();
        let haystack = right.into_iter().counts();

        left.into_iter()
            .map(|x| x * (*haystack.get(&x).unwrap_or(&0) as u64))
            .sum()
    }

    pub fn part1(self) -> u64 {
        self.into_ord_pairs().map(|(a, b)| a.abs_diff(b)).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ord_pairs() {
        let pz = Puzzle::new_test();
        let ords: Vec<_> = pz.into_ord_pairs().collect();
        assert_eq!(ords.first().unwrap(), &(1, 3));
        assert_eq!(ords.last().unwrap(), &(4, 9));
    }

    #[test]
    fn test_part1() {
        assert_eq!(11, Puzzle::new_test().part1());
        assert_eq!(2756096, Puzzle::new_puzzle().part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(31, Puzzle::new_test().part2());
        assert_eq!(23117829, Puzzle::new_puzzle().part2());
    }
}

pub mod input {
    use nom::character::complete::{digit1, multispace0, multispace1};
    use nom::combinator::{map, map_res};
    use nom::multi::many1;
    use nom::sequence::{separated_pair, terminated};

    use super::*;

    static PUZZLE_TEST: &'static str = include_str!("../../puzzles/day1_test.txt");
    static PUZZLE: &'static str = include_str!("../../puzzles/day1.txt");

    impl Pair {
        pub fn parse(input: &str) -> IResult<&str, Self> {
            map(
                separated_pair(
                    map_res(digit1, str::parse::<u64>),
                    multispace1,
                    map_res(digit1, str::parse::<u64>),
                ),
                |(a, b)| Self(a, b),
            )(input)
        }
    }

    impl Puzzle {
        pub fn new_test() -> Self {
            Self::parse(PUZZLE_TEST).unwrap().1
        }

        pub fn new_puzzle() -> Self {
            Self::parse(PUZZLE).unwrap().1
        }

        pub fn parse(input: &str) -> IResult<&str, Self> {
            map(many1(terminated(Pair::parse, multispace0)), Self)(input)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_parse_pair() {
            let input = "4    5\n";
            let (_, pair) = Pair::parse(input).unwrap();
            assert_eq!(pair, Pair(4, 5));
        }

        #[test]
        fn test_parse_puzzle() {
            let puzzle = Puzzle::new_test();
            assert_eq!(puzzle.0.len(), 6);
        }
    }
}
