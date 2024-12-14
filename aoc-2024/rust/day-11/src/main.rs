use std::{env, fs};

use anyhow::Result;

fn parse_data(data: String) -> Result<Vec<u64>> {
    let parsed = data
        .split_ascii_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();

    Ok(parsed)
}

fn split_or_return(stone: u64, steps: u32) -> u64 {
    if steps == 0 {
        return 1;
    }

    if stone == 0 {
        split_or_return(1, steps - 1)
    } else {
        let string = stone.to_string();
        let length = string.len();

        if length % 2 == 0 {
            let mid = length / 2;
            let left = string[..mid].parse::<u64>().unwrap();
            let right = string[mid..].parse::<u64>().unwrap();

            split_or_return(left, steps - 1) + split_or_return(right, steps - 1)
        } else {
            split_or_return(stone * 2024, steps - 1)
        }
    }
}

fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();
    let file_name = args[1].to_string();
    let data = fs::read_to_string(file_name)?;

    let parsed_data = parse_data(data).unwrap();

    let n_steps = 25;

    let p1: u64 = parsed_data
        .iter()
        .map(|&stone| split_or_return(stone, n_steps))
        .sum();
    println!("{p1:?}");

    Ok(())
}
