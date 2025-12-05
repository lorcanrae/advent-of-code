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

    let num_fresh = nums
        .iter()
        .filter(|n| ranges.iter().any(|range| range.contains(n)))
        .count();

    Ok(num_fresh.to_string())
}

fn part_two(file_path: &str) -> Result<String> {
    let (ranges, _) = parse(file_path)?;

    // numbers are too big to expand into collections of values :(

    // convert to (start, end) tuples and sort
    let mut sorted: Vec<(u64, u64)> = ranges.iter().map(|r| (*r.start(), *r.end())).collect();
    sorted.sort();

    // merge overlapping/adjacent
    let mut merged = vec![sorted[0]];

    sorted[1..].iter().for_each(|(start, end)| {
        let start = *start;
        let end = *end;

        let last = merged.last_mut().unwrap();

        // comp n-1.end vs n.start
        if start <= last.1 + 1 {
            last.1 = last.1.max(end);
        } else {
            merged.push((start, end));
        }
    });

    // Diff merged ranges, adjust for inclusive, sum
    let num_fresh: u64 = merged.iter().map(|(start, end)| end - start + 1).sum();

    Ok(num_fresh.to_string())
}

fn main() -> Result<()> {
    let file_path = "inputs/input.txt";

    let start = Instant::now();
    let p1 = part_one(file_path)?;
    let duration = start.elapsed();
    println!("p1 solution: {p1} in {duration:?}"); // 250.433µs

    let start = Instant::now();
    let p2 = part_two(file_path)?;
    let duration = start.elapsed();
    println!("p2 solution: {p2} in {duration:?}"); // 103.783µs

    Ok(())
}
