use anyhow::Result;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_data(path: &str) -> Result<String> {
    let reader = BufReader::new(File::open(path)?);
    let data: String = reader
        .lines()
        .filter_map(Result::ok)
        .collect::<Vec<String>>()
        .join("");
    Ok(data)
}

fn part_one(data: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut vals = Vec::new();
    for cap in re.captures_iter(data) {
        if let (Ok(a), Ok(b)) = (cap[1].parse::<i32>(), cap[2].parse::<i32>()) {
            vals.push((a, b));
        }
    }
    let result = vals.iter().map(|(a, b)| a * b).sum();
    println!("{result}");
    result
}

fn part_two(data: &str) -> i32 {
    let re = Regex::new(r"do\(\)|don't\(\)").unwrap();

    let mut do_vec = Vec::new();
    let mut last_match_end = 0;
    let mut last_match = 1;

    for (i, mat) in re.find_iter(data).enumerate() {
        // Initial case
        if i == 0 {
            do_vec.push(data[..mat.start()].to_string());
            last_match_end = mat.end();
            last_match = if mat.as_str() == "do()" { 1 } else { 0 };
        // Start of a do block
        } else if mat.as_str() == "do()" && last_match == 0 {
            last_match_end = mat.end();
            last_match = 1;
        // Append do block and start don't block
        } else if mat.as_str() == "don't()" && last_match == 1 {
            do_vec.push(data[last_match_end..mat.start()].to_string());
            last_match = 0;
        }
    }
    // Remainder if after a do
    if last_match == 1 {
        do_vec.push(data[last_match_end..].to_string());
    };
    // Apply part one to do blocks
    part_one(&do_vec.join(""))
}

fn main() {
    let file_path = "inputs/input.txt";
    let data = parse_data(file_path).unwrap();

    part_one(&data);

    part_two(&data);
}
