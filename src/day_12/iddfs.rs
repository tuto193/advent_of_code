use super::{Maze, Node};
use std::collections::HashSet;
// type Maze = Vec<Vec<Node>>;

pub fn iddfs(root: Node, maze: Maze) -> Option<Vec<Node>> {
    let mut visited: HashSet<Node> = HashSet::new();
    visited.insert(root);
    let mut current_depth: usize = 0;
    loop {
        // println!("Current depth in IDDFS: {}", current_depth);
        let (still_remaining, nodes) = dls(root, maze.clone(), visited.clone(), current_depth);
        if nodes.is_some() {
            // Foud a way
            return nodes;
        } else if !still_remaining {
            return None;
        }
        current_depth += 1
    }
}

fn dls(
    node: Node,
    maze: Maze,
    visited_by_parent: HashSet<Node>,
    depth: usize,
) -> (bool, Option<Vec<Node>>) {
    let mut there_are_more = false;
    let mut visited_by_parent = visited_by_parent.clone();
    if depth == 0 {
        if node.is_end_node() {
            return (there_are_more, Some(vec![node]));
        }
        // Already at depth. but maybe we could have carried on
        return (there_are_more, None);
    }
    // We need to go deeper!
    let neighbors = node.get_unvisited_neighbors(&maze, &visited_by_parent);
    for n in neighbors.clone().into_iter() {
        visited_by_parent.insert(n);
    }
    for neighbor in neighbors.into_iter() {
        let (remaining, nodes) = dls(
            neighbor,
            maze.clone(),
            visited_by_parent.clone(),
            depth - 1,
        );
        if let Some(mut ns) = nodes {
            let mut path = vec![node];
            path.append(&mut ns);
            return (true, Some(path));
        } else if remaining {
            // There is still at least one child
            there_are_more = true;
        }
    }
    (there_are_more, None)
}
