use super::node::{Node, print_maze};
type Maze = Vec<Vec<Node>>;
use std::{collections::{HashSet, VecDeque}, thread::current};

//      let Q be a queue
//      label root as explored
//      Q.enqueue(root)
//      while Q is not empty do
//          v := Q.dequeue()
//          if v is the goal then
//              return v
//          for all edges from v to w in G.adjacentEdges(v) do
//              if w is not labeled as explored then
//                  label w as explored
//                  w.parent := v
//                  Q.enqueue(w)

pub fn bfs(root: Node, maze: Maze) -> Option<Vec<Node>> {
    //      let Q be a queue
    //      Q.enqueue(root)
    // let mut queue: Vec<Node> = vec![root];
    let mut queue: VecDeque<Node> = VecDeque::new();
    queue.push_front(root);

    //      label root as explored
    let mut visited: HashSet<Node> = HashSet::new();
    visited.insert(root);

    // let mut next_queue: Vec<Node> = vec![];
    // let mut path: Vec<Node> = vec![];
    // let mut found = false;

    //      while Q is not empty do
    while let Some(current) = queue.pop_front() {
        // let current = queue.pop_front().unwrap();
        // let current = queue:
        // print_maze(current, &maze, &visited);
        // path.push(current); // This might need to be moved

        // let path_clone = path.clone();
        ///// DEQUEUE
        // if queue.len() > 1 {
        //     let queue2 = queue.drain(1..).collect();
        //     queue = queue2;
        // } else {
        //     queue.pop();
        // }
        ///// DEQUEUE
        if current.is_end_node() {
            return Some(get_path_from_last(current, &maze));
            // break;
        }

        let neighbors = current.get_unvisited_neighbors(&maze, &visited);
        for mut n in neighbors.clone().into_iter() {
            visited.insert(n);
            n.set_parent(current);
            // let mut to_append = path.
            queue.push_back(n);
        }
        // visited.insert(&mut neighbors.clone());
        // level.append(&mut neighbors);
        // next_level.append(&mut neighbors);

        // if level.len() == 0 {
        //     level = next_level.drain(..).collect();
        //     path.pop();
        // }
    }
    None
    // return (found, path);
}

fn get_path_from_last(end_node: Node, maze: &Maze) -> Vec<Node> {
    let mut path = vec![end_node];
    let mut current_node = end_node;
    while let Some((x, y)) = current_node.get_parent() {
        let parent = maze[y][x];
        path.push(parent);
        current_node = parent;
    }
    let path = path.into_iter().rev().collect();
    path
}
