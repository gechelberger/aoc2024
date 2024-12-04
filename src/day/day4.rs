use std::collections::HashSet;

use itertools::Itertools;

struct Grid {
    chars: Vec<char>,
    width: isize,
    height: isize,
}

const TEST_INPUT: &'static str = include_str!("../../puzzles/day4_test.txt");
const INPUT: &'static str = include_str!("../../puzzles/day4.txt");

type RC = [isize; 2]; // [Row, Col]

impl Grid {
    pub fn new() -> Self {
        Self::parse(INPUT)
    }

    pub fn new_test() -> Self {
        Self::parse(TEST_INPUT)
    }

    pub fn parse(input: &str) -> Self {
        let mut lines = input.lines().map(str::trim);
        let mut chars: Vec<_> = lines.next().unwrap().chars().collect();
        let width = chars.len() as isize;
        for line in lines {
            chars.extend(line.chars());
        }

        let height = chars.len() as isize / width;
        Self {
            chars,
            width,
            height,
        }
    }

    pub fn get(&self, rc: RC) -> Option<char> {
        let rows = 0..self.height;
        let cols = 0..self.width;

        if rows.contains(&rc[0]) && cols.contains(&rc[1]) {
            self.chars.get(self.rc_to_idx(rc) as usize).copied()
        } else {
            None
        }
    }

    fn rc_to_idx(&self, rc: RC) -> isize {
        let index: isize = rc[0] * self.width as isize + rc[1];
        index as isize
    }

    fn idx_to_rc(&self, index: isize) -> RC {
        [index / self.width, index % self.width]
    }

    pub fn part2(&self) -> usize {
        let match_start = 'A';
        let diag_goal = HashSet::<Option<char>>::from([Some('M'), Some('A'), Some('S')]);
        let init = HashSet::<Option<char>>::from([Some('A')]);

        let start_sites: Vec<_> = self
            .chars
            .iter()
            .positions(|c| c == &match_start)
            .map(|idx| self.idx_to_rc(idx as isize))
            .collect();

        start_sites
            .into_iter()
            .filter(|rc| {
                let mut diag1 = init.clone();
                diag1.insert(self.get([rc[0] - 1, rc[1] - 1]));
                diag1.insert(self.get([rc[0] + 1, rc[1] + 1]));

                let mut diag2 = init.clone();
                diag2.insert(self.get([rc[0] - 1, rc[1] + 1]));
                diag2.insert(self.get([rc[0] + 1, rc[1] - 1]));

                diag1 == diag_goal && diag2 == diag_goal
            })
            .count()
    }

    pub fn find_all(&self, needle: &str) -> impl Iterator<Item = isize> {
        let match_start = needle.chars().next().unwrap();

        let start_sites: Vec<_> = self
            .chars
            .iter()
            .positions(|c| c == &match_start)
            .map(|idx| self.idx_to_rc(idx as isize))
            .collect();

        start_sites.into_iter().map(|rc| {
            let count = self.counting_search(needle, rc, [-1, -1])
                + self.counting_search(needle, rc, [-1, 0])
                + self.counting_search(needle, rc, [-1, 1])
                + self.counting_search(needle, rc, [0, -1])
                + self.counting_search(needle, rc, [0, 1])
                + self.counting_search(needle, rc, [1, -1])
                + self.counting_search(needle, rc, [1, 0])
                + self.counting_search(needle, rc, [1, 1]);
            count
        })
    }

    fn counting_search(&self, needle: &str, rc: RC, dir: RC) -> isize {
        if needle.is_empty() {
            return 1; // needle exhausted/found
        }
        let (match_char, needle) = split_first_char(needle);

        if match_char == self.get(rc) {
            let rc = [rc[0] + dir[0], rc[1] + dir[1]];
            self.counting_search(needle, rc, dir)
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
        let pz = Grid::new_test();
        assert_eq!(pz.part1(), 18);

        let pz = Grid::new();
        assert_eq!(pz.part1(), 2532);
    }

    #[test]
    fn test_part2() {
        let pz = Grid::new_test();
        assert_eq!(pz.part2(), 9);

        let pz = Grid::new();
        assert_eq!(pz.part2(), 1941);
    }
}
