use std::{collections::HashSet, ops::{self}, hash::Hash, vec};

use crate::get_file_contents;

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    UpLeft,
    UpRight,
    Down,
    DownLeft,
    DownRight,
    Left,
    Right,
}

// #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
// struct Position {
//     x: i32,
//     y: i32,
// }

type Position = (i32, i32);
#[derive(Clone, Debug)]
struct Tail {
    position: Position,
    head: Position,
    visited_position: Vec<Position>,
}

// impl ops::Add for Position {
//     type Output = Position;
//     fn add(self, rhs: Position) -> Position {
//         Position{
//             x: self.0 + rhs.0,
//             y: self.1 + rhs.1,
//         }
//     }
// }

// impl ops::Sub for Position {
//     type Output = Self;

//     fn sub(self, other: Self) -> Self {
//         Self {
//             x: self.0 - other.0,
//             y: self.1 - other.1,
//         }
//     }
// }

fn is_adjacent(this: Position, other: Position) -> bool {
    let vector = (this.0 - other.0, this.1 - other.1);
    // vector.0.abs() + vector.1.abs() == 1 && (this.0 == other.0 || this.1 == other.1)
    let up = get_direction(Direction::Up);
    let down = get_direction(Direction::Down);
    let left = get_direction(Direction::Left);
    let right = get_direction(Direction::Right);
    vector == up || vector == down || vector == left || vector == right
}

fn get_direction(dir: Direction) -> Position {
    let (mut x, mut y) = (0, 0);
    match dir {
        Direction::Up => y = -1,
        Direction::Down => y = 1,
        Direction::Left => x = -1,
        Direction::Right => x = 1,
        Direction::UpLeft => {
            x = -1;
            y = -1;
        }
        Direction::UpRight => {
            x = 1;
            y = -1;
        }
        Direction::DownRight => {
            x = 1;
            y = 1;
        }
        Direction::DownLeft => {
            x = -1;
            y = 1;
        }
    }

    (x, y)
}

fn is_diagonal(this: Position, other: Position) -> bool {
    let vector = (this.0 - other.0, this.1 - other.1);
    !is_adjacent(this, other) && vector.0.abs() == 1 && vector.1.abs() == 1
}

impl Tail {
    pub fn new() -> Self {
        Self {
            position: (0, 0),
            head: (0, 0),
            visited_position: vec![(0, 0)],
        }
    }

    pub fn move_head(&mut self, direction: Direction, steps: usize) {
        if steps == 0 {
            return;
        }
        // Head always moves
        let old_head = self.head;
        let dir = get_direction(direction);
        let new_head = (self.head.0 + dir.0, self.head.1 + dir.1);
        self.head = new_head;
        // let on_top = self.position == self.head;
        // let is_adjacent = self.position.is_adjacent(self.head);
        // let is_diagonal = self.position.is_diagonal(self.head);
        // println!("On top? {} \nAdjacent? {} \nDiagonal? {}",on_top, is_adjacent, is_diagonal);
        // Head either is or was on top
        if self.position == self.head || // Head moved onto tail
            is_adjacent(self.position, self.head) || // head moved somewhere near tail
            is_diagonal(self.position, self.head) { // Head moved somewhere diagonal to tail
            // We do nothing
            // println!("Head moved, but tail didn't");
        } else {
            // println!("Tail moved from {:?} to {:?}", self.position, old_head);
            self.position = old_head;
            self.visited_position.push(self.position);
        }
        // Mimic what the head does
        self.move_head(direction, steps - 1);
    }

    pub fn get_visited_positions_set(&self) -> HashSet<Position> {
        let to_return: HashSet<Position> = self.visited_position.clone().into_iter().collect();
        println!("With repeated positions '{}'. Without repeated '{}'", self.visited_position.len(), to_return.len());
        to_return
    }
}

pub fn day_09() {
    let input = get_file_contents("09".to_string());
    let mut tail = Tail::new();
    for line in input.split("\n").into_iter() {
        let command: Vec<&str> = line.split(" ").into_iter().collect();
        let direction: Direction = match command[0] {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            x => {
                println!("Moved into an unexpected direction: '{}'", x);
                continue;
            }
        };
        let steps: usize = command[1].parse().unwrap();
        // println!("Moving '{}' to the '{:?}'", steps, direction);
        tail.move_head(direction, steps);
    }
    let visited_positions = tail.get_visited_positions_set().len();
    println!("The tail visited a total of '{}' different positions", visited_positions);
}
