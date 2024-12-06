use std::collections::VecDeque;
use std::ops::Not;

type PageOrder = (u32, u32);
type Booklet = Vec<u32>;

struct Puzzle {
    order_rules: Vec<PageOrder>,
    booklets: Vec<Booklet>,
}

impl Puzzle {
    fn booklet_rules(&self, booklet: &[u32]) -> impl Iterator<Item = &(u32, u32)> {
        self.order_rules
            .iter()
            .filter(|(a, b)| booklet.contains(a) && booklet.contains(b))
    }

    fn check_booklet(&self, booklet: &[u32]) -> bool {
        self.booklet_rules(booklet)
            .into_iter()
            .all(|(before, after)| {
                let index = booklet.iter().position(|x| x == before).unwrap();
                let (head, tail) = booklet.split_at(index);
                tail.contains(after)
            })
    }

    fn valid_booklets(&self) -> impl Iterator<Item = &[u32]> {
        self.booklets
            .iter()
            .map(|b| b.as_ref())
            .filter(|booklet| self.check_booklet(booklet))
    }

    fn invalid_booklets(&self) -> impl Iterator<Item = &[u32]> {
        self.booklets
            .iter()
            .map(|b| b.as_ref())
            .filter(|booklet| self.check_booklet(booklet).not())
    }

    fn partition_booklets(&self) -> (Vec<&[u32]>, Vec<&[u32]>) {
        self.booklets
            .iter()
            .map(|b| b.as_ref())
            .partition(|b| self.check_booklet(b))
    }

    pub fn part1(&self) -> u32 {
        self.valid_booklets().map(booklet_middle_page).sum()
    }

    pub fn reorder_booklet(&self, booklet: &[u32]) -> Vec<u32> {
        let rules: Vec<_> = self.booklet_rules(booklet).collect();
        let valid = |a: &u32, b: &u32| {
            let invalid = (*b, *a);
            if rules.contains(&&invalid) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Less
            }
        };

        let mut booklet = Vec::from(booklet);
        //booklet.sort_by(valid);
        booklet.sort_unstable_by(valid);
        booklet
    }

    pub fn part2(&self) -> u32 {
        self.invalid_booklets()
            .map(|b| self.reorder_booklet(b))
            .map(|b| booklet_middle_page(&b))
            .sum()
    }
}

fn booklet_middle_page(booklet: &[u32]) -> u32 {
    let index = booklet.len() / 2;
    booklet[index]
}

mod input {
    use nom::bytes::complete::tag;
    use nom::character::complete::{digit1, line_ending, multispace0};
    use nom::combinator::map_res;
    use nom::multi::{many1, separated_list1};
    use nom::sequence::{separated_pair, terminated};
    use nom::{Finish, IResult};

    use super::*;

    const INPUT: &'static str = include_str!("../../puzzles/day5.txt");
    const TEST_INPUT: &'static str = include_str!("../../puzzles/day5_test.txt");

    pub fn parse_u32(input: &str) -> IResult<&str, u32> {
        map_res(digit1, str::parse::<u32>)(input)
    }

    pub fn parse_page_order(input: &str) -> IResult<&str, PageOrder> {
        separated_pair(parse_u32, tag("|"), parse_u32)(input)
    }

    impl Puzzle {
        pub fn new_test() -> Self {
            Self::parse(TEST_INPUT).unwrap().1
        }

        pub fn new() -> Self {
            Self::parse(INPUT).unwrap().1
        }

        pub fn parse(input: &str) -> IResult<&str, Self> {
            let (input, order_rules) = many1(terminated(parse_page_order, line_ending))(input)?;
            let (input, booklets) = many1(terminated(
                separated_list1(tag(","), parse_u32),
                multispace0,
            ))(input.trim_start())?;

            Ok((input, Self {
                order_rules,
                booklets,
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_page_order() {
        let input = "47|53";
        assert_eq!(super::input::parse_page_order(input).unwrap().1, (47, 53));
    }

    #[test]
    fn test_parse_puzzle() {
        let pz = Puzzle::new_test();
        assert_eq!(pz.order_rules.len(), 21);
        assert_eq!(pz.booklets.len(), 6);
    }

    #[test]
    fn test_part_1() {
        let pz = Puzzle::new_test();
        assert_eq!(pz.part1(), 143);

        let pz = Puzzle::new();
        assert_eq!(pz.part1(), 6242);
    }

    #[test]
    fn test_reorder() {
        let pz = Puzzle::new_test();
        let b = [75, 97, 47, 61, 53];
        assert_eq!(pz.reorder_booklet(&b), vec![97, 75, 47, 61, 53]);

        let b = [61, 13, 29];
        assert_eq!(pz.reorder_booklet(&b), vec![61, 29, 13]);

        let b = [97, 13, 75, 29, 47];
        assert_eq!(pz.reorder_booklet(&b), vec![97, 75, 47, 29, 13]);
    }

    #[test]
    fn test_part_2() {
        let pz = Puzzle::new_test();
        assert_eq!(pz.part2(), 123);

        let pz = Puzzle::new();
        assert_eq!(pz.part2(), 5169);
    }
}
