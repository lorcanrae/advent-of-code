use std::{collections::HashMap, env, fs};

use anyhow::Result;

fn parse_data(data: String) -> Result<Vec<u128>> {
    let parsed = data
        .split_ascii_whitespace()
        .filter_map(|s| s.parse::<u128>().ok())
        .collect();

    Ok(parsed)
}

fn split_or_return(stone: u128, steps: u32, cache: &mut HashMap<(u128, u32), u128>) -> u128 {
    // first check the cache
    if let Some(&result) = cache.get(&(stone, steps)) {
        return result;
    }

    // Base case
    if steps == 0 {
        return 1;
    }

    let result = if stone == 0 {
        split_or_return(1, steps - 1, cache)
    } else {
        let string = stone.to_string();
        let length = string.len();

        if length % 2 == 0 {
            let mid = length / 2;
            let left = string[..mid].parse::<u128>().unwrap();
            let right = string[mid..].parse::<u128>().unwrap();

            split_or_return(left, steps - 1, cache) + split_or_return(right, steps - 1, cache)
        } else {
            split_or_return(stone * 2024, steps - 1, cache)
        }
    };

    // Cache any given result
    cache.insert((stone, steps), result);

    result
}

fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();
    let file_name = args[1].to_string();
    let data = fs::read_to_string(file_name)?;

    let parsed_data = parse_data(data).unwrap();

    let n_steps = 75;

    let mut cache: HashMap<(u128, u32), u128> = HashMap::new();

    let p1: u128 = parsed_data
        .iter()
        .map(|&stone| split_or_return(stone, n_steps, &mut cache))
        .sum();
    println!("{p1:?}");

    Ok(())
}
