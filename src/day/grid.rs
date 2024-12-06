use core::ops::{Add, Mul, Sub};
use std::path::Iter;

pub enum Horz {
    Left,
    Right,
}
pub enum Vert {
    Up,
    Down,
}
pub enum Orthogonal {
    H(Horz),
    V(Vert),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GridAdj {
    UL,
    U,
    UR,
    L,
    R,
    DL,
    D,
    DR,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GridOffset(pub isize, pub isize);

impl GridOffset {
    fn row(self) -> isize {
        self.0
    }

    fn col(self) -> isize {
        self.1
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GridIdx(isize, isize);

impl GridIdx {
    fn row(self) -> usize {
        self.0 as usize
    }

    fn col(self) -> usize {
        self.1 as usize
    }

    fn row_maj_idx(self, cols: usize) -> usize {
        self.row() * cols + self.col()
    }
}

impl Add for GridOffset {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Add<GridOffset> for GridIdx {
    type Output = Self;

    fn add(self, rhs: GridOffset) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for GridIdx {
    type Output = GridOffset;

    fn sub(self, rhs: Self) -> Self::Output {
        GridOffset(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Sub<GridOffset> for GridIdx {
    type Output = Self;

    fn sub(self, rhs: GridOffset) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul<isize> for GridOffset {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Grid<T> {
    pub cells: Vec<T>,
    pub rows: usize,
    pub cols: usize,
}

impl<T> Grid<T> {
    pub fn parse_chars(input: &str) -> Grid<char> {
        Grid::<char>::parse(input, |x| x)
    }

    pub fn parse(input: &str, sym: impl Fn(char) -> T) -> Self {
        let mut lines = input.lines().map(str::trim).filter(|x| !x.is_empty());
        let mut cells: Vec<_> = lines.next().unwrap().chars().collect();
        let cols = cells.len();
        for line in lines {
            cells.extend(line.chars());
        }

        let cells: Vec<T> = cells.into_iter().map(sym).collect();
        let rows = cells.len() / cols;
        Self { cells, rows, cols }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn get(&self, index: GridIdx) -> Option<&T> {
        let rows = 0..self.rows as isize;
        let cols = 0..self.cols as isize;
        if rows.contains(&index.0) && cols.contains(&index.1) {
            let idx = index.row_maj_idx(self.cols);
            self.cells.get(idx)
        } else {
            None
        }
    }

    pub fn position(&self, pred: impl Fn(&T) -> bool) -> Option<GridIdx> {
        let flat_index = self.cells.iter().position(pred)?;
        let row = flat_index / self.cols;
        let col = flat_index % self.cols;
        if row < self.rows {
            let idx = GridIdx(row as isize, col as isize);
            Some(idx)
        } else {
            None
        }
    }

    pub fn column(&self, col: usize) -> GridLane<'_, T> {
        self.iter_from_with_stride(GridIdx(0, col as isize), GridOffset(1, 0))
    }

    pub fn row(&self, row: usize) -> GridLane<'_, T> {
        self.iter_from_with_stride(GridIdx(row as isize, 0), GridOffset(0, 1))
    }

    pub fn iter_from_with_stride(&self, start: GridIdx, stride: GridOffset) -> GridLane<'_, T> {
        GridLane {
            grid: self,
            state: start,
            stride,
        }
    }
}

pub struct GridLane<'a, T> {
    grid: &'a Grid<T>,
    state: GridIdx,
    stride: GridOffset,
}

impl<'a, T> Iterator for GridLane<'a, T> {
    type Item = (GridIdx, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let cell = self.grid.get(self.state)?;
        let result = (self.state, cell);
        self.state = self.state + self.stride;
        Some(result)
    }
}
