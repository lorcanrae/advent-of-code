use anyhow::Result;
use itertools::Itertools;
use nom::{
    bytes::complete::take_till, character::complete::satisfy, multi::many1, sequence::preceded,
    AsChar, IResult,
};
use nom_locate::LocatedSpan;
use std::{env, fs, iter::successors};

type Span<'a> = LocatedSpan<&'a str>;

fn char_pos(input: Span) -> IResult<Span, ((i32, i32), char)> {
    let x = (input.get_column() - 1) as i32;
    let y = (input.location_line() - 1) as i32;
    let (input, c) = satisfy(|c| c.is_alphanum())(input)?;
    Ok((input, ((x, y), c)))
}

fn parse_data(input: Span) -> IResult<Span, (Vec<((i32, i32), char)>, i32, i32)> {
    let n_rows = input.lines().count() as i32;
    let n_cols = input.lines().next().unwrap().chars().count() as i32;

    let (input, mut items) =
        many1(preceded(take_till(|c: char| c.is_alphanum()), char_pos))(input)?;
    // could be nice to sort into contiguous chunks here
    items.sort_by(|a, b| a.1.cmp(&b.1));
    Ok((input, (items, n_rows, n_cols)))
}

fn part_one(data: &[((i32, i32), char)], n_rows: &i32, n_cols: &i32) -> Result<String> {
    let x_bounds = 0..*n_cols;
    let y_bounds = 0..*n_rows;

    let result = data
        .chunk_by(|a, b| a.1 == b.1)
        .flat_map(|chunk| {
            chunk
                .iter()
                .combinations(2)
                .flat_map(|antennas| {
                    let x_diff = antennas[0].0 .0 - antennas[1].0 .0;
                    let y_diff = antennas[0].0 .1 - antennas[1].0 .1;

                    let plus = (antennas[0].0 .0 + x_diff, antennas[0].0 .1 + y_diff);
                    let minus = (antennas[1].0 .0 - x_diff, antennas[1].0 .1 - y_diff);

                    [plus, minus]
                })
                .filter(|potential| {
                    x_bounds.contains(&potential.0) && y_bounds.contains(&potential.1)
                })
        })
        .unique()
        .count();
    Ok(result.to_string())
}

fn part_two(data: &[((i32, i32), char)], n_rows: &i32, n_cols: &i32) -> Result<String> {
    let x_bounds = 0..*n_cols;
    let y_bounds = 0..*n_rows;

    let result = data
        .chunk_by(|a, b| a.1 == b.1)
        .flat_map(|chunk| {
            chunk
                .iter()
                .combinations(2)
                .flat_map(|antennas| {
                    let x_diff = antennas[0].0 .0 - antennas[1].0 .0;
                    let y_diff = antennas[0].0 .1 - antennas[1].0 .1;

                    let plus: Vec<_> = successors(Some(antennas[0].0), |antenna| {
                        let next_pos = (antenna.0 + x_diff, antenna.1 + y_diff);
                        if x_bounds.contains(&next_pos.0) && y_bounds.contains(&next_pos.1) {
                            Some(next_pos)
                        } else {
                            None
                        }
                    })
                    .collect();

                    let minus: Vec<_> = successors(Some(antennas[1].0), |antenna| {
                        let next_pos = (antenna.0 - x_diff, antenna.1 - y_diff);
                        if x_bounds.contains(&next_pos.0) && y_bounds.contains(&next_pos.1) {
                            Some(next_pos)
                        } else {
                            None
                        }
                    })
                    .collect();

                    [plus, minus]
                })
                .flatten()
        })
        .unique()
        .count();
    Ok(result.to_string())
}

fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();
    let file_name = args[1].to_string();
    let data = fs::read_to_string(file_name)?;

    let (_, (parsed_data, n_rows, n_cols)) = parse_data(Span::new(&data)).unwrap();

    let p1 = part_one(&parsed_data, &n_rows, &n_cols)?;
    println!("{p1}");

    let p2 = part_two(&parsed_data, &n_rows, &n_cols)?;
    println!("{p2}");

    Ok(())
}
