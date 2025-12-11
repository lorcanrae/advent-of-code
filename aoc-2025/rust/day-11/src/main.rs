use std::{collections::HashMap, time::Instant};

use anyhow::Result;
use pathfinding::prelude::count_paths;

fn parse(input: &str) -> Result<HashMap<&str, Vec<&str>>> {
    let data: HashMap<&str, Vec<&str>> = input
        .lines()
        .map(|line| {
            line.split_once(':')
                .map(|(s, e)| (s, e.split_ascii_whitespace().collect::<Vec<&str>>()))
                .unwrap()
        })
        .collect();

    Ok(data)
}

fn part_one(input: &str) -> Result<String> {
    let devices = parse(input).unwrap();
    let n = count_paths(
        "you",
        |device| devices[device].iter().copied(),
        |&target| target == "out",
    );

    Ok(n.to_string())
}

// This is weak and I'm totally fine with it
// Fully understand this could be done with DP
fn part_two(input: &str) -> Result<String> {
    let mut devices = parse(input)?;
    devices.insert("out", vec![]);

    let count = |from: &str, to: &str| {
        count_paths(
            from,
            |device| devices[device].iter().copied(),
            |&target| target == to,
        )
    };

    let [s_d, d_f, f_o, s_f, f_d, d_o] = [
        ("svr", "dac"),
        ("dac", "fft"),
        ("fft", "out"),
        ("svr", "fft"),
        ("fft", "dac"),
        ("dac", "out"),
    ]
    .map(|(from, to)| count(from, to));

    // in one of the sides the value will be zero, reducing that path to zero.
    let n_paths = (s_d * d_f * f_o) + (s_f * f_d * d_o);

    Ok(n_paths.to_string())
}

fn main() -> Result<()> {
    let data = include_str!("../inputs/input.txt");

    let start = Instant::now();
    let p1 = part_one(data)?;
    let duration = start.elapsed();
    println!("p1 solution: {p1} in {duration:?}"); // 116.967µs

    let start = Instant::now();
    let p2 = part_two(data)?;
    let duration = start.elapsed();
    println!("p2 solution: {p2} in {duration:?}"); // 256.788µs

    Ok(())
}
