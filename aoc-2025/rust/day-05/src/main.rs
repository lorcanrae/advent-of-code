use std::{collections::HashSet, fs, ops::RangeInclusive, time::Instant};

use anyhow::Result;

fn parse(path: &str) -> Result<(Vec<RangeInclusive<u64>>, HashSet<u64>)> {
    let raw = fs::read_to_string(path)?;
    let mut sections = raw.split("\n\n");

    let ranges = sections
        .next()
        .unwrap()
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| {
            line.split_once("-")
                .map(|(start, end)| start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap())
        })
        .collect();

    let nums = sections
        .next()
        .unwrap()
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| line.trim().parse::<u64>().ok())
        .collect();

    Ok((ranges, nums))
}

fn part_one(file_path: &str) -> Result<String> {
    let (ranges, nums) = parse(file_path)?;

    let fresh = nums
        .iter()
        .filter(|n| ranges.iter().any(|range| range.contains(n)))
        .count();
    Ok(fresh.to_string())
}

fn main() -> Result<()> {
    let file_path = "inputs/input.txt";
    // let data = parse(file_path);
    // dbg!(&data);

    let start = Instant::now();
    let p1 = part_one(file_path)?;
    let duration = start.elapsed();
    println!("p1 solution: {p1} in {duration:?}"); // 416.372µs

    Ok(())
}
