use std::{collections::HashSet, ops, hash::Hash};

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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug)]
struct Tail {
    position: Position,
    head: Position,
    visited_position: Vec<Position>,
}

impl ops::Add for Position {
    type Output = Position;
    fn add(self, rhs: Position) -> Position {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Position {
    pub fn is_adjacent(&self, other: Self) -> bool {
        self.x == other.x || self.y == other.y
    }

    pub fn get_direction(dir: Direction) -> Self {
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

        Self{x, y}
    }

    pub fn is_diagonal(&self, other: Self) -> bool {
        let vector = *self - other;
        !self.is_adjacent(other) && vector.x != 0 && vector.y != 0
    }
}

impl Tail {
    pub fn new() -> Self {
        Self {
            position: Position { x: 0, y: 0 },
            head: Position { x: 0, y: 0 },
            visited_position: vec![Position{x: 0, y: 0}],
        }
    }

    pub fn move_head(&mut self, direction: Direction, steps: usize) {
        println!("Moving");
        if steps == 0 {
            return;
        }
        // Head always moves
        let old_head = self.head;
        let new_head = self.head + Position::get_direction(direction);
        self.head = new_head;
        // Head either is or was on top
        if self.position == self.head || self.position.is_adjacent(self.head) || self.position.is_diagonal(self.head) {
            // We do nothing
            println!("")
        } else {
            self.position = old_head.clone();
            self.visited_position.push(old_head);
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
            _ => {
                println!("Moved into an unexpected direction");
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
