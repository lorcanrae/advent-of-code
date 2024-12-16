use std::{
    collections::{HashSet, VecDeque},
    env, fs,
};

use anyhow::Result;

fn parse_data(input: &str) -> Result<Vec<Vec<char>>> {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    Ok(matrix)
}

fn part_one(grid: &[Vec<char>]) -> Result<String> {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut visited = HashSet::new();
    let mut regions: Vec<(char, Vec<(usize, usize)>)> = Vec::new();

    let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)];

    // Naive flood fill
    for row in 0..rows {
        for col in 0..cols {
            if !visited.contains(&(row, col)) {
                let mut region = Vec::new();
                let mut queue = VecDeque::new();

                let character = grid[row][col];

                queue.push_back((row, col));
                visited.insert((row, col));

                while let Some((row, col)) = queue.pop_front() {
                    region.push((row, col));

                    for (dx, dy) in directions.iter() {
                        let new_row = row as isize + dx;
                        let new_col = col as isize + dy;

                        if new_row >= 0
                            && new_row < rows as isize
                            && new_col >= 0
                            && new_col < cols as isize
                        {
                            let new_row = new_row as usize;
                            let new_col = new_col as usize;

                            if !visited.contains(&(new_row, new_col))
                                && grid[new_row][new_col] == character
                            {
                                queue.push_back((new_row, new_col));
                                visited.insert((new_row, new_col));
                            }
                        }
                    }
                }
                regions.push((character, region))
            }
        }
    }
    // dbg!(&regions);

    // Now determine length of the perimeter of the region:
    let mut total_price = 0;
    for (character, points) in regions {
        let region_area = points.len();
        let mut region_perimeter = 0;
        for (row, col) in points {
            // Count neighbours
            let mut neighours = 0;

            for (dx, dy) in directions {
                let new_row = row as isize + dx;
                let new_col = col as isize + dy;

                // if point out of bounds, then not a neighbour;
                if new_row < 0
                    || new_row >= rows as isize
                    || new_col < 0
                    || new_col >= cols as isize
                {
                    continue;
                } else if grid[new_row as usize][new_col as usize] == character {
                    neighours += 1;
                }
            }
            region_perimeter += 4 - neighours;
        }
        total_price += region_area * region_perimeter;
    }

    Ok(total_price.to_string())
}

fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();
    let file_name = args[1].to_string();
    let data = fs::read_to_string(file_name)?;

    let parsed_data = parse_data(&data).unwrap();

    let p1 = part_one(&parsed_data)?;
    println!("{p1}");

    Ok(())
}
