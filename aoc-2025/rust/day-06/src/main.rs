use anyhow::{Result, anyhow};
use std::{fs, time::Instant};

fn parse_one(path: &str) -> Result<(Vec<Vec<u64>>, Vec<String>)> {
    let raw = fs::read_to_string(path)?;
    let mut lines: Vec<&str> = raw.lines().collect();

    let ops_line = lines.pop().ok_or_else(|| anyhow!("empty"))?;
    let ops: Vec<String> = ops_line
        .split_ascii_whitespace()
        .map(|c| c.to_string())
        .collect();

    let nums: Vec<Vec<u64>> = lines
        .iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .filter_map(|n| n.parse::<u64>().ok())
                .collect()
        })
        .collect();

    Ok((nums, ops))
}

fn part_one(path: &str) -> Result<String> {
    let (nums, ops) = parse_one(path)?;

    let transposed: Vec<Vec<u64>> = (0..nums[0].len())
        .map(|col_idx| nums.iter().map(|row| row[col_idx]).collect())
        .collect();

    let total: u64 = transposed
        .iter()
        .zip(&ops)
        .map(|(nums, op)| match op.as_str() {
            "+" => nums.iter().sum::<u64>(),
            "*" => nums.iter().product::<u64>(),
            _ => panic!(),
        })
        .collect::<Vec<u64>>()
        .iter()
        .sum();

    Ok(total.to_string())
}

fn parse_two(path: &str) -> Result<(Vec<Vec<u64>>, Vec<String>)> {
    let raw = fs::read_to_string(path)?;
    let mut lines: Vec<String> = raw.lines().map(|line| line.to_string()).collect();
    // let mut lines: Vec<&str> = raw.lines().collect();

    let ops_line = lines.pop().ok_or_else(|| anyhow!("empty"))?;
    let ops: Vec<String> = ops_line
        .split_ascii_whitespace()
        .map(|c| c.to_string())
        .collect();

    let max_line_length = lines.iter().map(|line| line.len()).max().unwrap_or(0);

    lines.iter_mut().for_each(|line| {
        if line.len() < max_line_length {
            let padding = " ".repeat(max_line_length - line.len());
            line.push_str(&padding);
        }
    });

    let break_points: Vec<usize> = (0..max_line_length)
        .filter(|&col_index| {
            lines
                .iter()
                .all(|row| row.chars().nth(col_index).map_or(true, |c| c == ' '))
        })
        .collect();

    let mut groups: Vec<(usize, usize)> = Vec::new();
    let mut start = 0;

    for bp in break_points {
        if bp > start {
            groups.push((start, bp));
        }
        start = bp + 1;
    }

    // last one
    if start < max_line_length {
        groups.push((start, max_line_length));
    }

    let nums: Vec<Vec<u64>> = groups
        .iter()
        .map(|(start, end)| {
            (*start..*end)
                .rev()
                .map(|col_idx| {
                    let num_str: String = lines
                        .iter()
                        .filter_map(|row| row.chars().nth(col_idx))
                        .collect();

                    num_str.trim().parse::<u64>().ok().unwrap()
                })
                .collect()
        })
        .collect();

    Ok((nums, ops))
}

fn part_two(path: &str) -> Result<String> {
    let (nums, ops) = parse_two(path)?;

    let total: u64 = nums
        .iter()
        .zip(&ops)
        .map(|(nums, op)| match op.as_str() {
            "+" => nums.iter().sum(),
            "*" => nums.iter().product(),
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
    // 5_381_996_914_800

    let start = Instant::now();
    let p2 = part_two(file_path)?;
    let duration = start.elapsed();
    println!("p2 solution: {p2} in {duration:?}");
    // 3_264_322 - too low
    // 9_734_866_307_577 - too high

    Ok(())
}
