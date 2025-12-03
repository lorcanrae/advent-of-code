use anyhow::Result;
use std::fs;

fn parse_data(path: &str) -> Result<Vec<(i64, i64)>> {
    let input_data: String = fs::read_to_string(path)?;
    let data: Vec<(i64, i64)> = input_data
        .trim()
        .split(",")
        .map(|line| {
            let (start, end) = line.split_once("-").unwrap_or_default();
            let start = start.parse::<i64>().unwrap();
            let end = end.parse::<i64>().unwrap();
            (start, end)
        })
        .collect();
    Ok(data)
}

fn part_one(data: &[(i64, i64)]) -> Result<String> {
    let invalid_ids: Vec<i64> = data
        .iter()
        .flat_map(|(start, end)| *start..=*end)
        .filter(|&num| {
            let stringed = num.to_string();
            if stringed.len() % 2 != 0 {
                return false;
            };
            let (a, b) = stringed.split_at(stringed.len() / 2);
            a == b
        })
        .collect();

    let result: i64 = invalid_ids.iter().sum();
    Ok(result.to_string())
}

fn part_two(data: &[(i64, i64)]) -> Result<String> {
    let invalid_ids: Vec<i64> = data
        .iter()
        .flat_map(|(start, end)| *start..=*end)
        .filter(|&num| {
            let stringed = num.to_string();
            let len = stringed.len();

            // Start from largest potential pattern and get smaller
            for pattern_len in (1..=len / 2).rev() {
                // Skip pattern lengths that don't modulo
                if len % pattern_len != 0 {
                    continue;
                }
                // Slice a section, repeat to fit length, compare
                let pattern = &stringed[..pattern_len];
                let repeated = pattern.repeat(len / pattern_len);
                if repeated == stringed {
                    return true;
                }
            }

            false
        })
        .collect();

    let result: i64 = invalid_ids.iter().sum();
    Ok(result.to_string())
}

fn main() -> Result<()> {
    let file_path = "inputs/test.txt";
    let data = parse_data(file_path)?;

    let p1 = part_one(&data)?;
    println!("p1 solution: {p1}");

    let p2 = part_two(&data)?;
    println!("p2 solution: {p2}");

    Ok(())
}
