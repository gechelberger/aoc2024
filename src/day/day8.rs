use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::grid::GridIdx;

pub struct Puzzle {
    cells: HashMap<char, Vec<GridIdx>>,
    rows: usize,
    cols: usize,
}

impl Puzzle {
    fn project(&self, a: GridIdx, b: GridIdx) -> Option<GridIdx> {
        let offset = a - b;
        let projected = a + offset;
        let rows = 0..self.rows as isize;
        let cols = 0..self.cols as isize;
        if rows.contains(&projected.0) && cols.contains(&projected.1) {
            Some(projected)
        } else {
            None
        }
    }

    pub fn part1(&self) -> usize {
        let mut reflections: HashSet<Option<GridIdx>> = HashSet::new();
        reflections.insert(None);

        for (_, nodes) in self.cells.iter() {
            for (a, b) in nodes.iter().tuple_combinations() {
                reflections.insert(self.project(*a, *b));
                reflections.insert(self.project(*b, *a));
            }
        }

        reflections.len() - 1
    }

    pub fn part2(&self) -> usize {
        let mut reflections: HashSet<Option<GridIdx>> = HashSet::new();
        reflections.insert(None);

        for (_, nodes) in self.cells.iter() {
            for (a, b) in nodes.iter().tuple_combinations() {
                self.accum_towards(&mut reflections, *a, *b);
                self.accum_towards(&mut reflections, *b, *a);
            }
        }

        reflections.len() - 1
    }

    pub fn accum_towards(&self, set: &mut HashSet<Option<GridIdx>>, a: GridIdx, mut b: GridIdx) {
        let rows = 0..self.rows as isize;
        let cols = 0..self.cols as isize;

        let step = b - a;
        while rows.contains(&b.0) && cols.contains(&b.1) {
            set.insert(Some(b));
            b = b + step;
        }
    }
}

mod input {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../puzzles/day8_test.txt");
    const INPUT: &str = include_str!("../../puzzles/day8.txt");

    impl Puzzle {
        pub fn new_test() -> Self {
            Self::parse(TEST_INPUT)
        }

        pub fn new() -> Self {
            Self::parse(INPUT)
        }

        pub fn parse(input: &str) -> Self {
            let mut cells: HashMap<char, Vec<GridIdx>> = HashMap::new();
            let mut lines = input.lines().filter(|x| !x.is_empty()).peekable();
            let cols = lines.peek().unwrap().len();
            let mut rows = 0;
            for (row, line) in lines.enumerate() {
                for (col, token) in line.chars().enumerate() {
                    if token == '.' {
                        continue;
                    }

                    let idx = GridIdx::new(row, col);
                    cells.entry(token).or_insert(Vec::new()).push(idx);
                }
                rows += 1;
            }

            Self { cells, rows, cols }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let pz = Puzzle::new_test();
        assert_eq!(pz.part1(), 14);

        let pz = Puzzle::new();
        assert_eq!(pz.part1(), 305);
    }

    #[test]
    fn test_part2() {
        let pz = Puzzle::new_test();
        assert_eq!(pz.part2(), 34);

        let pz = Puzzle::new();
        assert_eq!(pz.part2(), 1150);
    }
}
