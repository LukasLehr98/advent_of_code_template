pub mod grid;
pub mod parsing;
pub mod direction;
pub mod stack;

pub use grid::{Grid, Point};
pub use direction::Direction;
pub use stack::Stack;

pub fn parse_numbers(input: &str) -> Vec<i64> {
    input
        .lines()
        .filter_map(|line| line.trim().parse().ok())
        .collect()
}

// Create a 10x10 grid of booleans
let mut grid = Grid::<bool>::new(10, 10);

// Set a point
let p = Point::new(5, 3);
if let Some(cell) = grid.get_mut(p) {
    *cell = true;
}

// Check if two points are adjacent using Manhattan distance
let p1 = Point::new(0, 0);
let p2 = Point::new(1, 0);
assert_eq!(p1.manhattan_distance(&p2), 1);

// Parse lines into numbers
let numbers: Vec<i32> = parse_lines("123\n456\n789")?;

// Extract numbers from complex string
let coords = extract_numbers("Sensor at x=2, y=18");
assert_eq!(coords, vec![2, 18]);

// Parse a grid of walls and spaces
let maze = parse_grid(input, |c| match c {
    '#' => Some(true),  // wall
    '.' => Some(false), // space
    _ => None,          // ignore other characters
})?;