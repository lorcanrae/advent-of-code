use std::{env, fs};

use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    number::complete::double,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult, Parser,
};

#[derive(Debug)]
struct Machine {
    ax: f64,
    ay: f64,
    bx: f64,
    by: f64,
    tx: f64,
    ty: f64,
}

fn a_button(input: &str) -> IResult<&str, (f64, f64)> {
    preceded(
        tag("Button A: X+"),
        separated_pair(double, tag(", Y+"), double),
    )(input)
}

fn b_button(input: &str) -> IResult<&str, (f64, f64)> {
    preceded(
        tag("Button B: X+"),
        separated_pair(double, tag(", Y+"), double),
    )(input)
}

fn prize(input: &str) -> IResult<&str, (f64, f64)> {
    preceded(
        tag("Prize: X="),
        separated_pair(double, tag(", Y="), double),
    )(input)
}

fn parse_data(input: &str) -> IResult<&str, Vec<Machine>> {
    let (input, data) = separated_list1(
        line_ending,
        tuple((
            terminated(a_button, line_ending),
            terminated(b_button, line_ending),
            terminated(prize, line_ending),
        ))
        .map(|((ax, ay), (bx, by), (tx, ty))| Machine {
            ax,
            ay,
            bx,
            by,
            tx,
            ty,
        }),
    )(input)?;

    Ok((input, data))
}

fn part_one(machines: &[Machine]) -> Result<String> {
    let mut result = 0;
    // Do the math
    for machine in machines {
        let a = (machine.by * machine.tx - machine.bx * machine.ty)
            / (machine.by * machine.ax - machine.bx * machine.ay);

        let b = (machine.tx - machine.ax * a) / machine.bx;

        if a == a.trunc() && b == b.trunc() && a >= 0. && b >= 0. {
            result += a as i64 * 3 + b as i64;
        }
    }

    Ok(result.to_string())
}

fn part_two(machines: &[Machine]) -> Result<String> {
    let mut result = 0;
    let big_number = 10000000000000.0;

    // Do the math
    for machine in machines {
        let a = (machine.by * (machine.tx + big_number) - machine.bx * (machine.ty + big_number))
            / (machine.by * machine.ax - machine.bx * machine.ay);

        let b = ((machine.tx + big_number) - machine.ax * a) / machine.bx;

        if a == a.trunc() && b == b.trunc() && a >= 0. && b >= 0. {
            result += a as i64 * 3 + b as i64;
        }
    }

    Ok(result.to_string())
}

fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();
    let file_name = args[1].to_string();
    let data = fs::read_to_string(file_name)?;

    let (_, parsed_data) = parse_data(&data).unwrap();

    let p1 = part_one(&parsed_data).unwrap();

    println!("{p1}");

    let p2 = part_two(&parsed_data).unwrap();

    println!("{p2}");

    Ok(())
}
