use itertools::Itertools;

#[derive(Debug, Clone, PartialEq)]
pub struct Report(Vec<i32>);

impl Report {
    pub fn is_safe(&self) -> bool {
        const X: std::ops::Range<i32> = 1..4;
        self.0.is_sorted_by(|a, b| X.contains(&(a - b)))
            || self.0.is_sorted_by(|a, b| X.contains(&(b - a)))
    }

    pub fn is_safe_pt2(&self) -> bool {
        (0..self.0.len()).any(|idx| self.clone().without(idx).is_safe())
    }

    pub fn without(mut self, index: usize) -> Self {
        self.0.remove(index);
        self
    }
}

#[derive(Debug, PartialEq)]
struct Puzzle(Vec<Report>);

impl Puzzle {
    pub fn count_safe(&self) -> usize {
        self.0.iter().filter(|x| x.is_safe()).count()
    }

    pub fn count_safe_pt2(&self) -> usize {
        self.0.iter().filter(|x| x.is_safe_pt2()).count()
    }
}

mod input {
    use nom::IResult;
    use nom::character::complete::{digit1, multispace0, space1};
    use nom::combinator::{map, map_res};
    use nom::multi::{many0, separated_list1};
    use nom::sequence::terminated;

    use super::*;

    const PUZZLE_TEST: &'static str = include_str!("../../puzzles/day2_test.txt");
    const PUZZLE: &'static str = include_str!("../../puzzles/day2.txt");

    impl Report {
        pub fn parse(input: &str) -> IResult<&str, Self> {
            map(
                separated_list1(space1, map_res(digit1, str::parse::<i32>)),
                Self,
            )(input)
        }
    }

    impl Puzzle {
        pub fn new_test() -> Self {
            Self::parse(PUZZLE_TEST).unwrap().1
        }

        pub fn new() -> Self {
            Self::parse(PUZZLE).unwrap().1
        }

        pub fn parse(input: &str) -> IResult<&str, Self> {
            map(many0(terminated(Report::parse, multispace0)), Self)(input)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_report() {
        assert_eq!(
            Report::parse("1 2 3 4 5").unwrap().1,
            Report(vec![1, 2, 3, 4, 5])
        )
    }

    #[test]
    fn test_parse_input() {
        let test_input = Puzzle::new_test();
        assert_eq!(test_input.0.len(), 6);
    }

    #[test]
    fn test_part1() {
        assert_eq!(Puzzle::new_test().count_safe(), 2);
        assert_eq!(Puzzle::new().count_safe(), 407);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Puzzle::new_test().count_safe_pt2(), 4);
        assert_eq!(Puzzle::new().count_safe_pt2(), 459);
    }
}
