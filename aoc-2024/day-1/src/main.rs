use std::fs::File;
use std::io::{self, BufRead, Result};

fn parse_data(path: &str) -> Result<(Vec<i32>, Vec<i32>)> {
    // let file = File::open(path)?;
    let reader = io::BufReader::new(File::open(path)?);

    let mut column1: Vec<i32> = vec![];
    let mut column2: Vec<i32> = vec![];

    for line in reader.lines() {
        let numbers: Vec<i32> = line?
            .split_whitespace()
            .filter_map(|word| word.parse::<i32>().ok())
            .collect();

        column1.push(numbers[0]);
        column2.push(numbers[1]);
    }

    Ok((column1, column2))
}

fn sort_and_compute_difference(col1: &[i32], col2: &[i32]) -> i32 {
    assert_eq!(col1.len(), col2.len(), "Vectors must have the same length");

    let mut sorted_col1 = col1.to_vec();
    let mut sorted_col2 = col2.to_vec();

    sorted_col1.sort();
    sorted_col2.sort();

    sorted_col1
        .iter()
        .zip(sorted_col2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn similarity_score(col1: &[i32], col2: &[i32]) -> i32 {
    let mut similarity = 0;
    for element in col1.iter() {
        let count: i32 = col2.iter().filter(|n| *n == element).count() as i32;
        similarity += if count > 0 { element * count } else { 0 };
    }
    similarity
}

fn main() -> Result<()> {
    // Parse data
    let file_path = "inputs/input.txt";
    let (col1, col2) = parse_data(file_path)?;

    // Part one
    let sorted_diff = sort_and_compute_difference(&col1, &col2);
    println!("{sorted_diff}");

    // Part two
    let sim_score = similarity_score(&col1, &col2);
    print!("{sim_score}");

    Ok(())
}
