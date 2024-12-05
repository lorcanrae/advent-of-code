use anyhow::Result;
use std::{env, fs};

struct ParsedData {
    rules: Vec<Vec<i32>>,
    pages: Vec<Vec<i32>>,
}

fn parse_input() -> Result<ParsedData> {
    let args: Vec<String> = env::args().collect();
    let file_name = args[1].to_string();
    let content = fs::read_to_string(file_name)?;

    let sections = content.split("\n\n").collect::<Vec<&str>>();

    let rules: Vec<Vec<i32>> = sections[0]
        .lines()
        .map(|line| {
            line.split("|")
                .filter_map(|x| x.trim().parse::<i32>().ok())
                .collect()
        })
        .collect();

    let pages: Vec<Vec<i32>> = sections[1]
        .lines()
        .map(|line| {
            line.split(",")
                .filter_map(|x| x.trim().parse::<i32>().ok())
                .collect()
        })
        .collect();
    Ok(ParsedData { rules, pages })
}

fn part_one(data: &ParsedData) -> i32 {
    let mut sum = 0;

    for page_set in &data.pages {
        let mut valid = true;
        let l = page_set.len();

        for i in 0..l {
            if i == l - 1 {
                break;
            }
            if i < l {
                let pair = vec![page_set[i], page_set[i + 1]];
                if data.rules.contains(&pair) {
                    continue;
                }
            }
            valid = false;
        }

        sum += if valid { page_set[l / 2] } else { 0 };
    }
    println!("Sum: {sum}");
    sum
}

fn part_two(data: &ParsedData) -> i32 {
    let mut sum_corrected = 0;

    for page_set in &data.pages {
        let mut corrected = false;
        let mut corrected_pages = page_set.clone();
        let l = corrected_pages.len();

        for i in 0..l {
            for j in i + 1..l {
                let switcheroo = vec![corrected_pages[j], corrected_pages[i]];
                if data.rules.contains(&switcheroo) {
                    corrected_pages.swap(i, j);
                    corrected = true;
                }
            }
        }

        sum_corrected += if corrected { corrected_pages[l / 2] } else { 0 };
    }
    println!("Sum corrected: {sum_corrected}");
    sum_corrected
}

fn main() {
    match parse_input() {
        Ok(data) => {
            part_one(&data);
            part_two(&data);
        }
        Err(e) => println!("Error opening file: {e}"),
    }
}
