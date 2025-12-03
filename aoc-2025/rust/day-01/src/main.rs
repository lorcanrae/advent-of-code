use anyhow::Result;
use std::{
    fs::File,
    io::{self, BufRead},
};

fn parse_data(path: &str) -> Result<Vec<(char, i32)>> {
    let reader = io::BufReader::new(File::open(path)?);

    let data: Vec<(char, i32)> = reader
        .lines()
        .map(|line| {
            let s = line.unwrap_or_default();
            let (letter, number) = s.split_at(1);
            let ch = letter.chars().next().unwrap();
            let num = number.parse().unwrap();
            (ch, num)
        })
        .collect();

    Ok(data)
}

fn part_one(data: &[(char, i32)]) -> Result<String> {
    let mut dial = 50;
    let mut zeroes = 0;

    data.iter().for_each(|(c, num)| {
        let sign = if *c == 'R' { 1 } else { -1 };
        dial = (dial + sign * num).rem_euclid(100);
        zeroes += if dial == 0 { 1 } else { 0 };
    });

    Ok(zeroes.to_string())
}

fn part_two(data: &[(char, i32)]) -> Result<String> {
    let mut dial = 50;
    let mut zeroes = 0;

    data.iter().for_each(|(c, num)| {
        let crossings = if *c == 'R' {
            // Adding
            (dial + num) / 100
        } else {
            // Subtracting
            if num >= &dial && dial == 0 {
                (num - dial) / 100
            } else if num >= &dial {
                1 + (num - dial) / 100
            } else {
                0
            }
        };
        // println!("Crossings found \t dial: {dial:?} \t direction: {c:?} \t num: {num:?} \t crossings: {crossings:?}");
        zeroes += crossings;

        let sign = if *c == 'R' { 1 } else { -1 };
        dial = (dial + sign * num).rem_euclid(100);
    });

    Ok(zeroes.to_string())
}

fn main() -> Result<()> {
    let file_path = "inputs/input.txt";
    let data = parse_data(file_path)?;

    let p1 = part_one(&data)?;
    println!("p1 solution: {}", p1);

    let p2 = part_two(&data)?;
    println!("p2 solution: {}", p2);
    // 2446 - too low
    // 7491 - too high

    Ok(())
}
