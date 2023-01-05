use std::time::Instant;

use crate::get_file_contents;
// use std::{cell::RefCell, rc::Rc};

mod node;

use node::Node;
type Maze = Vec<Vec<Node>>;
// type Maze = Rc<Vec<Vec<Rc<RefCell<Node>>>>>;

fn iddfs(root: Node, maze: Maze) -> Option<Vec<Node>> {
    let mut current_depth = 0;
    loop {
        println!("Current depth in IDDFS: {}", current_depth);
        let (still_remaining, nodes) = dls(root, maze.clone(), vec![], current_depth);
        if nodes.is_some() { // Foud a way
            return nodes;
        } else if ! still_remaining {
            return None;
        }
        current_depth += 1;
    }
}

fn dls(node: Node, maze: Maze, previously_visited: Vec<Node>, depth: usize) -> (bool, Option<Vec<Node>>) {
    if depth == 0 {
        if node.is_end_node() {
            return (true, Some(vec![node]))
        }
        // Already at depth. but maybe we could have carried on
            return (true, None)
    }
    // We need to go deeper!
    let mut visited = previously_visited.clone();
    let mut still_remaining = false;
    visited.push(node);
    // let mut neighbors = node.get_unvisited_neighbors(&maze, &visited);
    // visited.append(&mut neighbors.clone());
    for neighbor in node.get_unvisited_neighbors(&maze, &visited) {
        let (remaining, nodes) = dls(neighbor, maze.clone(), visited.clone(), depth - 1);
        if let Some(mut ns) = nodes {
            let mut path = vec![node];
            path.append(&mut ns);
            return (true, Some(path));
        } else if remaining { // There is still at least one child
            still_remaining = true;
        }
    }
    (still_remaining, None)
}

fn bfs(node: Node, maze: Maze) -> (bool, Vec<Node>) {
    let mut level: Vec<Node> = vec![node];
    let mut visited: Vec<Node> = vec![node];
    let mut next_level: Vec<Node> = vec![];
    let mut path: Vec<Node> = vec![];
    let mut found = false;

    while let Some(current) = level.pop() {
        path.push(current);
        if current.is_end_node() {
            // We found it
            found = true;
            break;
        }
        let mut neighbors = current.get_unvisited_neighbors(&maze, &visited);
        visited.append(&mut neighbors.clone());
        next_level.append(&mut neighbors);

        if level.len() == 0 {
            level = next_level.drain(..).collect();
        }
    }
    (found, path)

}

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
        // println!("Trying depth {}", current_depth);
    // let (_found, path) = bfs(starting_node, maze.clone());
    // let mut current_depth: usize = 0;
    // let mut path: Vec<Node> = vec![];
    let time_before = Instant::now();
    let first_path = bfs(starting_node, maze);
    let elapsed_time_bfs = time_before.elapsed();
    println!("Found path at depth {} ({} seconds)", first_path.1.len(), elapsed_time_bfs.as_secs());
    // let time_before = Instant::now();
    // let path = iddfs(starting_node, maze);
    // let elapsed_time_iddfs = time_before.elapsed();
    // if let Some(found) = path {
    //     println!("Found a way of length {}", found.len());
    // } else {
    //     println!("No path found");
    // }
}
