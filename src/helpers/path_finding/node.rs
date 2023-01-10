// use std::{cell::RefCell, rc::Rc};

type Maze = Vec<Vec<Node>>;
use std::collections::HashSet;

// use crate::helpers::path_finding::node;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NodeType {
    Start,
    End,
    Normal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Node {
    coords: (usize, usize),
    height: usize,
    // visited: bool,
    // end_node: bool,
    node_type: NodeType,
    parent: Option<(usize, usize)>,
}

impl Node {
    pub fn new(coords: (usize, usize), height_letter: char) -> Self {
        let (height, node_type) = match height_letter {
            'E' => {
                // println!("End one was clearly found and marked");
                ('z' as usize + 1, NodeType::End)
            }
            'S' => ('a' as usize - 1, NodeType::Start),
            x => (x as usize, NodeType::Normal),
        };
        Self {
            coords: coords,
            height: height,
            // visited: false,
            // end_node: is_end_node,
            node_type: node_type,
            parent: None,
        }
    }

    // pub fn mark_visited(&mut self) {
    //     self.visited = true;
    // }
    pub fn set_parent(&mut self, parent: Node) {
        if let Some(_already) = self.parent {
            panic!("Tried to set the parent of this node more than once");
        }
        self.parent = Some(parent.coords);
    }

    pub fn get_coords(&self) -> (usize, usize) {
        self.coords
    }

    pub fn is_starting_node(&self) -> bool {
        self.node_type == NodeType::Start
    }
    /// Get all neighbors that are not visited yet and are also at most one higher than this one
    // pub fn get_unvisited_neighbors_coords(&self, maze: Maze) -> Vec<Rc<RefCell<Node>>> {
    pub fn get_unvisited_neighbors(&self, maze: &Maze, visited: &HashSet<Node>) -> Vec<Node> {
        // pub fn get_unvisited_neighbors_coords(&self, maze: &Vec<Vec<Node>>) -> Vec<Node> {
        // let mut neighbors: Vec<Rc<RefCell<Node>>> = vec![];
        // let maze = maze.clone();
        let mut neighbors: Vec<Node> = vec![];
        let maze_height = maze.len();
        let maze_width = maze[0].len();
        let (x, y) = self.coords;
        if x > 0 {
            let neighbor = maze[y][x - 1];
            // let neighbor = Rc::clone(&neighbor_og);
            // let neighbor = neighbor.try_borrow().unwrap();
            if !visited.contains(&neighbor)
                && neighbor.height < self.height + 2
                && neighbor.height >= self.height
            {
                neighbors.push(neighbor);
            }
        }
        if x < maze_width - 1 {
            let neighbor = maze[y][x + 1];
            // let neighbor = Rc::clone(&neighbor_og);
            // let neighbor = neighbor.try_borrow().unwrap();
            if !visited.contains(&neighbor)
                && neighbor.height < self.height + 2
                && neighbor.height >= self.height
            {
                neighbors.push(neighbor);
            }
        }
        if y > 0 {
            let neighbor = maze[y - 1][x];
            // let neighbor = Rc::clone(&neighbor_og);
            // let neighbor = neighbor.try_borrow().unwrap();
            if !visited.contains(&neighbor)
                && neighbor.height < self.height + 2
                && neighbor.height >= self.height
            {
                neighbors.push(neighbor);
            }
        }
        if y < maze_height - 1 {
            let neighbor = maze[y + 1][x];
            // let neighbor = Rc::clone(&neighbor_og);
            // let neighbor = neighbor.try_borrow().unwrap();
            if !visited.contains(&neighbor)
                && neighbor.height < self.height + 2
                && neighbor.height >= self.height
            {
                neighbors.push(neighbor);
            }
        }

        // dbg!(
        //     "Node {:?}, has neighbors: {:?}",
        //     self.coords,
        //     neighbors
        //         .iter()
        //         .map(|n| n.coords)
        //         .collect::<Vec<(usize, usize)>>()
        // );
        neighbors
    }

    pub fn is_end_node(&self) -> bool {
        self.node_type == NodeType::End
    }
    pub fn get_parent(&self) -> Option<(usize, usize)> {
        self.parent
    }
}

pub fn print_maze(current: Node, maze: &Maze, visited_nodes: &HashSet<Node>) {
    let width = maze[0].len();
    let height = maze.len();
    for y in 0..height {
        for x in 0..width {
            if current.get_coords() == (x, y) {
                print!("@");
            } else if visited_nodes.contains(&maze[y][x]) {
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
