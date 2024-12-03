

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Mul(i64, i64),
    Do,
    Dont
}

impl Instruction {

    pub fn exec(&self) -> i64 {
        match self {
            Self::Mul(lhs, rhs) => lhs * rhs,
            _ => 0
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
        println!("{:?}", self);
        let mut total = 0i64;
        let mut exec_enabled = true;
        for inst in &self.0 {
            match inst {
                Instruction::Do => exec_enabled = true,
                Instruction::Dont => exec_enabled = false,
                Instruction::Mul(_, _) => total = match exec_enabled {
                    true => total + inst.exec(),
                    false => total 
                }
            }
        }
        total
    }
}

// struct Executor<I> 
// where 
//     I: IntoIterator<Item=Instruction>
// {
//     enabled: bool,
//     instructions: I
// }



mod input {
    use super::*;

    use nom::{branch::alt, bytes::complete::tag, character::complete::digit1, combinator::{map, map_res}, sequence::{delimited, pair, preceded, separated_pair}, IResult};

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
                    },
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
                                map_res(digit1, str::parse::<i64>)
                            ),
                            tag(")")
                        )
                    ),
                    |(a, b)| Self::Mul(a, b)
                ),
                map(tag("do()"), |_| Self::Do),
                map(tag("don't()"), |_| Self::Dont)
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