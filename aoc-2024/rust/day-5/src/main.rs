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
    // println!("{rules:?}");
    // println!("{pages:?}");

    Ok(ParsedData { rules, pages })
}

fn part_one(data: &ParsedData) -> i32 {
    let mut sum = 0;

    for page_set in &data.pages {
        let mut valid = true;
        for i in 0..page_set.len() {
            if i == 0 {
                continue;
            }

            if i != 0 {
                let prev_i = i - 1;
                let pair = page_set[prev_i..=i].to_vec();
                if data.rules.contains(&pair) {
                    continue;
                }
            }
            valid = false;
        }

        sum += if valid {
            page_set[page_set.len() / 2]
        } else {
            0
        };
    }
    println!("Sum: {sum}");
    sum
}

fn part_two(data: &ParsedData) -> i32 {
    let mut sum_valid = 0;
    let mut sum_corrected = 0;

    for page_set in &data.pages {
        let mut valid = true;
        let mut corrected = false;
        let mut corrected_pages = page_set.clone();
        let mut broken = false;

        for i in 0..corrected_pages.len() {
            if i == 0 {
                continue;
            }

            let prev_i = i - 1;
            if i != 0 {
                let pair = corrected_pages[prev_i..=i].to_vec();
                if data.rules.contains(&pair) {
                    continue;
                }
            }
            valid = false;

            if !valid {
                let swap = vec![corrected_pages[i], corrected_pages[prev_i]];
                if data.rules.contains(&swap) {
                    corrected_pages.swap(i, prev_i);
                    corrected = true;
                } else {
                    broken = true;
                }
            }
        }

        if broken {
            println!("Broken: {corrected_pages:?}");
        }

        if valid {
            sum_valid += page_set[page_set.len() / 2]
        } else if corrected {
            println!("{corrected_pages:?}");
            sum_corrected += corrected_pages[corrected_pages.len() / 2]
        };
    }
    println!("Sum valid: {sum_valid}");
    println!("Sum corrected: {sum_corrected}");
    sum_valid
}

fn main() {
    match parse_input() {
        Ok(data) => {
            // part_one(&data);
            part_two(&data);
        }
        Err(e) => println!("Error opening file: {e}"),
    }
}
