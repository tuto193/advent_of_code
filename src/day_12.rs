use std::time::Instant;

use crate::get_file_contents;
// use std::{cell::RefCell, rc::Rc};

mod bfs;
mod iddfs;
mod node;
use bfs::bfs;
use iddfs::iddfs;
use node::Node;
type Maze = Vec<Vec<Node>>;

fn get_starting_node(maze: Maze) -> Option<Node> {
    for r in maze.into_iter() {
        for node in r.into_iter() {
            if node.is_starting_node() {
                return Some(node);
            }
        }
    }
    None
}

pub fn part_1() {
    // let input = get_file_contents("012".into());
    let input = get_file_contents("12".into());
    let input: Vec<&str> = input.split("\n").collect();
    // let row_length = input[0].len();
    let height = input.len() - 1; // Because of last empty line
                                  // let total_nodes = row_length * height;
    let input: Vec<Vec<char>> = input
        .into_iter()
        .take(height)
        .map(|r| {
            r.chars()
                .into_iter()
                // .skip(1) // We don't want empty stuff
                // .take(row_length)
                .collect()
        })
        .collect();
    let maze: Maze = input
        .into_iter()
        .enumerate()
        .map(|(y, row)| {
            row.into_iter()
                .enumerate()
                .map(|(x, letter)| Node::new((x, y), letter))
                .collect()
        })
        .collect();
    let starting_node = get_starting_node(maze.clone()).unwrap();

    /////// BFS
    // let time_before = Instant::now();
    // let path = bfs(starting_node, maze);
    // let elapsed_time_bfs = time_before.elapsed();
    // // println!("Maze should have 40 Nodes and has {}", total_nodes);
    // if let Some(found) = path {
    //     println!(
    //         "Found path at depth {} ({} seconds)",
    //         path.1.len() - 2,
    //         elapsed_time_bfs.as_secs_f32()
    //     );
    //     path.1
    //         .into_iter()
    //         .for_each(|n| println!("Positions are {:?}", n.get_coords()));
    // } else {
    //     println!("Didn't find shit");
    // }

    /////// IDDFS
    let time_before = Instant::now();
    let path = iddfs(starting_node, maze);
    let elapsed_time_iddfs = time_before.elapsed();
    println!("Time passed after 'finishing search' {} seconds", elapsed_time_iddfs.as_secs_f32());
    if let Some(found) = path {
        println!("Found a way of length {}", found.len() - 1);
        // for n in found.into_iter() {
        //     println!("{:?}", n.get_coords());
        // }
    } else {
        println!("No path found");
    }
}
