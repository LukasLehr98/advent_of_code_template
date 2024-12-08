pub mod grid;
pub mod parsing;


pub use grid::{Grid, Point};

pub fn parse_numbers(input: &str) -> Vec<i64> {
    input
        .lines()
        .filter_map(|line| line.trim().parse().ok())
        .collect()
}
