#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Mul(i64, i64),
    Do,
    Dont,
}

impl Instruction {
    pub fn exec(&self) -> i64 {
        match self {
            Self::Mul(lhs, rhs) => lhs * rhs,
            _ => 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Puzzle(Vec<Instruction>);

impl Puzzle {
    fn solve_pt1(&self) -> i64 {
        self.0.iter().map(Instruction::exec).sum()
    }

    fn solve_pt2(&self) -> i64 {
        
        struct State(bool, i64);

        impl State {
            fn set_enabled(mut self, enabled: bool) -> Self {
                self.0 = enabled;
                self

            }

            fn incr(mut self, amount: i64) -> Self {
                if self.0 {
                    self.1 += amount;
                }
                self
            }
        }

        let mut state = State(true, 0);
        for inst in &self.0 {
            state = match inst {
                Instruction::Do => state.set_enabled(true),
                Instruction::Dont => state.set_enabled(false),
                Instruction::Mul(lhs, rhs) => state.incr(lhs * rhs)
            }
        }
        state.1
    }
}

mod input {
    use nom::IResult;
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::digit1;
    use nom::combinator::{map, map_res};
    use nom::sequence::{delimited, preceded, separated_pair};

    use super::*;

    const INPUT: &'static str = include_str!("../../puzzles/day3.txt");
    const TEST_INPUT: &'static str = include_str!("../../puzzles/day3_test.txt");
    const TEST_INPUT2: &'static str = include_str!("../../puzzles/day3_test_pt2.txt");

    impl Puzzle {
        pub fn new() -> Self {
            Self::parse(INPUT)
        }

        pub fn new_test() -> Self {
            Self::parse(TEST_INPUT)
        }

        pub fn new_test_pt2() -> Self {
            Self::parse(TEST_INPUT2)
        }

        pub fn parse(mut input: &str) -> Self {
            let mut instructions = vec![];
            while !input.is_empty() {
                input = match Instruction::parse(input) {
                    Ok((tail, inst)) => {
                        instructions.push(inst);
                        tail
                    }
                    Err(_) => &input[1..],
                }
            }
            Self(instructions)
        }
    }

    impl Instruction {
        pub fn parse(input: &str) -> IResult<&str, Self> {
            alt((
                map(
                    preceded(
                        tag("mul"),
                        delimited(
                            tag("("),
                            separated_pair(
                                map_res(digit1, str::parse::<i64>),
                                tag(","),
                                map_res(digit1, str::parse::<i64>),
                            ),
                            tag(")"),
                        ),
                    ),
                    |(a, b)| Self::Mul(a, b),
                ),
                map(tag("do()"), |_| Self::Do),
                map(tag("don't()"), |_| Self::Dont),
            ))(input)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mul() {
        let x = "mul(123,234)";
        let inst = Instruction::parse(x).unwrap().1;
        assert_eq!(inst, Instruction::Mul(123, 234));
    }

    #[test]
    fn test_parse_test_input() {
        let pz = Puzzle::new_test();
        assert_eq!(pz.0.len(), 4);
    }

    #[test]
    fn test_part1() {
        let pz = Puzzle::new_test();
        assert_eq!(pz.solve_pt1(), 161);

        let pz = Puzzle::new();
        assert_eq!(pz.solve_pt1(), 196826776);
    }

    #[test]
    fn test_part2() {
        let pz = Puzzle::new_test_pt2();
        assert_eq!(pz.solve_pt2(), 48);

        let pz = Puzzle::new();
        assert_eq!(pz.solve_pt2(), 106780429);
    }
}
