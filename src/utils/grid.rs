use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn manhattan_distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid<T> {
    cells: Vec<Vec<T>>,
    rows: usize,
    cols: usize,
}

impl<T: Clone + Default> Grid<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            cells: vec![vec![T::default(); cols]; rows],
            rows,
            cols,
        }
    }

    pub fn get(&self, p: Point) -> Option<&T> {
        if self.in_bounds(p) {
            Some(&self.cells[p.y as usize][p.x as usize])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, p: Point) -> Option<&mut T> {
        if self.in_bounds(p) {
            Some(&mut self.cells[p.y as usize][p.x as usize])
        } else {
            None
        }
    }

    pub fn in_bounds(&self, p: Point) -> bool {
        p.x >= 0 && p.y >= 0 && 
        (p.x as usize) < self.cols && 
        (p.y as usize) < self.rows
    }
} 