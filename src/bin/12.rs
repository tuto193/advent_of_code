type Maze = Vec<Vec<Node>>;

use std::collections::HashSet;

use advent_of_code::helpers::path_finding::{
    bfs::{bfs, bfs_with_coords},
    iddfs::iddfs,
    node::Node,
};

fn get_starting_node(maze: Maze) -> Option<Node> {
    for r in maze.into_iter() {
        for node in r.into_iter() {
            if node.is_starting_node() {
                return Some(node);
            }
        }
    }
    None
}

fn get_end_node(maze: Maze) -> Option<Node> {
    for r in maze.into_iter() {
        for node in r.into_iter() {
            if node.is_end_node() {
                return Some(node);
            }
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<usize> {
    let input: Vec<&str> = input.split("\n").collect();
    // let row_length = input[0].len();
    let height = input.len() - 1; // Because of last empty line
                                  // let total_nodes = row_length * height;
    let input: Vec<Vec<char>> = input
        .into_iter()
        .take(height)
        .map(|r| {
            r.chars()
                .into_iter()
                // .skip(1) // We don't want empty stuff
                // .take(row_length)
                .collect()
        })
        .collect();
    // let maze: Maze = input
    //     .into_iter()
    //     .enumerate()
    //     .map(|(y, row)| {
    //         row.into_iter()
    //             .enumerate()
    //             .map(|(x, letter)| Node::new((x, y), letter))
    //             .collect()
    //     })
    //     .collect();
    let mut start: (usize, usize) = (0, 0);
    let mut goal: (usize, usize) = (0, 0);
    let maze: Vec<Vec<u32>> = input
        .into_iter()
        .enumerate()
        .map(|(y, row)| {
            row.into_iter()
                .enumerate()
                .map(|(x, letter)| match letter {
                    'S' => {
                        start = (x, y);
                        'a' as u32 - 1
                    }
                    'E' => {
                        goal = (x, y);
                        'z' as u32
                    }
                    l => l as u32,
                })
                .collect()
        })
        .collect();
    if let Some(path) = bfs_with_coords(start, goal, &maze) {
        return Some(path.len() - 1);
    }
    // let starting_node = get_starting_node(maze.clone()).unwrap();
    // let end_node = get_end_node(maze.clone()).unwrap();
    // println!("Start at {:?} and end at {:?}", starting_node.get_coords(), end_node.get_coords());

    // let path = bfs(starting_node, maze);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), None);
    }
}
