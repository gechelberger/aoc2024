use core::ops::{Add, Mul, Sub};
use std::path::Iter;

use itertools::Itertools;

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

pub fn parse_char_grid(input: &str) -> Grid<char> {
    Grid::<char>::parse(input, |x| x)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Grid<T> {
    pub cells: Vec<T>,
    pub rows: usize,
    pub cols: usize,
}

impl<T> Grid<T> {
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

    pub fn flat_index(&self, index: GridIdx) -> Option<usize> {
        let rows = 0..self.rows;
        let cols = 0..self.cols;
        if rows.contains(&index.row()) && cols.contains(&index.col()) {
            Some(index.row() * self.cols + index.col())
        } else {
            None
        }
    }

    pub fn grid_idx(&self, index: usize) -> Option<GridIdx> {
        let range = 0..self.cells.len();
        if range.contains(&index) {
            let index = index as isize;
            let cols = self.cols as isize;
            let grid_idx = GridIdx(index / cols, index % cols);
            Some(grid_idx)
        } else {
            None
        }
    }

    pub fn get(&self, index: GridIdx) -> Option<&T> {
        self.cells.get(self.flat_index(index)?)
    }

    pub fn put(&mut self, index: GridIdx, cell: T) -> bool {
        match self.flat_index(index) {
            Some(index) => {
                self.cells[index] = cell;
                true
            }
            None => false,
        }
    }

    pub fn position(&self, pred: impl Fn(&T) -> bool) -> Option<GridIdx> {
        self.grid_idx(self.cells.iter().position(pred)?)
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

    pub fn indices(&self) -> impl Iterator<Item = GridIdx> {
        (0..self.rows as isize)
            .cartesian_product(0..self.cols as isize)
            .map(|(r, c)| GridIdx(r, c))
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
