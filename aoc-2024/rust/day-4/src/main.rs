use anyhow::Result;
use std::env;
use std::fs;
use std::io::{BufRead, BufReader};

fn parse_input() -> Result<Vec<Vec<char>>> {
    let args: Vec<String> = env::args().collect();
    let file_name = args[1].to_string();
    let file = fs::File::open(file_name)?;
    let reader = BufReader::new(file);

    let mut matrix = Vec::new();
    for line in reader.lines() {
        let row = line?.chars().collect();
        matrix.push(row);
    }
    Ok(matrix)
}

fn part_one(matrix: &Vec<Vec<char>>) -> i32 {
    let mut counter = 0;
    let num_rows = matrix.len();
    let num_cols = if num_rows > 0 { matrix[0].len() } else { 0 };
    let target = ['X', 'M', 'A', 'S'];

    for i_row in 0..num_rows {
        for j_col in 0..num_cols {
            if matrix[i_row][j_col] == 'X' {
                // Right
                if j_col + 3 < num_cols && matrix[i_row][j_col..=j_col + 3] == target {
                    counter += 1;
                };

                // Left
                if j_col >= 3
                    && (0..4).map(|i| matrix[i_row][j_col - i]).collect::<Vec<_>>() == target
                // && matrix[i_row][j_col - 3..j_col] == target
                {
                    counter += 1;
                };

                // Up
                if i_row >= 3
                    && (0..4).map(|i| matrix[i_row - i][j_col]).collect::<Vec<_>>() == target
                {
                    counter += 1;
                };

                // Down
                if i_row + 3 < num_rows
                    && (0..4).map(|i| matrix[i_row + i][j_col]).collect::<Vec<_>>() == target
                {
                    counter += 1;
                };

                // Up Right
                if i_row >= 3
                    && j_col + 3 < num_cols
                    && (0..4)
                        .map(|i| matrix[i_row - i][j_col + i])
                        .collect::<Vec<_>>()
                        == target
                {
                    counter += 1;
                };

                // Up Left
                if i_row >= 3
                    && j_col >= 3
                    && (0..4)
                        .map(|i| matrix[i_row - i][j_col - i])
                        .collect::<Vec<_>>()
                        == target
                {
                    counter += 1;
                };

                // Down Right
                if i_row + 3 < num_rows
                    && j_col + 3 < num_cols
                    && (0..4)
                        .map(|i| matrix[i_row + i][j_col + i])
                        .collect::<Vec<_>>()
                        == target
                {
                    counter += 1;
                };

                // Down Left
                if i_row + 3 < num_rows
                    && j_col >= 3
                    && (0..4)
                        .map(|i| matrix[i_row + i][j_col - i])
                        .collect::<Vec<_>>()
                        == target
                {
                    counter += 1;
                };
            }
        }
    }
    println!("Found: {counter}");
    counter
}

fn main() {
    match parse_input() {
        Ok(matrix) => {
            part_one(&matrix);
        }
        Err(e) => {
            eprintln!("Error opening file: {e}");
        }
    }
}
