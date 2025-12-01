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
        match c {
            'R' => {
                dial -= num;
            }
            'L' => {
                dial += num;
            }
            _ => unreachable!(),
        };

        dial %= 100;
        zeroes += if dial == 0 { 1 } else { 0 };
    });

    Ok(zeroes.to_string())
}

fn main() -> Result<()> {
    // Parse data

    let file_path = "inputs/input.txt";
    let data = parse_data(file_path)?;

    let p1 = part_one(&data)?;
    println!("p1 solution: {}", p1);

    Ok(())
}
