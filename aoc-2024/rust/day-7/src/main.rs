use anyhow::Result;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{self, newline, space1};
use nom::multi::separated_list1;
use nom::sequence::{self, separated_pair};
use nom::IResult;
use std::env;
use std::fs;

fn parse_input(data: &str) -> IResult<&str, Vec<(u64, Vec<u64>)>> {
    separated_list1(
        newline,
        separated_pair(
            complete::u64,
            tag(": "),
            separated_list1(space1, complete::u64),
        ),
    )(data)
}

fn part_one(data: Vec<(u64, Vec<u64>)>) -> Result<String> {
    let data = data.clone();

    let result: u64 = data
        .iter()
        .filter_map(|(target, sequence)| {
            let operator_count = sequence.len() - 1;
            dbg!(operator_count);
            (0..operator_count)
                .map(|_| vec!['+', '*'])
                .multi_cartesian_product()
                .any(|permutation| {
                    let mut operation = permutation.iter();
                    *target
                        == sequence
                            .iter()
                            .copied()
                            .reduce(|acc, next| match operation.next().unwrap() {
                                '+' => acc + next,
                                '*' => acc * next,
                                _ => panic!("Invalid operation"),
                            })
                            .unwrap()
                })
                .then_some(target)
        })
        .sum();
    Ok(result.to_string())
}

fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();
    let file_name = args[1].to_string();
    let data = fs::read_to_string(file_name)?;

    // let (_, parsed_data) = parse_input(data)?;
    match parse_input(&data) {
        Ok((_, parsed_data)) => {
            let p1 = part_one(parsed_data);
            println!("{p1:?}");
        }
        Err(e) => eprintln!("{e}"),
    }

    // dbg!(parsed_data);

    Ok(())
}
