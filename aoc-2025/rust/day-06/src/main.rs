use anyhow::{Result, anyhow};
use std::{fs, time::Instant};

fn parse(path: &str) -> Result<(Vec<Vec<u64>>, Vec<String>)> {
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
    let (nums, ops) = parse(path)?;

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

fn part_two_b(path: &str) -> Result<String> {
    let raw = fs::read_to_string(path)?;

    let lines: Vec<String> = raw.lines().map(|line| line.to_string()).collect();

    let num_rows = lines.len() - 1;
    let max_line_len = lines.iter().map(|line| line.len()).max().unwrap_or(0);

    let char_grid: Vec<Vec<char>> = lines
        .iter()
        .map(|line| {
            let mut chars: Vec<char> = line.chars().collect();
            chars.resize(max_line_len, ' ');
            chars
        })
        .collect();

    let mut buffer: Vec<u64> = Vec::new();
    let mut op = " ";
    let mut total: u64 = 0;

    for col_idx in (0..max_line_len).rev() {
        // check to see if at a column break
        let is_separator = char_grid.iter().all(|row| row[col_idx] == ' ');

        if is_separator && !buffer.is_empty() {
            // at a column break, do math, clear buffer
            match op {
                "+" => total += buffer.iter().sum::<u64>(),
                "*" => total += buffer.iter().product::<u64>(),
                _ => {}
            }
            buffer.clear();
            op = " ";
        } else if !is_separator {
            // get the val and push it to a buffer
            if let Ok(val) = char_grid[..num_rows]
                .iter()
                .map(|row| row[col_idx])
                .collect::<String>()
                .trim()
                .parse()
            {
                buffer.push(val);
            };
        }

        // Get the op and push to buffer
        match char_grid[num_rows][col_idx] {
            '+' => op = "+",
            '*' => op = "*",
            _ => {}
        }
    }

    // last val
    if !buffer.is_empty() {
        match op {
            "+" => total += buffer.iter().sum::<u64>(),
            "*" => total += buffer.iter().product::<u64>(),
            _ => panic!(),
        }
    }

    Ok(total.to_string())
}

fn part_two(path: &str) -> Result<String> {
    let raw = fs::read_to_string(path)?;
    let mut lines: Vec<String> = raw.lines().map(|line| line.to_string()).collect();

    let ops: Vec<String> = lines
        .pop()
        .ok_or_else(|| anyhow!("empty"))?
        .split_ascii_whitespace()
        .map(|c| c.to_string())
        .collect();

    let max_line_len = lines.iter().map(|line| line.len()).max().unwrap_or(0);

    // convert to grid and pad
    let char_grid: Vec<Vec<char>> = lines
        .iter()
        .map(|line| {
            let mut chars: Vec<char> = line.chars().collect();
            chars.resize(max_line_len, ' ');
            chars
        })
        .collect();

    // lambda for col of whitespaces
    let is_separator = |col_idx: usize| -> bool { char_grid.iter().all(|row| row[col_idx] == ' ') };

    // vec of tuples for individual columns
    let mut groups: Vec<(usize, usize)> = Vec::new();
    let mut start = 0;
    for col in 0..max_line_len {
        if is_separator(col) {
            if col > start {
                groups.push((start, col));
            }
            start = col + 1;
        }
    }

    // Don't forget the last one
    if start < max_line_len {
        groups.push((start, max_line_len));
    }

    // parse grid to useable values
    let nums: Vec<Vec<u64>> = groups
        .iter()
        .map(|(start, end)| {
            (*start..*end)
                .rev() // right to left
                .map(|col_idx| {
                    char_grid
                        .iter()
                        .map(|row| row[col_idx])
                        .collect::<String>()
                        .trim()
                        .parse()
                        .expect("invalid number in grid")
                })
                .collect()
        })
        .collect();

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

    let start = Instant::now();
    let p1 = part_one(file_path)?;
    let duration = start.elapsed();
    println!("p1 solution: {p1} in {duration:?}"); // 167.914µs
    // 5_381_996_914_800 - :check

    let start = Instant::now();
    let p2 = part_two(file_path)?;
    let duration = start.elapsed();
    println!("p2 solution: {p2} in {duration:?}"); // 240.926µs
    // 3_264_322 - too low
    // 9_734_866_307_577 - too high

    let start = Instant::now();
    let p2 = part_two_b(file_path)?;
    let duration = start.elapsed();
    println!("p2 - reverse parse: {p2} in {duration:?}"); // 240.926µs

    Ok(())
}
