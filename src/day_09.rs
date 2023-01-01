use std::{collections::HashSet, ops};

use crate::get_file_contents;

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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug)]
struct Tail {
    position: Position,
    head: Position,
    visited_position: HashSet<Position>,
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
}

impl Tail {
    pub fn new() -> Self {
        Self {
            position: Position { x: 0, y: 0 },
            head: Position { x: 0, y: 0 },
            visited_position: HashSet::new(),
        }
    }

    pub fn move_head(&mut self, direction: Direction) {
        // Head always moves
        let old_head = self.head;
        let new_head = self.head + Position::get_direction(direction);
        self.head = new_head;
        // Head either is or was on top
        if self.position == self.head || self.position.is_adjacent(self.head) {
            // We do nothing
        }
        // Mimic what the head does
        if self.position.is_adjacent(self.head)
    }
}

pub fn day_09() {
    let input = get_file_contents("09".to_string());
}
