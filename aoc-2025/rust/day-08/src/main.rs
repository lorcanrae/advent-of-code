use anyhow::{Result, bail};
use glam::IVec3;
use itertools::Itertools;
use std::{collections::HashSet, fs, time::Instant};

fn part_one(path: &str, n_iter: usize) -> Result<String> {
    let raw = fs::read_to_string(path)?;
    let data: Vec<IVec3> = raw
        .lines()
        .map(|line| {
            let coords: Vec<i32> = line.split(",").map(|v| v.parse().unwrap()).collect();
            IVec3::from([coords[0], coords[1], coords[2]])
        })
        .collect();

    // calc distances between all points and order
    let dist_pairs = data
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (a.as_vec3().distance(b.as_vec3()), a, b))
        .sorted_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    // hashset was a terrible choice
    let mut all_circuits: Vec<HashSet<IVec3>> = Vec::new();

    // iterate n times
    for (_, p1, p2) in dist_pairs.take(n_iter) {
        // see if a point belongs to a circuit
        let mut pushed = false;
        for circuit in all_circuits.iter_mut() {
            if circuit.contains(p1) || circuit.contains(p2) {
                circuit.insert(*p2);
                circuit.insert(*p1);
                pushed = true;
            }
        }
        // other wise create new circuit
        if !pushed {
            let new_circuit: HashSet<IVec3> = HashSet::from([*p1, *p2]);
            all_circuits.push(new_circuit);
        }

        // merge connected circuits together
        let mut changed = true;
        while changed {
            changed = false;

            'outer: for i in 0..all_circuits.len() {
                for j in (i + 1)..all_circuits.len() {
                    let has_overlap = !all_circuits[j].is_disjoint(&all_circuits[i]);

                    if has_overlap {
                        let circuit_j = all_circuits.remove(j);
                        all_circuits[i].extend(circuit_j);

                        changed = true;
                        break 'outer;
                    }
                }
            }
        }
    }

    all_circuits.sort_by(|a, b| b.len().cmp(&a.len()));
    let result: usize = all_circuits.iter().map(|v| v.len()).take(3).product();

    Ok(result.to_string())
}

fn part_two(path: &str) -> Result<String> {
    let raw = fs::read_to_string(path)?;
    let data: Vec<IVec3> = raw
        .lines()
        .map(|line| {
            let coords: Vec<i32> = line.split(",").map(|v| v.parse().unwrap()).collect();
            IVec3::from([coords[0], coords[1], coords[2]])
        })
        .collect();

    // calc distances and order
    let dist_pairs = data
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (a.as_vec3().distance(b.as_vec3()), a, b))
        .sorted_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut all_circuits: Vec<HashSet<IVec3>> = Vec::new();

    // iterate over every pair
    for (_, p1, p2) in dist_pairs {
        // check to see if a point already belongs to a circuit
        let mut pushed = false;
        for circuit in all_circuits.iter_mut() {
            if circuit.contains(p1) || circuit.contains(p2) {
                circuit.insert(*p2);
                circuit.insert(*p1);
                pushed = true;
            }
        }
        if !pushed {
            let new_circuit: HashSet<IVec3> = HashSet::from([*p1, *p2]);
            all_circuits.push(new_circuit);
        }

        // merge connected circuits together
        let mut changed = true;
        while changed {
            changed = false;
            'outer: for i in 0..all_circuits.len() {
                for j in (i + 1)..all_circuits.len() {
                    let has_overlap = !all_circuits[j].is_disjoint(&all_circuits[i]);

                    if has_overlap {
                        let circuit_j = all_circuits.remove(j);
                        all_circuits[i].extend(circuit_j);

                        changed = true;
                        break 'outer;
                    }
                }
            }
        }

        // break when there is one circuit of length input junction boxes
        if all_circuits.len() == 1 && all_circuits[0].len() == data.len() {
            return Ok((p1.x * p2.x).to_string());
        }
    }

    bail!("Unprocessable");
}

fn main() -> Result<()> {
    let file_path = "inputs/input.txt";
    let n_iter = if file_path == "input/test.txt" {
        10
    } else {
        1000
    };

    let start = Instant::now();
    let p1 = part_one(file_path, n_iter)?;
    let duration = start.elapsed();
    println!("p1 solution: {p1} in {duration:?}");
    // test: 40

    let start = Instant::now();
    let p2 = part_two(file_path)?;
    let duration = start.elapsed();
    println!("p2 solution: {p2} in {duration:?}");
    // test:
    // 750567975 - too high
    // 11597417 - too low
    // 36045012

    Ok(())
}
