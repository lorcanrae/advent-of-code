use anyhow::Result;
use std::fs;
use std::time::{Duration, Instant};

fn parse(path: &str) -> Result<Vec<Vec<GridCell>>> {
    let raw_data = fs::read_to_string(path)?;

    let data: Vec<Vec<GridCell>> = raw_data
        .trim()
        .split("\n")
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '@' => GridCell {
                        coords: (i, j),
                        has_paper: true,
                    },
                    '.' => GridCell {
                        coords: (i, j),
                        has_paper: (false),
                    },
                    _ => panic!(),
                })
                .collect()
        })
        .collect();
    Ok(data)
}

#[derive(Clone, Copy, Debug)]
struct GridCell {
    coords: (usize, usize),
    has_paper: bool,
}

impl GridCell {
    fn count_adjacent(&self, grid: &[Vec<GridCell>], num_rows: usize, num_cols: usize) -> usize {
        let mut adj_rolls = 0;
        // All the cases
        // left
        if self.coords.1 > 0 && grid[self.coords.0][self.coords.1 - 1].has_paper {
            adj_rolls += 1;
        }
        // right
        if self.coords.1 + 1 < num_cols && grid[self.coords.0][self.coords.1 + 1].has_paper {
            adj_rolls += 1;
        }
        // up
        if self.coords.0 > 0 && grid[self.coords.0 - 1][self.coords.1].has_paper {
            adj_rolls += 1;
        }
        // down
        if self.coords.0 + 1 < num_rows && grid[self.coords.0 + 1][self.coords.1].has_paper {
            adj_rolls += 1;
        }
        // up left
        if self.coords.0 > 0
            && self.coords.1 > 0
            && grid[self.coords.0 - 1][self.coords.1 - 1].has_paper
        {
            adj_rolls += 1;
        }
        // up right
        if self.coords.0 > 0
            && self.coords.1 + 1 < num_cols
            && grid[self.coords.0 - 1][self.coords.1 + 1].has_paper
        {
            adj_rolls += 1;
        }
        // down left
        if self.coords.0 + 1 < num_rows
            && self.coords.1 > 0
            && grid[self.coords.0 + 1][self.coords.1 - 1].has_paper
        {
            adj_rolls += 1;
        }

        // down right
        if self.coords.0 + 1 < num_rows
            && self.coords.1 + 1 < num_cols
            && grid[self.coords.0 + 1][self.coords.1 + 1].has_paper
        {
            adj_rolls += 1;
        }

        adj_rolls
    }

    fn remove_paper(&mut self) {
        self.has_paper = false
    }
}

fn part_one(file_path: &str) -> Result<String> {
    let data = parse(file_path)?;
    let mut counter = 0;

    let num_rows = data.len();
    let num_cols = if num_rows > 0 {
        data[0].len()
    } else {
        panic!()
    };

    for i_row in 0..num_rows {
        for j_col in 0..num_cols {
            if data[i_row][j_col].has_paper {
                let adj_rolls = data[i_row][j_col].count_adjacent(&data, num_rows, num_cols);

                counter += if adj_rolls < 4 { 1 } else { 0 }
            }
        }
    }

    Ok(counter.to_string())
}

fn part_two(file_path: &str) -> Result<String> {
    let mut data = parse(file_path)?;
    let mut total_removed = 0;

    let num_rows = data.len();
    let num_cols = if num_rows > 0 {
        data[0].len()
    } else {
        panic!()
    };

    loop {
        // state
        let mut removable_rolls: Vec<GridCell> = vec![];

        // calculate removable rolls
        for i_row in 0..num_rows {
            for j_col in 0..num_cols {
                if data[i_row][j_col].has_paper {
                    let adj_rolls = data[i_row][j_col].count_adjacent(&data, num_rows, num_cols);

                    if adj_rolls < 4 {
                        removable_rolls.push(data[i_row][j_col]);
                    }
                }
            }
        }

        // add number of removable rolls to accumulator
        if removable_rolls.is_empty() {
            break;
        } else {
            total_removed += removable_rolls.len()
        };

        // update grid
        for roll in removable_rolls {
            data[roll.coords.0][roll.coords.1].remove_paper();
        }
    }

    Ok(total_removed.to_string())
}

fn main() -> Result<()> {
    let file_path = "inputs/input.txt";

    let start = Instant::now();
    let p1 = part_one(file_path)?;
    let duration = start.elapsed();
    println!("p1 solution: {p1} in {duration:?}"); // 2.584 ms

    let start = Instant::now();
    let p2 = part_two(file_path)?;
    let duration = start.elapsed();
    println!("p2 solution: {p2} in {duration:?}"); // 55. 083 ms

    Ok(())
}
