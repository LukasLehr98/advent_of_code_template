use crate::Result;
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
    input
        .lines()
        .map(|line| line.chars().filter_map(parser).collect())
        .collect::<Vec<_>>()
        .into()
} 