use itertools::Itertools;

#[derive(Debug, Clone, PartialEq)]
pub struct Equation {
    test_value: u64,
    operands: Vec<u64>,
}

impl Equation {
    pub fn balances(&self, operators: &[BinOp]) -> bool {
        let (head, tail) = match self.operands.as_slice().split_first() {
            Some(res) => res,
            None => return false,
        };

        let init = (*head, tail);
        let mut stack = Vec::from([init]);
        while let Some((lhs, tail)) = stack.pop() {
            if tail.is_empty() {
                if lhs == self.test_value {
                    return true;
                } else {
                    continue;
                }
            }

            if lhs > self.test_value {
                continue;
            }

            let (rhs, tail) = tail.split_first().unwrap();
            for op in operators {
                stack.push((op.eval(lhs, *rhs), tail));
            }
        }

        false
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum BinOp {
    Add,
    Mul,
    Concat,
}

impl BinOp {
    pub fn eval(self, lhs: u64, rhs: u64) -> u64 {
        match self {
            Self::Add => lhs + rhs,
            Self::Mul => lhs * rhs,
            // Self::Concat => Self::string_concat(lhs, rhs),
            //Self::Concat => Self::recursive_concat(lhs, rhs),
            Self::Concat => Self::accum_concat(lhs, rhs),
        }
    }

    fn string_concat(lhs: u64, rhs: u64) -> u64 {
        let mut concat = lhs.to_string();
        concat.push_str(rhs.to_string().as_str());
        str::parse::<u64>(concat.as_str()).unwrap()
    }

    fn recursive_concat(mut lhs: u64, rhs: u64) -> u64 {
        if rhs > 0 {
            lhs = Self::recursive_concat(lhs * 10, rhs / 10);
        }
        return lhs - rhs / 10 + rhs;
    }

    fn accum_concat(mut lhs: u64, rhs: u64) -> u64 {
        let mut shift = rhs;
        while shift > 0 {
            shift = shift / 10;
            lhs = lhs * 10;
        }
        lhs + rhs
    }
}

pub struct Puzzle(Vec<Equation>);

impl Puzzle {
    pub fn part1(&self) -> u64 {
        const OPS: [BinOp; 2] = [BinOp::Add, BinOp::Mul];
        self.0
            .iter()
            .filter(|eq| eq.balances(&OPS))
            .map(|eq| eq.test_value)
            .sum()
    }

    pub fn part2(&self) -> u64 {
        const OPS: [BinOp; 3] = [BinOp::Add, BinOp::Mul, BinOp::Concat];
        self.0
            .iter()
            .filter(|eq| eq.balances(&OPS))
            .map(|eq| eq.test_value)
            .sum()
    }
}

mod input {
    use nom::IResult;
    use nom::bytes::complete::tag;
    use nom::character::complete::{multispace0, space1};
    use nom::combinator::map;
    use nom::multi::{many1, separated_list1};
    use nom::sequence::{separated_pair, terminated};

    use super::*;

    const TEST_INPUT: &'static str = include_str!("../../puzzles/day7_test.txt");
    const INPUT: &'static str = include_str!("../../puzzles/day7.txt");

    impl Equation {
        pub fn parse(input: &str) -> IResult<&str, Self> {
            let (input, test_value) = crate::parse::parse_u64(input)?;
            let (input, _) = tag(":")(input)?;
            let (input, operands) = separated_list1(space1, crate::parse::parse_u64)(input.trim())?;
            Ok((input, Self {
                test_value,
                operands,
            }))
        }
    }

    impl Puzzle {
        pub fn new_test() -> Self {
            Self::parse(TEST_INPUT).unwrap().1
        }

        pub fn new() -> Self {
            Self::parse(INPUT).unwrap().1
        }

        pub fn parse(input: &str) -> IResult<&str, Self> {
            map(many1(terminated(Equation::parse, multispace0)), Self)(input)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_equation() {
        let eq = "190: 10 19";
        let expected = Equation {
            test_value: 190,
            operands: vec![10, 19],
        };
        assert_eq!(expected, Equation::parse(eq).unwrap().1);
    }

    #[test]
    fn test_parse_puzzle() {
        let pz = Puzzle::new_test();
        assert_eq!(pz.0.len(), 9);
    }

    #[test]
    fn test_balances() {
        let eq = Equation {
            test_value: 190,
            operands: vec![10, 19],
        };
        assert_eq!(eq.balances(&[BinOp::Add, BinOp::Mul]), true);

        let eq = Equation {
            test_value: 3267,
            operands: vec![81, 40, 27],
        };
        assert_eq!(eq.balances(&[BinOp::Add, BinOp::Mul]), true);

        let eq = Equation {
            test_value: 83,
            operands: vec![17, 5],
        };
        assert_eq!(eq.balances(&[BinOp::Add, BinOp::Mul]), false);
    }

    #[test]
    fn test_part1() {
        let pz = Puzzle::new_test();
        assert_eq!(pz.part1(), 3749);

        let pz = Puzzle::new();
        assert_eq!(pz.part1(), 7710205485870);
    }

    #[test]
    fn test_part2() {
        let pz = Puzzle::new_test();
        assert_eq!(pz.part2(), 11387);

        let pz = Puzzle::new();
        assert_eq!(pz.part2(), 20928985450275);
    }
}
