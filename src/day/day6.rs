use std::collections::HashSet;

use super::grid::*;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Cell {
    Obstruction,
    Open,
    Start,
}

fn rotate_right(offset: GridOffset) -> GridOffset {
    let row = 0 * offset.0 + 1 * offset.1;
    let col = -1 * offset.0 + 0 * offset.1;
    GridOffset(row, col)
}

type Turn = (GridIdx, GridOffset);

#[derive(Debug, Clone)]
struct Puzzle {
    grid: Grid<Cell>,
    start: GridIdx,
}

impl Puzzle {
    pub fn part1(&self) -> usize {
        let direction = GridOffset(-1, 0); // UP
        let mut visited: HashSet<GridIdx> = HashSet::new();
        // iterator in a direction until:
        // - we walk out of bounds (iter returns None)
        //   - return the total unique cells visited
        // - we hit an obstacle
        //   - turn right and continue
        self.walk(self.start, direction, &mut visited)
    }

    pub fn walk(&self, start: GridIdx, dir: GridOffset, visited: &mut HashSet<GridIdx>) -> usize {
        let mut location = start;
        for (idx, cell) in self.grid.iter_from_with_stride(start, dir) {
            if cell == &Cell::Obstruction {
                return self.walk(location, rotate_right(dir), visited);
            } else {
                location = idx;
                visited.insert(location);
            }
        }
        visited.len()
    }

    pub fn has_cycles(&self, start: GridIdx, dir: GridOffset, path: &mut HashSet<Turn>) -> bool {
        let turn = (start, dir);
        if path.contains(&turn) {
            return true; // we've already hit this cell moving in this direction so must be in a cycle.
        }

        path.insert(turn);

        let mut location = start;
        for (idx, cell) in self.grid.iter_from_with_stride(start, dir) {
            if cell == &Cell::Obstruction {
                return self.has_cycles(location, rotate_right(dir), path);
            } else {
                location = idx;
            }
        }
        false
    }

    pub fn part2_brute_force(&self) -> usize {
        let dir = GridOffset(-1, 0);
        let mut count = 0;
        for i in 0..self.grid.cells.len() {
            if self.grid.cells[i] == Cell::Open {
                let mut case = self.clone();
                case.grid.cells[i] = Cell::Obstruction;
                let mut path = HashSet::new();
                if case.has_cycles(self.start, dir, &mut path) {
                    count += 1;
                }
            }
        }
        count
    }
}

mod input {
    use super::*;

    pub const TEST_INPUT: &'static str = include_str!("../../puzzles/day6_test.txt");
    pub const INPUT: &'static str = include_str!("../../puzzles/day6.txt");

    impl Puzzle {
        pub fn new_test() -> Self {
            Self::parse(TEST_INPUT)
        }

        pub fn new() -> Self {
            Self::parse(INPUT)
        }

        pub fn parse(input: &str) -> Self {
            let grid = parse_grid(input);
            let start = grid.position(|c| c == &Cell::Start).unwrap();
            Puzzle { grid, start }
        }
    }

    pub fn parse_grid(input: &str) -> Grid<Cell> {
        Grid::parse(input, |c| match c {
            '.' => Cell::Open,
            '#' => Cell::Obstruction,
            '^' => Cell::Start,
            e => panic!("bad input: {:?}", e),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let grid = input::parse_grid(input::TEST_INPUT);
        assert_eq!(grid.size(), (10, 10));
    }

    #[test]
    fn test_rotate() {
        let offset = GridOffset(-1, 0);
        let offset = rotate_right(offset);
        assert_eq!(offset, GridOffset(0, 1));
        let offset = rotate_right(offset);
        assert_eq!(offset, GridOffset(1, 0));
        let offset = rotate_right(offset);
        assert_eq!(offset, GridOffset(0, -1));
        let offset = rotate_right(offset);
        assert_eq!(offset, GridOffset(-1, 0));
    }

    #[test]
    fn test_part1() {
        let pz = Puzzle::new_test();
        assert_eq!(pz.part1(), 41);

        let pz = Puzzle::new();
        assert_eq!(pz.part1(), 5409);
    }

    #[ignore]
    #[test]
    fn test_part2_brute_force() {
        let pz = Puzzle::new_test();
        assert_eq!(pz.part2_brute_force(), 6);

        let pz = Puzzle::new();
        assert_eq!(pz.part2_brute_force(), 2022);
    }
}
