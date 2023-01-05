use std::collections::HashSet;
use super::{Node, Maze};
// type Maze = Vec<Vec<Node>>;

pub fn iddfs(root: Node, maze: Maze) -> Option<Vec<Node>> {
    let mut visited: HashSet<Node> = HashSet::new();
    visited.insert(root);
    let mut current_depth: usize = 0;
    loop {
        // println!("Current depth in IDDFS: {}", current_depth);
        let (still_remaining, nodes) = dls(root, maze.clone(), &mut visited.clone(), current_depth);
        if nodes.is_some() { // Foud a way
            return nodes;
        } else if ! still_remaining {
            return None;
        }
        current_depth += 1
    }
}

fn dls(node: Node, maze: Maze, visited: &mut HashSet<Node>, depth: usize) -> (bool, Option<Vec<Node>>) {
    if depth == 0 {
        if node.is_end_node() {
            return (true, Some(vec![node]))
        }
        // Already at depth. but maybe we could have carried on
            return (true, None)
    }
    // We need to go deeper!
    // let mut visited = visited.clone();
    let mut still_remaining = false;
    // visited.push(node);
    let neighbors = node.get_unvisited_neighbors(&maze, &visited);
    for n in neighbors.clone().into_iter() {
        visited.insert(n);
    }
    // visited.append(&mut neighbors.clone());
    for neighbor in neighbors.into_iter() {
        let (remaining, nodes) = dls(neighbor, maze.clone(), &mut visited.clone(), depth - 1);
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
