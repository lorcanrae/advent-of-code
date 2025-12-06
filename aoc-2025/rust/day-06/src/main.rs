use anyhow::{Result, anyhow};
use std::{fs, time::Instant};

fn parse(path: &str) -> Result<(Vec<Vec<String>>, Vec<String>)> {
    let raw = fs::read_to_string(path)?;
    let mut lines: Vec<&str> = raw.lines().collect();

    let ops_line = lines.pop().ok_or_else(|| anyhow!("empty"))?;
    let ops: Vec<String> = ops_line
        .split_ascii_whitespace()
        .map(|c| c.to_string())
        .collect();

    let nums: Vec<Vec<String>> = lines
        .iter()
        .map(|line| {
            line.split_ascii_whitespace()
                // .filter_map(|n| n.parse::<u64>().ok())
                .map(|c| c.to_string())
                .collect()
        })
        .collect();

    Ok((nums, ops))
}

fn part_one(path: &str) -> Result<String> {
    let (nums, ops) = parse(path)?;

    let transposed: Vec<Vec<String>> = (0..nums[0].len())
        .map(|col_idx| nums.iter().map(|row| row[col_idx].clone()).collect())
        .collect();

    let total: u64 = transposed
        .iter()
        .zip(&ops)
        .map(|(nums, op)| match op.as_str() {
            "+" => nums.iter().map(|v| v.parse::<u64>().unwrap()).sum::<u64>(),
            "*" => nums
                .iter()
                .map(|v| v.parse::<u64>().unwrap())
                .product::<u64>(),
            _ => panic!(),
        })
        .collect::<Vec<u64>>()
        .iter()
        .sum();

    Ok(total.to_string())
}

fn main() -> Result<()> {
    let file_path = "inputs/input.txt";

    // let data = parse(file_path)?;
    // test p1 = 4_277_556

    let start = Instant::now();
    let p1 = part_one(file_path)?;
    let duration = start.elapsed();
    println!("p1 solution: {p1} in {duration:?}");

    // let start = Instant::now();
    // let p2 = part_two(file_path)?;
    // let duration = start.elapsed();
    // println!("p2 solution: {p2} in {duration:?}");

    Ok(())
}
