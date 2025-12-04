use std::fs;

use anyhow::Result;

fn parse(path: &str) -> Result<Vec<Vec<i32>>> {
    let raw_data = fs::read_to_string(path)?;

    let data: Vec<Vec<i32>> = raw_data
        .trim()
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '@' => 1,
                    '.' => 0,
                    _ => panic!(),
                })
                .collect()
        })
        .collect();
    Ok(data)
}

fn part_one(file_path: &str) -> Result<String> {
    let data = parse(file_path)?;
    let mut counter = 0;

    let num_rows = data.len();
    let num_cols = if num_rows > 0 { data[0].len() } else { 0 };

    // dbg!(num_rows, num_cols);
    for i_row in 0..num_rows {
        for j_col in 0..num_cols {
            if data[i_row][j_col] == 1 {
                // all the cases
                let mut adj_rolls = 0;
                // left
                if j_col > 0 && data[i_row][j_col - 1] == 1 {
                    adj_rolls += 1;
                };
                // right
                if j_col + 1 < num_cols && data[i_row][j_col + 1] == 1 {
                    adj_rolls += 1;
                }
                // up
                if i_row > 0 && data[i_row - 1][j_col] == 1 {
                    adj_rolls += 1;
                }
                // down
                if i_row + 1 < num_rows && data[i_row + 1][j_col] == 1 {
                    adj_rolls += 1;
                }
                // up left
                if i_row > 0 && j_col > 0 && data[i_row - 1][j_col - 1] == 1 {
                    adj_rolls += 1;
                }
                // up right
                if i_row > 0 && j_col + 1 < num_cols && data[i_row - 1][j_col + 1] == 1 {
                    adj_rolls += 1;
                }
                // down left
                if i_row + 1 < num_rows && j_col > 0 && data[i_row + 1][j_col - 1] == 1 {
                    adj_rolls += 1;
                }
                // down right
                if i_row + 1 < num_rows && j_col + 1 < num_cols && data[i_row + 1][j_col + 1] == 1 {
                    adj_rolls += 1;
                }
                counter += if adj_rolls < 4 { 1 } else { 0 }
            }
        }
    }

    Ok(counter.to_string())
}

fn main() -> Result<()> {
    let file_path = "inputs/input.txt";
    // let data = parse(file_path)?;
    // dbg!(data);

    let p1 = part_one(file_path)?;
    println!("P1 solution: {p1}");

    Ok(())
}
