type Position = (usize, usize);
use std::collections::{HashSet, VecDeque};

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

fn get_neighboring_coords_for(
    node: Position,
    maze: &Vec<Vec<u32>>,
    backwards: bool,
) -> Vec<Position> {
    let current_height = maze[node.1][node.0];
    let mut neighbors = vec![];
    let width = maze[0].len();
    let height = maze.len();
    let (x, y) = node;
    if x > 0 {
        let neighbor = (x - 1, y);
        let neighbor_height = maze[neighbor.1][neighbor.0];
        if !backwards {
            if neighbor_height < current_height + 2 {
                neighbors.push(neighbor);
            }
        } else {
            if neighbor_height > current_height - 2 {
                neighbors.push(neighbor);
            }
        }
    }
    if x < width - 1 {
        let neighbor = (x + 1, y);
        let neighbor_height = maze[neighbor.1][neighbor.0];
        if !backwards {
            if neighbor_height < current_height + 2 {
                neighbors.push(neighbor);
            }
        } else {
            if neighbor_height > current_height - 2 {
                neighbors.push(neighbor);
            }
        }
    }
    if y < height - 1 {
        let neighbor = (x, y + 1);
        let neighbor_height = maze[neighbor.1][neighbor.0];
        if !backwards {
            if neighbor_height < current_height + 2 {
                neighbors.push(neighbor);
            }
        } else {
            if neighbor_height > current_height - 2 {
                neighbors.push(neighbor);
            }
        }
    }
    if y > 0 {
        let neighbor = (x, y - 1);
        let neighbor_height = maze[neighbor.1][neighbor.0];
        if !backwards {
            if neighbor_height < current_height + 2 {
                neighbors.push(neighbor);
            }
        } else {
            if neighbor_height > current_height - 2 {
                neighbors.push(neighbor);
            }
        }
    }

    neighbors
}

pub fn bfs_with_coords(
    start: Position,
    goal: Position,
    maze: &Vec<Vec<u32>>,
    backwards: bool,
) -> Option<Vec<Position>> {
    let mut queue: VecDeque<(Vec<Position>, Position)> = VecDeque::new();
    queue.push_back((vec![start], start));
    let mut visited: HashSet<Position> = HashSet::new();
    // visited.insert(start);

    while let Some((path, current)) = queue.pop_front() {
        if !backwards {
            if current == goal {
                // || maze[current.1][current.0] == 'z' as u32
                return Some(path);
            }
        } else {
            if maze[current.1][current.0] == 'a' as u32 {
                return Some(path);
            }
        }
        if !visited.contains(&current) {
            let visited_until_now: HashSet<Position> = visited.clone().into_iter().collect();
            let neighbors = get_neighboring_coords_for(current, maze, backwards);
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

pub fn print_maze(current: Position, maze: &Vec<Vec<u32>>, visited_nodes: &HashSet<Position>) {
    let width = maze[0].len();
    let height = maze.len();
    for y in 0..height {
        for x in 0..width {
            if current == (x, y) {
                print!("@");
            } else if visited_nodes.contains(&(x, y)) {
                print!("*");
            } else {
                print!("·");
            }
        }
        println!();
    }
    println!();
    // let duration = Duration::from_millis(3);
    // std::thread::sleep(duration);
    // for _ in 0..9 {
    //     println!();
    // }
}
