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
    Ok(0)
}

fn solve_part2(input: &str) -> Result<i64> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const TEST_INPUT: &str = r#"
    7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9
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