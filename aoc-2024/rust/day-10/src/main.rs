use std::collections::HashMap;
use std::{env, fs};

use petgraph::dot::Dot;
use petgraph::prelude::*;

use anyhow::Result;
use petgraph::graph::DiGraph;
use petgraph::visit::Dfs;

fn parse_data(input: &str) -> Result<Vec<Vec<i32>>> {
    let matrix: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).map(|d| d as i32).unwrap())
                .collect()
        })
        .collect();

    Ok(matrix)
}

fn part_one(grid: &[Vec<i32>]) -> Result<String> {
    let rows = grid.len();
    let cols = grid[0].len();

    // Helper function to check valid grid bounds
    // let is_valid =
    //     |x: isize, y: isize| -> bool { x >= 0 && x < rows as isize && y >= 0 && y < cols as isize };

    // Build the graph
    let mut graph = DiGraph::new();
    let mut node_map = HashMap::new();

    for row in 0..rows {
        for col in 0..cols {
            let node = graph.add_node((row, col));
            node_map.insert((row, col), node);
        }
    }

    // Add edges to the graph
    // Directions for moving north, south, east, west
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    for row in 0..rows {
        for col in 0..cols {
            let current_value = grid[row][col];
            for &(dx, dy) in &directions {
                let new_row = row as isize + dx;
                let new_col = col as isize + dy;
                if new_row >= 0
                    && new_row < rows as isize
                    && new_col >= 0
                    && new_col < cols as isize
                {
                    let new_row = new_row as usize;
                    let new_col = new_col as usize;
                    let next_val = grid[new_row][new_col];
                    if next_val == current_value + 1 {
                        let from_node = node_map[&(row, col)];
                        let to_node = node_map[&(new_row, new_col)];
                        graph.add_edge(from_node, to_node, ());
                    }
                }
            }
        }
    }

    // println!("{:?}", Dot::with_config(&graph, &[]));

    // dbg!(&node_map);
    // todo!("");

    // Now find the reachable nines

    let mut results = HashMap::new();

    for ((row, col), start_node) in node_map {
        if grid[row][col] != 0 {
            continue;
        }
        let mut dfs = Dfs::new(&graph, start_node);
        let mut summits = 0;

        while let Some(node) = dfs.next(&graph) {
            let (r, c) = graph[node];
            if grid[r][c] == 9 {
                summits += 1;
            }
        }
        results.insert((row, col), summits);
    }

    let total_summits: i32 = results.iter().map(|(_, summits)| *summits as i32).sum();
    Ok(total_summits.to_string())
}

fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();
    let file_name = args[1].to_string();
    let data = fs::read_to_string(file_name)?;

    let parsed_data = parse_data(&data).unwrap();

    let p1 = part_one(&parsed_data).unwrap();

    println!("{p1}");

    Ok(())
}
