use crate::Result;
use crate::Error;
use std::str::FromStr;

pub fn parse_lines<T>(input: &str) -> Result<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    input
        .lines()
        .map(|line| {
            line.parse()
                .map_err(|e| crate::Error::Parse(format!("Failed to parse line: {}", e)))
        })
        .collect()
}

pub fn extract_numbers(s: &str) -> Vec<i64> {
    s.split(|c: char| !c.is_ascii_digit() && c != '-')
        .filter_map(|n| n.parse().ok())
        .collect()
}

pub fn parse_grid<T>(input: &str, parser: impl Fn(char) -> Option<T>) -> Result<Vec<Vec<T>>> {
    let grid: Vec<Vec<T>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let row: Vec<T> = line
                .chars()
                .filter_map(&parser)
                .collect();
            
            // Check if row is empty (all chars were filtered out)
            if row.is_empty() {
                return Err(Error::Parse(format!("Failed to parse line: {}", line)));
            }
            Ok(row)
        })
        .collect::<Result<Vec<Vec<T>>>>()?;

    // Check if grid is empty or has inconsistent row lengths
    if grid.is_empty() {
        return Err(Error::Parse("Empty grid".to_string()));
    }
    
    let expected_len = grid[0].len();
    if grid.iter().any(|row| row.len() != expected_len) {
        return Err(Error::Parse("Inconsistent row lengths".to_string()));
    }

    Ok(grid)
} 