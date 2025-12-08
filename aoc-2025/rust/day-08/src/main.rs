use anyhow::Result;
use glam::I64Vec3;
use std::{
    collections::{HashMap, HashSet},
    fs,
    time::Instant,
};

// struct Circuit {
//     connected_pairs: Vec<(I64Vec3, I64Vec3)>,
//     junction_boxes: HashSet<I64Vec3>,
// }

// impl Circuit {
//     fn new(p1: I64Vec3, p2: I64Vec3) -> Self {
//         let connected_pairs = vec![(p1, p2)];
//         let junction_boxes = HashSet::from([p1, p2]);

//         Self {
//             connected_pairs,
//             junction_boxes,
//         }
//     }

//     fn add_junction_box(&mut self, existing: I64Vec3, new: I64Vec3) {
//         self.connected_pairs.push((existing, new));
//         self.junction_boxes.insert(new);
//     }
// }

type Circuit = HashMap<I64Vec3, HashSet<I64Vec3>>;

fn part_one(path: &str, n_iter: usize) -> Result<String> {
    let raw = fs::read_to_string(path)?;
    let data: Vec<I64Vec3> = raw
        .lines()
        .map(|line| {
            let coords: Vec<i64> = line.split(",").map(|v| v.parse().unwrap()).collect();
            I64Vec3::from([coords[0], coords[1], coords[2]])
        })
        .collect();

    // brute force should work
    // will need some type of cache for distances
    let mut cache: Vec<(i64, (I64Vec3, I64Vec3))> = Vec::new();

    for i in 0..data.len() {
        for j in (i + 1)..data.len() {
            let dist = data[i].distance_squared(data[j]);
            cache.push((dist, (data[i], data[j])));
        }
    }
    // Sort by distance for easier lookup
    cache.sort_by_key(|&(distance, _)| distance);
    let cache_iter = cache.iter().take(n_iter);

    let mut all_circuits: Vec<Circuit> = Vec::new();

    // iterate n times
    for (_, (p1, p2)) in cache_iter {
        // for each pair
        let mut pushed = false;
        for circuit in all_circuits.iter_mut() {
            if circuit.contains_key(p1) || circuit.contains_key(p2) {
                circuit.entry(*p1).or_insert_with(HashSet::new).insert(*p2);
                circuit.entry(*p2).or_insert_with(HashSet::new).insert(*p1);
                pushed = true;
            }
        }

        if !pushed {
            let mut new_circuit: Circuit = HashMap::new();
            new_circuit.entry(*p1).or_default().insert(*p2);
            new_circuit.entry(*p2).or_default().insert(*p1);
            all_circuits.push(new_circuit);
        }

        // merge connected circuits together
        let mut changed = true;

        while changed {
            changed = false;

            'outer: for i in 0..all_circuits.len() {
                for j in (i + 1)..all_circuits.len() {
                    let has_overlap = all_circuits[i]
                        .keys()
                        .any(|key| all_circuits[j].contains_key(key));

                    if has_overlap {
                        let circuit_j = all_circuits.remove(j);

                        for (point, connections) in circuit_j {
                            all_circuits[i]
                                .entry(point)
                                .or_default()
                                .extend(connections);
                        }
                        changed = true;
                        break 'outer;
                    }
                }
            }
        }
    }

    // dbg!(all_circuits.len());

    let mut circuit_lengths: Vec<usize> = all_circuits.iter().map(|v| v.keys().count()).collect();
    circuit_lengths.sort();
    let top_three: Vec<usize> = circuit_lengths.iter().rev().take(3).copied().collect();

    let result = top_three.iter().product::<usize>();

    // dbg!(top_three, result);

    Ok(result.to_string())
}

fn part_two(path: &str) -> Result<String> {
    let raw = fs::read_to_string(path)?;
    let data: Vec<I64Vec3> = raw
        .lines()
        .map(|line| {
            let coords: Vec<i64> = line.split(",").map(|v| v.parse().unwrap()).collect();
            I64Vec3::from([coords[0], coords[1], coords[2]])
        })
        .collect();

    // brute force should work
    // will need some type of cache for distances
    let mut cache: Vec<(i64, (I64Vec3, I64Vec3))> = Vec::new();

    for i in 0..data.len() {
        for j in (i + 1)..data.len() {
            let dist = data[i].distance_squared(data[j]);
            cache.push((dist, (data[i], data[j])));
        }
    }
    // Sort by distance for easier lookup
    cache.sort_by_key(|&(distance, _)| distance);

    let mut all_circuits: Vec<Circuit> = Vec::new();
    let mut p1x = 0;
    let mut p2x = 0;

    // iterate all the times
    for (i, (_, (p1, p2))) in cache.iter().enumerate() {
        let prev_circuit_count = all_circuits.len();

        // for each pair
        // check to see if a point already belongs to a circuit
        let mut pushed = false;
        for circuit in all_circuits.iter_mut() {
            if circuit.contains_key(p1) || circuit.contains_key(p2) {
                circuit.entry(*p1).or_insert_with(HashSet::new).insert(*p2);
                circuit.entry(*p2).or_insert_with(HashSet::new).insert(*p1);
                pushed = true;
            }
        }
        // Or create a new circuit
        if !pushed {
            let mut new_circuit: Circuit = HashMap::new();
            new_circuit.entry(*p1).or_default().insert(*p2);
            new_circuit.entry(*p2).or_default().insert(*p1);
            all_circuits.push(new_circuit);
        }

        // merge connected circuits together
        let mut changed = true;
        while changed {
            changed = false;

            'outer: for i in 0..all_circuits.len() {
                for j in (i + 1)..all_circuits.len() {
                    let has_overlap = all_circuits[i]
                        .keys()
                        .any(|key| all_circuits[j].contains_key(key));

                    if has_overlap {
                        let circuit_j = all_circuits.remove(j);

                        for (point, connections) in circuit_j {
                            all_circuits[i]
                                .entry(point)
                                .or_default()
                                .extend(connections);
                        }
                        changed = true;
                        break 'outer;
                    }
                }
            }
        }
        // dbg!(all_circuits.len());
        if i > 2 && prev_circuit_count == 2 && all_circuits.len() == 1 {
            p1x = p1.x;
            p2x = p2.x;

            // dbg!(p1, p2);
            break;
        }
    }
    Ok((p1x * p2x).to_string())
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

    Ok(())
}
