use std::{
    collections::{HashSet, VecDeque},
    fs,
    time::Instant,
};

use anyhow::Result;
use glam::{I64Vec2, IVec2};
use itertools::Itertools;

fn part_one(path: &str) -> Result<String> {
    let raw = fs::read_to_string(path)?;

    let points: Vec<I64Vec2> = raw
        .lines()
        .map(|line| {
            let coords: Vec<i64> = line.split(",").map(|v| v.parse().unwrap()).collect();
            I64Vec2::from([coords[0], coords[1]])
        })
        .collect();

    let result: i64 = points
        .iter()
        .tuple_combinations()
        .map(|(a, b)| ((b.x - a.x).abs() + 1) * ((b.y - a.y).abs() + 1))
        .max()
        .unwrap();

    Ok(result.to_string())
}

fn part_two(path: &str) -> Result<String> {
    let raw = fs::read_to_string(path)?;

    let points: Vec<IVec2> = raw
        .lines()
        .map(|line| {
            let coords: Vec<i64> = line.split(",").map(|v| v.parse().unwrap()).collect();
            IVec2::from([coords[0], coords[1]])
        })
        .collect();

    //  min, max: x & y
    let (x_min, x_max, y_min, y_max) = points.iter().fold(
        (i32::MAX, i32::MIN, i32::MAX, i32::MIN),
        |(x_min, x_max, y_min, y_max), point| {
            (
                x_min.min(point.x),
                x_max.max(point.x),
                y_min.min(point.y),
                y_max.max(point.y),
            )
        },
    );
    // dbg!(x_max, x_min, y_max, y_min);

    println!("Calculating Boundaries");
    // determine boundaries
    let mut boundary: HashSet<IVec2> = HashSet::new();

    for i in 0..points.len() {
        let p1 = points[i];
        let p2 = points[(i + 1) % points.len()];

        // vertical line
        if p1.x == p2.x {
            let (start, end) = if p1.y < p2.y {
                (p1.y, p2.y)
            } else {
                (p2.y, p1.y)
            };
            for y in start..=end {
                boundary.insert(IVec2::from([p1.x, y]));
            }
        } else {
            // horizontal
            let (start, end) = if p1.x < p2.x {
                (p1.x, p2.x)
            } else {
                (p2.x, p1.x)
            };
            for x in start..=end {
                boundary.insert(IVec2::from([x, p1.y]));
            }
        }
    }

    println!("Flood fill");
    // very naive external flood fill
    let mut exterior: HashSet<IVec2> = HashSet::new();
    let mut queue: VecDeque<IVec2> = VecDeque::new();

    queue.push_back(IVec2::ZERO);
    exterior.insert(IVec2::ZERO);

    while let Some(p) = queue.pop_front() {
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let np = IVec2::from([p.x + dx, p.y + dy]);

            if np.x < 0 || np.y < 0 || np.x > x_max + 1 || np.y > y_max + 1 {
                continue;
            }

            if boundary.contains(&np) || exterior.contains(&np) {
                continue;
            }

            exterior.insert(np);
            queue.push_back(np);
        }
    }

    println!("Building grid");
    let mut grid: Vec<Vec<u8>> = vec![vec![0; (x_max + 2) as usize]; (y_max + 2) as usize];

    println!("Filling grid");
    for y in 0..(y_max + 2) {
        for x in 0..(x_max + 2) {
            if exterior.contains(&IVec2::from([x, y])) {
                grid[y as usize][x as usize] = 1;
            }
        }
    }

    // dbg!(&grid.len(), &grid[0].len());
    for line in &grid {
        println!("{line:?}");
    }

    println!("Calculating pairs");
    let result: i32 = points
        .iter()
        .tuple_combinations()
        .filter(|(a, b)| {
            // filter combinations that don't fall inside the grid
            let (x_min, x_max) = (a.x.min(b.x) as usize, a.x.max(b.x) as usize);
            let (y_min, y_max) = (a.y.min(b.y) as usize, a.y.max(b.y) as usize);
            grid[y_min..y_max]
                .iter()
                .flat_map(|row| row[x_min..x_max].to_vec())
                .sum::<u8>()
                == 0
        })
        .map(|(a, b)| ((b.x - a.x).abs() + 1) * ((b.y - a.y).abs() + 1))
        .max()
        .unwrap();

    Ok(result.to_string())
}

fn main() -> Result<()> {
    let file_path = "inputs/test.txt";

    let start = Instant::now();
    let p1 = part_one(file_path)?;
    let duration = start.elapsed();
    println!("p1 solution: {p1} in {duration:?}"); // 321.311µs
    // test: 50
    // test: 4781377701

    let start = Instant::now();
    let p2 = part_two(file_path)?;
    let duration = start.elapsed();
    println!("p2 solution: {p2} in {duration:?}"); // 204.422µs
    // test:

    Ok(())
}
