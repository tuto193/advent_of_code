use std::{rc::Rc, cell::RefCell};

use crate::get_file_contents;

struct Node {
    coords: (usize, usize),
    height: usize,
    visited: bool,
    end_node: bool,
}

impl Node {
    pub fn new(coords: (usize, usize), height_letter: &str) -> Self {
        let height = match height_letter {
            "E" => 0,
            "S" => usize::MAX - 1,
            x => x.as_bytes()[0].try_into().unwrap(),
        };
        let is_end_node = height == 0;
        Self {
            coords: coords,
            height: height,
            visited: false,
            end_node: is_end_node,
        }
    }

    // pub fn get_unvisited_neighbors_coords(&self, maze: Vec<Vec<Rc<RefCell<Node>>>>) -> Vec<Rc<RefCell<Node>>> {
    pub fn get_unvisited_neighbors_coords(&self, maze: &Vec<Vec<Node>>) -> Vec<Node> {
        let mut neighbors: Vec<Rc<RefCell<Node>>> = vec![];
        let maze_height = maze.len();
        let maze_width = maze[0].len();
        neighbors
    }

    pub fn is_end_node(&self) -> bool {
        self.end_node
    }
}

fn iterative_dfs(node: Node, maze: &mut Vec<Vec<Node>>, iterative_depth: usize) -> bool {
    if node.is_end_node() {
        return true;
    }
    if iterative_depth == 0 {
        return false
    }
    false
}

pub fn part_1() {
    let input = get_file_contents("12".into());
}
