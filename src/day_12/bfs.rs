use std::collections::HashSet;
use super::{Node, Maze};


pub fn bfs(start: Node, maze: Maze) -> (bool, Vec<Node>) {
    let mut queue: Vec<Node> = vec![start];
    let mut visited: HashSet<Node> = HashSet::new();
    visited.insert(start);
    // let mut next_level: Vec<Node> = vec![];
    let mut path: Vec<Node> = vec![];
    let mut found = false;

    while queue.len() > 0 {
        let current = queue[0];
        path.push(current); // This might need to be moved

        let path_clone = path.clone();
        ///// POP LEFT
        if queue.len() > 1 {
            queue = queue.drain(1..).collect();
        } else {
            queue.pop();
        }
        ///// POPPED LEFT
        if current.is_end_node() {
            found = true;
            break;
        } else if !visited.contains(&current) {
            visited.insert(current);
            queue.push(current);
            let path_visited: HashSet<Node> = path_clone.into_iter().collect();
            let neighbors = current.get_unvisited_neighbors(&maze, &path_visited);
            for n in neighbors.clone().into_iter() {
                path.push(n);
                // let mut to_append = path.
                queue.append(&mut path.clone());
            }
            // visited.insert(&mut neighbors.clone());
            // level.append(&mut neighbors);
            // next_level.append(&mut neighbors);

            // if level.len() == 0 {
            //     level = next_level.drain(..).collect();
            //     path.pop();
            // }
        }
    }
    return (found, path);
}
