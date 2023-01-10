use super::node::{print_maze, Node};
type Maze = Vec<Vec<Node>>;
type Position = (usize, usize);
use std::{
    collections::{HashSet, VecDeque},
    thread::current,
};

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

pub fn bfs(root: &Node, maze: &Maze) -> (Option<Vec<Node>>, usize) {
    //      let Q be a queue
    //      Q.enqueue(root)
    // let mut queue: Vec<Node> = vec![root];
    let mut queue: VecDeque<Node> = VecDeque::new();
    queue.push_front(*root);
    let mut next_level: Vec<Node> = vec![];

    //      label root as explored
    let mut visited: HashSet<Node> = HashSet::new();
    visited.insert(*root);
    let mut depth = 0;

    // let mut last_depth = 0;
    //      while Q is not empty do
    while let Some(current) = queue.pop_front() {
        // print_maze(current, maze, &visited);
        println!("Current's coords {:?}", current.get_coords());
        if current.is_end_node() {
            dbg!(current);
            println!("Found end node");
            return (Some(get_path_from_last(current, &maze)), depth);
        }

        let neighbors = current.get_unvisited_neighbors(&maze, &visited);
        let neighbors = neighbors.clone();
        for n in neighbors.into_iter() {
            visited.insert(n);
            next_level.push(n);
        }
        if queue.is_empty() {
            queue = next_level.drain(..).collect();
            // last_depth = depth;
            println!("Depth was {}", depth);
            depth += 1;
        }
    }
    (None, depth)
    // return (found, path);
}

fn get_path_from_last(end_node: Node, maze: &Maze) -> Vec<Node> {
    let mut path = vec![end_node];
    let mut current_node = end_node;
    while let Some((x, y)) = current_node.get_parent() {
        println!("Reverting: {:?}", current_node);
        let parent = maze[y][x];
        dbg!(parent);
        path.push(parent);
        current_node = parent;
        dbg!(current_node);
        print!("Then start again");
    }
    let path = path.into_iter().rev().collect();
    path
}

fn get_neighboring_coords_for(node: Position, maze: &Vec<Vec<u32>>) -> Vec<Position> {
    let current_height = maze[node.1][node.0];
    let mut neighbors = vec![];
    let width = maze[0].len();
    let height = maze.len();
    let (x, y) = node;
    if x > 0 {
        let neighbor = (x - 1, y);
        let neighbor_height = maze[neighbor.1][neighbor.0];
        if neighbor_height < current_height + 2 {
            neighbors.push(neighbor);
        }
    }
    if x < width - 1 {
        let neighbor = (x + 1, y);
        let neighbor_height = maze[neighbor.1][neighbor.0];
        if neighbor_height < current_height + 2 {
            neighbors.push(neighbor);
        }
    }
    if y < height - 1 {
        let neighbor = (x, y + 1);
        let neighbor_height = maze[neighbor.1][neighbor.0];
        if neighbor_height < current_height + 2 {
            neighbors.push(neighbor);
        }
    }
    if y > 0 {
        let neighbor = (x, y - 1);
        let neighbor_height = maze[neighbor.1][neighbor.0];
        if neighbor_height < current_height + 2 {
            neighbors.push(neighbor);
        }
    }

    neighbors
}

pub fn bfs_with_coords(
    start: Position,
    goal: Position,
    maze: &Vec<Vec<u32>>,
) -> Option<Vec<Position>> {
    let mut queue: VecDeque<(Vec<Position>, Position)> = VecDeque::new();
    queue.push_back((vec![start], start));
    let mut visited: HashSet<Position> = HashSet::new();
    // visited.insert(start);

    while let Some((path, current)) = queue.pop_front() {
        if current == goal
        // || maze[current.1][current.0] == 'z' as u32
        {
            return Some(path);
        }
        if !visited.contains(&current) {
            let visited_until_now: HashSet<Position> = visited.clone().into_iter().collect();
            let neighbors = get_neighboring_coords_for(current, maze);
            for n in neighbors
                .into_iter()
                .filter(|node| !visited_until_now.contains(node))
            {
                let mut p = path.clone();
                p.push(n);
                queue.push_back((p, n));
            }
            visited.insert(current);
        }
    }
    None
}
