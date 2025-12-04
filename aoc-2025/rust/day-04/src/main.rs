use anyhow::Result;
use std::fs;
use std::time::Instant;

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
        const DIRECTIONS: [(isize, isize); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        DIRECTIONS
            .iter()
            .filter(|(dr, dc)| {
                let new_row = self.coords.0 as isize + dr;
                let new_col = self.coords.1 as isize + dc;

                new_row >= 0
                    && new_row < num_rows as isize
                    && new_col >= 0
                    && new_col < num_cols as isize
                    && grid[new_row as usize][new_col as usize].has_paper
            })
            .count()
    }

    fn remove_paper(&mut self) {
        self.has_paper = false
    }
}

fn part_one(file_path: &str) -> Result<String> {
    let data = parse(file_path)?;

    let num_rows = data.len();
    let num_cols = if num_rows > 0 {
        data[0].len()
    } else {
        panic!()
    };

    let counter = data
        .iter()
        .flatten()
        .filter(|cell| cell.has_paper)
        .filter(|cell| cell.count_adjacent(&data, num_rows, num_cols) < 4)
        .count();

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
        // per iteration state
        let mut removable_rolls: Vec<GridCell> = vec![];

        // find removable
        data.iter()
            .flatten()
            .filter(|cell| cell.has_paper)
            .filter(|cell| cell.count_adjacent(&data, num_rows, num_cols) < 4)
            .for_each(|&cell| removable_rolls.push(cell));

        // add num removable to accumulator else break
        if removable_rolls.is_empty() {
            break;
        } else {
            total_removed += removable_rolls.len()
        };

        // update grid state
        removable_rolls
            .iter()
            .for_each(|roll| data[roll.coords.0][roll.coords.1].remove_paper());
    }

    Ok(total_removed.to_string())
}

fn main() -> Result<()> {
    let file_path = "inputs/input.txt";

    let start = Instant::now();
    let p1 = part_one(file_path)?;
    let duration = start.elapsed();
    println!("p1 solution: {p1} in {duration:?}"); // 416.372µs
    // 1363

    let start = Instant::now();
    let p2 = part_two(file_path)?;
    let duration = start.elapsed();
    println!("p2 solution: {p2} in {duration:?}"); // 4.716294ms
    // 8184

    Ok(())
}
