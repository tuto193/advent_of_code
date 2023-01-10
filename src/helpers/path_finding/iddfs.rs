use super::node::{print_maze, Node};
type Maze = Vec<Vec<Node>>;
use std::collections::HashSet;
// type Maze = Vec<Vec<Node>>;

pub fn iddfs(root: Node, maze: &Maze) -> Option<Vec<Node>> {
    let mut visited: HashSet<Node> = HashSet::new();
    visited.insert(root);
    let mut current_depth: usize = 0;
    loop {
        // println!("Current depth in IDDFS: {}", current_depth);
        let (still_remaining, nodes) = dls(root, &maze, &mut visited, current_depth);
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
    maze: &Maze,
    mut visited_by_parent: &mut HashSet<Node>,
    depth: usize,
) -> (bool, Option<Vec<Node>>) {
    // print_maze(node, &maze, &visited_by_parent);
    let mut remaining = false;
    // let mut visited_by_parent = visited_by_parent.clone();
    if depth == 0 {
        if node.is_end_node() {
            return (true, Some(vec![node]));
        }
        // Already at depth. but maybe we could have carried on
        return (true, None);
    }
    // visited_by_parent.insert(node);
    // We need to go deeper!
    // println!("Depth is {}", depth);
    // println!("Current parent's visited_nodes -> {:?}", visited_by_parent);
    let neighbors = node.get_unvisited_neighbors(&maze, &visited_by_parent);
    // for n in neighbors.iter() {
    //     visited_by_parent.insert(n);
    // }
    // println!("Afterwards ......visited_nodes -> {:?}", visited_by_parent);
    for neighbor in neighbors.iter() {
        // if ! visited_by_parent.contains(&neighbor) {
        //     println!("Panic! A clone of a neighbor does't have same hash");
        // }
        let (remain, nodes) = dls(*neighbor, &maze, &mut visited_by_parent, depth - 1);
        if let Some(mut ns) = nodes {
            let mut path = vec![node];
            path.append(&mut ns);
            return (true, Some(path));
        } else if remain {
            // There is still at least one child
            remaining = true;
        }
    }
    (remaining, None)
}
