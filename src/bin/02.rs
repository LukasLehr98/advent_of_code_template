use advent_2024::Result;
use std::fs::read_to_string;

fn main() -> Result<()> {
    let input = read_to_string("input/day02.txt")?;
    
    let part1 = solve_part1(&input)?;
    let part2 = solve_part2(&input)?;
    
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    Ok(())
}

fn solve_part1(input: &str) -> Result<i64> {
    let nums: Vec<Vec<i64>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect()
        })
        .collect();

        let mut cols: (Vec<i64>, Vec<i64>) = (
            nums.iter().map(|row: &Vec<i64>| row[0]).collect::<Vec<i64>>(),
            nums.iter().map(|row: &Vec<i64>| row[1]).collect::<Vec<i64>>()
        );
        cols.0.sort();
        cols.1.sort();

        let total: i64 = cols.0.iter().zip(cols.1.iter()).map(|(a, b)| (a - b).abs()).sum();

    Ok(total)
}

fn solve_part2(input: &str) -> Result<i64> {
    let nums: Vec<Vec<i64>> = input
    .lines()
    .filter(|line| !line.is_empty())
    .map(|line| {
        line.split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect()
    })
    .collect();

    let cols: (Vec<i64>, Vec<i64>) = (
        nums.iter().map(|row: &Vec<i64>| row[0]).collect::<Vec<i64>>(),
        nums.iter().map(|row: &Vec<i64>| row[1]).collect::<Vec<i64>>()
    );

    let total: i64 = cols.0.iter().map(|a| {
        a * (cols.1.iter().filter(|&b| b == a).count() as i64)
    }).sum();

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const TEST_INPUT: &str = r#"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
        "#;
    
    #[test]
    fn test_part1() -> Result<()> {
        let result = solve_part1(TEST_INPUT.trim())?;
        println!("Got result: {}", result);
        assert_eq!(result, 11);
        Ok(())
    }
    
    #[test]
    fn test_part2() -> Result<()> {
        let result = solve_part2(TEST_INPUT.trim())?;
        println!("Got result: {}", result);
        assert_eq!(result, 11);
        Ok(())
    }
} 