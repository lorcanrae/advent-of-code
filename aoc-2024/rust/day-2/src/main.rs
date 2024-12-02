use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_data(path: &str) -> Result<Vec<Vec<i32>>> {
    let reader = BufReader::new(File::open(path)?);
    let data = reader
        .lines()
        .map(|line| {
            line.unwrap_or_default()
                .split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .collect();
    Ok(data)
}

fn part_one(data: &Vec<Vec<i32>>) -> i32 {
    let mut safe_rows = 0;

    for row in data {
        // All increasing
        let increasing = row.windows(2).all(|pair| {
            let diff = pair[1] - pair[0];
            (1..=3).contains(&diff)
        });
        // All decreasing
        let decreasing = row.windows(2).all(|pair| {
            let diff = pair[1] - pair[0];
            (-3..=-1).contains(&diff)
        });
        safe_rows += if increasing || decreasing { 1 } else { 0 };
    }
    safe_rows
}

fn part_two(data: &Vec<Vec<i32>>) -> i32 {
    let mut safe_rows = 0;

    for row in data {
        // All increasing
        let is_safe = |row: &[i32]| {
            let increasing = row.windows(2).all(|pair| {
                let diff = pair[1] - pair[0];
                (1..=3).contains(&diff)
            });
            // All decreasing
            let decreasing = row.windows(2).all(|pair| {
                let diff = pair[1] - pair[0];
                (-3..=-1).contains(&diff)
            });

            increasing || decreasing
        };

        if is_safe(row) {
            safe_rows += 1;
        } else {
            // start iterating sucker
            let mut valid_permutation = false;
            for i in 0..row.len() {
                let mut mutated_row = row.clone();
                mutated_row.remove(i);

                if is_safe(&mutated_row) {
                    valid_permutation = true;
                    break; // stop checking permutations
                }
            }
            if valid_permutation {
                safe_rows += 1;
            }
        }
    }
    safe_rows
}

fn main() -> Result<()> {
    let file_path = "inputs/input.txt";

    let data = parse_data(file_path).unwrap();

    let safe_rows = part_one(&data);

    println!("Part 1: {safe_rows}");

    let tolerant_rows = part_two(&data);

    print!("Part 2: {tolerant_rows}");

    Ok(())
}
