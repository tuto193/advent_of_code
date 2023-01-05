// use std::{cell::RefCell, rc::Rc};

// type Maze = Rc<Vec<Vec<Rc<RefCell<Node>>>>>;
type Maze = Vec<Vec<Node>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Node {
    coords: (usize, usize),
    height: usize,
    visited: bool,
    end_node: bool,
}

impl Node {
    pub fn new(coords: (usize, usize), height_letter: char) -> Self {
        let height = match height_letter {
            'E' => 0,
            'S' => 69999,
            x => x as usize,
        };
        let is_end_node = height == 0;
        Self {
            coords: coords,
            height: height,
            visited: false,
            end_node: is_end_node,
        }
    }

    pub fn mark_visited(&mut self) {
        self.visited = true;
    }

    pub fn get_coords(&self) -> (usize, usize) {
        self.coords
    }

    pub fn is_starting_node(&self) -> bool {
        self.height == 69999
    }
    /// Get all neighbors that are not visited yet and are also at most one higher than this one
    // pub fn get_unvisited_neighbors_coords(&self, maze: Maze) -> Vec<Rc<RefCell<Node>>> {
    pub fn get_unvisited_neighbors(&self, maze: &Maze, all_visited: &Vec<Node>) -> Vec<Node> {
        // pub fn get_unvisited_neighbors_coords(&self, maze: &Vec<Vec<Node>>) -> Vec<Node> {
        // let mut neighbors: Vec<Rc<RefCell<Node>>> = vec![];
        let maze = maze.clone();
        let mut neighbors: Vec<Node> = vec![];
        let maze_height = maze.len();
        let maze_width = maze[0].len();
        let (x, y) = self.coords;
        if x > 0 {
            let neighbor = &maze[y][x - 1];
            // let neighbor = Rc::clone(&neighbor_og);
            // let neighbor = neighbor.try_borrow().unwrap();
            if !all_visited.contains(&neighbor) && neighbor.height < self.height + 2 {
                neighbors.push(*neighbor);
            }
        }
        if x < maze_width - 1 {
            let neighbor = &maze[y][x + 1];
            // let neighbor = Rc::clone(&neighbor_og);
            // let neighbor = neighbor.try_borrow().unwrap();
            if !all_visited.contains(&neighbor) && neighbor.height < self.height + 2 {
                neighbors.push(*neighbor);
            }
        }
        if y > 0 {
            let neighbor = &maze[y - 1][x];
            // let neighbor = Rc::clone(&neighbor_og);
            // let neighbor = neighbor.try_borrow().unwrap();
            if !all_visited.contains(&neighbor) && neighbor.height < self.height + 2 {
                neighbors.push(*neighbor);
            }
        }
        if y < maze_height - 1 {
            let neighbor = &maze[y + 1][x];
            // let neighbor = Rc::clone(&neighbor_og);
            // let neighbor = neighbor.try_borrow().unwrap();
            if !all_visited.contains(&neighbor) && neighbor.height < self.height + 2 {
                neighbors.push(*neighbor);
            }
        }
        neighbors
    }

    pub fn is_end_node(&self) -> bool {
        self.end_node
    }
}
