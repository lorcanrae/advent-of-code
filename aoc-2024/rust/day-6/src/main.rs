use anyhow::Result;
use glam::IVec2;
use nom::{
    character::complete::{line_ending, one_of},
    multi::{many1, separated_list1},
    IResult,
};
use nom_locate::LocatedSpan;
use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

#[derive(Hash, PartialEq, Eq, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }

    fn to_ivec2(&self) -> IVec2 {
        match self {
            Direction::North => IVec2::NEG_Y,
            Direction::South => IVec2::Y,
            Direction::East => IVec2::X,
            Direction::West => IVec2::NEG_X,
        }
    }
}

type Span<'a> = LocatedSpan<&'a str>;

fn token(input: Span) -> IResult<Span, (IVec2, char)> {
    let x = (input.get_column() - 1) as i32;
    let y = (input.location_line() - 1) as i32;
    let (input, token) = one_of(".#^")(input)?;

    Ok((input, (IVec2::new(x, y), token)))
}

fn parse_input(data: Span) -> IResult<Span, ((IVec2, char), HashMap<IVec2, char>, i32, i32)> {
    let (input, items) = separated_list1(line_ending, many1(token))(data)?;

    let n_rows = items.len() as i32;
    let n_cols = items.iter().map(|row| row.len()).max().unwrap_or(0) as i32;
    let guard = items
        .iter()
        .flatten()
        .find(|(_, symbol)| symbol == &'^')
        .cloned()
        .expect("Didn't find the guard :(");

    let walls: HashMap<IVec2, char> = items
        .into_iter()
        .flatten()
        .filter(|(_, symbol)| symbol == &'#')
        .collect();

    Ok((input, (guard, walls, n_rows, n_cols)))
}

fn part_one(data: &(Span, ((IVec2, char), HashMap<IVec2, char>, i32, i32))) -> Result<String> {
    let (_, ((mut guard_pos, _), walls, n_rows, n_cols)) = data;

    let mut direction = Direction::North;

    let mut visited: HashSet<IVec2> = HashSet::from([guard_pos]);

    while (0..*n_rows).contains(&guard_pos.y) && (0..*n_cols).contains(&guard_pos.x) {
        let next_pos = guard_pos + direction.to_ivec2();
        if walls.contains_key(&next_pos) {
            direction = direction.turn_right();
        } else {
            // move forward
            visited.insert(guard_pos);
            guard_pos = next_pos;
        }
    }

    Ok(visited.len().to_string())
}

fn part_two(data: (Span, ((IVec2, char), HashMap<IVec2, char>, i32, i32))) -> Result<String> {
    let (_, ((mut guard_pos, _), walls, n_rows, n_cols)) = data;

    let guard_start_pos = guard_pos;

    let mut direction = Direction::North;
    let mut visited: HashSet<IVec2> = HashSet::from([guard_pos]);

    loop {
        let next_pos = guard_pos + direction.to_ivec2();

        if walls.contains_key(&next_pos) {
            direction = direction.turn_right();
        } else if (0..n_rows).contains(&guard_pos.y) && (0..n_cols).contains(&guard_pos.x) {
            // move forward
            visited.insert(guard_pos);
            guard_pos = next_pos;
        } else {
            break;
        }
    }
    visited.remove(&guard_start_pos);

    let loop_walls = visited
        .iter()
        .filter(|new_wall| {
            let mut direction = Direction::North;
            let mut guard_pos = guard_start_pos;

            let mut visited: HashSet<(IVec2, Direction)> =
                HashSet::from([(guard_pos, direction.clone())]);

            loop {
                let next_pos = guard_pos + direction.to_ivec2();

                if visited.contains(&(next_pos, direction.clone())) {
                    break true;
                } else if walls.contains_key(&next_pos) || &&next_pos == new_wall {
                    direction = direction.turn_right();
                    continue;
                } else if (0..n_rows).contains(&guard_pos.y) && (0..n_cols).contains(&guard_pos.x) {
                    visited.insert((guard_pos, direction.clone()));
                    guard_pos = next_pos;
                } else {
                    break false;
                }
            }
        })
        .count();

    Ok(loop_walls.to_string())
}

fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();
    let file_name = args[1].to_string();
    let data = fs::read_to_string(file_name)?;

    let parsed_data = parse_input(Span::new(&data)).unwrap();

    let p1 = part_one(&parsed_data)?;
    println!("{p1}");

    let parsed_data = parse_input(Span::new(&data)).unwrap();
    let p2 = part_two(parsed_data)?;
    println!("{p2}");

    Ok(())
}
