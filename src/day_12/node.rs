// use std::{cell::RefCell, rc::Rc};

use std::collections::HashSet;

use super::Maze;
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
}

impl Node {
    pub fn new(coords: (usize, usize), height_letter: char) -> Self {
        let (height, node_type) = match height_letter {
            'E' => ('z' as usize + 1, NodeType::End),
            'S' => ('a' as usize - 1, NodeType::Start),
            x => (x as usize, NodeType::Normal),
        };
        Self {
            coords: coords,
            height: height,
            // visited: false,
            // end_node: is_end_node,
            node_type: node_type,
        }
    }

    // pub fn mark_visited(&mut self) {
    //     self.visited = true;
    // }

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
            if !visited.contains(&neighbor) && neighbor.height < self.height + 2 && neighbor.height >= self.height {
                neighbors.push(neighbor);
            }
        }
        if x < maze_width - 1 {
            let neighbor = maze[y][x + 1];
            // let neighbor = Rc::clone(&neighbor_og);
            // let neighbor = neighbor.try_borrow().unwrap();
            if !visited.contains(&neighbor) && neighbor.height < self.height + 2 && neighbor.height >= self.height {
                neighbors.push(neighbor);
            }
        }
        if y > 0 {
            let neighbor = maze[y - 1][x];
            // let neighbor = Rc::clone(&neighbor_og);
            // let neighbor = neighbor.try_borrow().unwrap();
            if !visited.contains(&neighbor) && neighbor.height < self.height + 2 && neighbor.height >= self.height {
                neighbors.push(neighbor);
            }
        }
        if y < maze_height - 1 {
            let neighbor = maze[y + 1][x];
            // let neighbor = Rc::clone(&neighbor_og);
            // let neighbor = neighbor.try_borrow().unwrap();
            if !visited.contains(&neighbor) && neighbor.height < self.height + 2 && neighbor.height >= self.height {
                neighbors.push(neighbor);
            }
        }
        println!("Node {:?}, has neighbors: {:?}", self.coords, neighbors.iter().map(|n| n.coords).collect::<Vec<(usize, usize)>>());
        neighbors
    }

    pub fn is_end_node(&self) -> bool {
        self.node_type == NodeType::End
    }
}
