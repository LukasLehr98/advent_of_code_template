use advent_2024::Result;
use std::fs::read_to_string;

fn main() -> Result<()> {
    let input = read_to_string("input/dayXX.txt")?;
    
    let part1 = solve_part1(&input)?;
    let part2 = solve_part2(&input)?;
    
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    Ok(())
}

fn solve_part1(input: &str) -> Result<i64> {
    Ok(0) // TODO: Implement solution
}

fn solve_part2(input: &str) -> Result<i64> {
    Ok(0) // TODO: Implement solution
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const TEST_INPUT: &str = r#"
    // Add test input here
    "#;
    
    #[test]
    fn test_part1() -> Result<()> {
        let result = solve_part1(TEST_INPUT.trim())?;
        println!("Got result: {}", result);
        assert_eq!(result, 0);
        Ok(())
    }
    
    #[test]
    fn test_part2() -> Result<()> {
        let result = solve_part2(TEST_INPUT.trim())?;
        println!("Got result: {}", result);
        assert_eq!(result, 0);
        Ok(())
    }
} 