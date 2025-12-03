use anyhow::Result;
use std::fs;

fn parse(path: &str) -> Result<Vec<Vec<i32>>> {
    let input = fs::read_to_string(path)?;
    let data: Vec<Vec<i32>> = input
        .trim()
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();
    Ok(data)
}

fn part_one(file_path: &str) -> Result<String> {
    let data = parse(file_path)?;

    let result: i32 = data
        .iter()
        .map(|line| {
            // this makes me sad rust
            let slice = &line[..line.len() - 1];
            let first_max = slice.iter().max().unwrap();
            let first_max_index = slice.iter().position(|val| val == first_max).unwrap();

            let remaining = &line[(first_max_index + 1)..];
            let second_max = remaining.iter().max().copied().unwrap();
            format!("{first_max}{second_max}").parse::<i32>().unwrap()
        })
        .sum();

    Ok(result.to_string())
}

fn part_two(file_path: &str) -> Result<String> {
    let data = parse(file_path)?;

    let result: i64 = data
        .iter()
        .map(|line| {
            let mut vals: Vec<i64> = vec![];
            let mut index = 0;
            for j in (0..12).rev() {
                let end = line.len() - j;
                let slice = &line[index..end];
                let max = *slice.iter().max().unwrap() as i64;
                let max_index = slice.iter().position(|&val| val as i64 == max).unwrap();

                vals.push(max);
                index += max_index + 1;
            }

            vals.iter()
                .map(|v| v.to_string())
                .collect::<String>()
                .parse::<i64>()
                .unwrap()
        })
        .sum();

    Ok(result.to_string())
}

fn main() -> Result<()> {
    let file_path = "inputs/input.txt";

    let p1 = part_one(file_path)?;
    println!("P1 solution: {p1}");
    // 16937 - too low
    // 17309 - too high

    let p2 = part_two(file_path)?;
    println!("P2 solution: {p2}");
    // 16934976227338 - too low

    Ok(())
}
