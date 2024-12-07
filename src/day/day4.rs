use itertools::Itertools;

use crate::grid::*;

const TEST_INPUT: &'static str = include_str!("../../puzzles/day4_test.txt");
const INPUT: &'static str = include_str!("../../puzzles/day4.txt");

pub struct Puzzle(Grid<char>);

impl Puzzle {
    pub fn new() -> Self {
        Self(parse_char_grid(INPUT))
    }

    pub fn new_test() -> Self {
        Self(parse_char_grid(TEST_INPUT))
    }

    pub fn part2(&self) -> usize {
        const MATCH_START: char = 'A';
        const CASE1: [Option<char>; 2] = [Some('M'), Some('S')];
        const CASE2: [Option<char>; 2] = [Some('S'), Some('M')];

        self.0
            .cells
            .iter()
            .positions(|c| c == &MATCH_START)
            .map(|idx| self.0.grid_idx(idx).unwrap())
            .filter(|rc| {
                const D1: GridOffset = GridOffset(1, 1);
                let diag1 = [self.0.get(*rc - D1).copied(), self.0.get(*rc + D1).copied()];

                const D2: GridOffset = GridOffset(-1, 1);
                let diag2 = [self.0.get(*rc - D2).copied(), self.0.get(*rc + D2).copied()];

                (diag1 == CASE1 || diag1 == CASE2) && (diag2 == CASE1 || diag2 == CASE2)
            })
            .count()
    }

    pub fn find_all(&self, needle: &str) -> impl Iterator<Item = isize> {
        let match_start = needle.chars().next().unwrap();

        self.0
            .cells
            .iter()
            .positions(move |c| c == &match_start)
            .map(|idx| self.0.grid_idx(idx).unwrap())
            .map(|rc| {
                self.counting_search(needle, rc, GridOffset(-1, -1))
                    + self.counting_search(needle, rc, GridOffset(-1, 0))
                    + self.counting_search(needle, rc, GridOffset(-1, 1))
                    + self.counting_search(needle, rc, GridOffset(0, -1))
                    + self.counting_search(needle, rc, GridOffset(0, 1))
                    + self.counting_search(needle, rc, GridOffset(1, -1))
                    + self.counting_search(needle, rc, GridOffset(1, 0))
                    + self.counting_search(needle, rc, GridOffset(1, 1))
            })
    }

    fn counting_search(&self, needle: &str, rc: GridIdx, dir: GridOffset) -> isize {
        if needle.is_empty() {
            return 1; // needle exhausted/found
        }
        let (match_char, needle) = split_first_char(needle);

        if match_char == self.0.get(rc).copied() {
            self.counting_search(needle, rc + dir, dir)
        } else {
            0
        }
    }

    pub fn part1(&self) -> isize {
        self.find_all("XMAS").sum()
    }
}

fn split_first_char(needle: &str) -> (Option<char>, &str) {
    let (head, tail) = needle.split_at(1);
    (head.chars().next(), tail)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let pz = Puzzle::new_test();
        assert_eq!(pz.part1(), 18);

        let pz = Puzzle::new();
        assert_eq!(pz.part1(), 2532);
    }

    #[test]
    fn test_part2() {
        let pz = Puzzle::new_test();
        assert_eq!(pz.part2(), 9);

        let pz = Puzzle::new();
        assert_eq!(pz.part2(), 1941);
    }
}
