use std::{
    collections::{HashMap, HashSet},
    fs,
    time::Instant,
};

use anyhow::Result;

fn parse(path: &str) -> Result<Vec<Vec<char>>> {
    let raw = fs::read_to_string(path)?;

    let grid: Vec<Vec<char>> = raw.lines().map(|line| line.chars().collect()).collect();

    Ok(grid)
}

fn part_one(path: &str) -> Result<String> {
    let grid = parse(path)?;

    // strip out redundant rows, probs be a feature of p2
    let grid = grid
        .into_iter()
        .filter(|line| line.contains(&'S') || line.contains(&'^'))
        .collect::<Vec<Vec<char>>>();

    let start = grid[0].iter().position(|c| *c == 'S').unwrap();
    let mut beams: HashSet<usize> = HashSet::from([start]);

    let mut splits = 0;

    for row in grid[1..].iter() {
        let splitters = row
            .iter()
            .enumerate()
            .filter(|(_, c)| c == &&'^')
            .map(|(i, _)| i)
            .collect::<HashSet<usize>>();

        let mut new_beams: HashSet<usize> = HashSet::new();
        for splitter in splitters.iter() {
            if beams.contains(splitter) {
                splits += 1;
                new_beams.extend([splitter + 1, splitter - 1]);
            }
        }

        // remove beams that hit splitters
        beams.retain(|beam| !splitters.contains(beam));
        // add newly split beams
        beams.extend(new_beams);
    }

    Ok(splits.to_string())
}

fn part_two(path: &str) -> Result<String> {
    let grid = parse(path)?;

    let grid = grid
        .into_iter()
        .filter(|line| line.contains(&'S') || line.contains(&'^'))
        .collect::<Vec<Vec<char>>>();

    let start = grid[0].iter().position(|c| *c == 'S').unwrap();

    let mut beams: HashMap<usize, usize> = HashMap::from([(start, 1)]);

    for line in grid.iter() {
        let mut new_positions = HashMap::<usize, usize>::new();

        for (index, count) in beams.iter() {
            let index = *index;
            let count = *count;
            if line.get(index).unwrap() == &'^' {
                new_positions
                    .entry(index - 1)
                    .and_modify(|val| *val += count)
                    .or_insert(count);

                new_positions
                    .entry(index + 1)
                    .and_modify(|val| *val += count)
                    .or_insert(count);
            } else {
                new_positions
                    .entry(index)
                    .and_modify(|val| *val += count)
                    .or_insert(count);
            }
        }

        beams = new_positions;
    }

    Ok(beams.values().sum::<usize>().to_string())
}

fn part_two_fold(path: &str) -> Result<String> {
    let binding = fs::read_to_string(path)?;
    let mut lines_iter = binding.lines().enumerate();

    let s_pos = lines_iter
        .next()
        .unwrap()
        .1
        .chars()
        .position(|val| val == 'S')
        .unwrap();

    let map = lines_iter.fold(
        HashMap::<usize, usize>::from([(s_pos, 1)]),
        |positions, (_, line)| {
            // dbg!(&positions);
            let mut new_positions = HashMap::<usize, usize>::new();
            for (index, count) in positions {
                if line.as_bytes()[index] == b'^' {
                    new_positions
                        .entry(index - 1)
                        .and_modify(|val| *val += count)
                        .or_insert(count);
                    new_positions
                        .entry(index + 1)
                        .and_modify(|val| *val += count)
                        .or_insert(count);
                } else {
                    new_positions
                        .entry(index)
                        .and_modify(|val| *val += count)
                        .or_insert(count);
                }
            }
            new_positions
        },
    );

    Ok(map.values().sum::<usize>().to_string())
}

fn main() -> Result<()> {
    let file_path = "inputs/input.txt";

    let start = Instant::now();
    let p1 = part_one(file_path)?;
    let duration = start.elapsed();
    println!("p1 solution: {p1} in {duration:?}"); // 321.311µs
    // test: 21

    let start = Instant::now();
    let p2 = part_two(file_path)?;
    let duration = start.elapsed();
    println!("p2 solution: {p2} in {duration:?}"); // 204.422µs
    // test: 40

    let start = Instant::now();
    let p2 = part_two_fold(file_path)?;
    let duration = start.elapsed();
    println!("p2 solution: {p2} in {duration:?}"); // 272.09µs
    // test: 40

    Ok(())
}
