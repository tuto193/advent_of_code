use crate::get_file_contents;
// use std::{cell::RefCell, rc::Rc};

mod node;

use node::Node;
type Maze = Vec<Vec<Node>>;
// type Maze = Rc<Vec<Vec<Rc<RefCell<Node>>>>>;

fn iddls(root: Node, maze: Maze) -> Option<Vec<Node>> {
    let mut current_depth = 0;
    loop {
        let (found, nodes) = dls(root, maze.clone(), current_depth);
        if found {
            return Some(nodes)
        } else if nodes.len() == 0 {
            return None;
        }
        current_depth += 1;
    }
}

fn dls(start: Node, maze: Maze, limit_depth: usize) -> (bool, Vec<Node>) {
    let mut depth_level: Vec<Node> = vec![start];
    let mut visited: Vec<Node> = vec![start];
    // let mut next_level: Vec<Node> = vec![];
    let mut path: Vec<Node> = vec![];
    let mut current_depth: usize = 0;
    let mut found = false;

    // println!("Trying depth {}", iterative_depth);
    while let Some(current) = depth_level.pop() {
        // visited.push(current);
        // current.mark_visited();
        path.push(current);
        // println!("\t-> Our path looks like so {}", path.len());
        if current.is_end_node() {
            // We found it
            found = true;
            break;
        }
        if current_depth == limit_depth {
            // We didn't find it at this depth
            break;
        }
        if current_depth < limit_depth {
            let mut new_neighbors = current.get_unvisited_neighbors(&maze, &visited);
            if new_neighbors.len() > 0 {
                depth_level.append(&mut new_neighbors);
                visited.append(&mut new_neighbors);
                current_depth += 1;
            } else {
                // current_depth -= 1;
                // Dead End
                path.pop();
            }
        }

    }
    (found, path)
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
        let mut neighbors = node.get_unvisited_neighbors(&maze, &visited);
        visited.append(&mut neighbors);
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
    let row_length = input[0].len();
    let height = input.len() - 1; // Because of last empty line
    let total_nodes = row_length * height;
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
    let mut current_depth: usize = 0;
    let mut path: Vec<Node> = vec![];
    while current_depth < total_nodes {
        let (found, nodes) = dls(starting_node, maze.clone(), current_depth);
        if found {
            path = nodes;
            break;
        }
        current_depth += 1;
    }

    println!("Found a way of length {}", path.len());
}
