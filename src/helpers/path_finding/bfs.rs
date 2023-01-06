use super::node::Node;
type Maze = Vec<Vec<Node>>;
use std::collections::HashSet;

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

pub fn bfs(root: Node, maze: Maze) -> Option<Node> {
    //      let Q be a queue
    //      Q.enqueue(root)
    let mut queue: Vec<Node> = vec![root];

    //      label root as explored
    let mut visited: HashSet<Node> = HashSet::new();
    visited.insert(root);

    // let mut next_queue: Vec<Node> = vec![];
    // let mut path: Vec<Node> = vec![];
    // let mut found = false;

    //      while Q is not empty do
    while queue.len() > 0 {
        let current = queue[0];
        // path.push(current); // This might need to be moved

        // let path_clone = path.clone();
        ///// DEQUEUE
        if queue.len() > 1 {
            queue = queue.drain(1..).collect();
        } else {
            queue.pop();
        }
        ///// DEQUEUE

        if current.is_end_node() {
            return Some(current);
            // break;
        }

        let neighbors = current.get_unvisited_neighbors(&maze, &visited);
        for mut n in neighbors.clone().into_iter() {
            visited.insert(n);
            n.set_parent(current);
            // let mut to_append = path.
            queue.push(n);
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
