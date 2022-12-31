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

#[derive(Copy, Clone, Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug)]
struct Tail {
    current: Position,
    head: Position,
    visited_position: HashSet<Position>
}

impl ops::Add for Position {
    type Output = Position;
    fn add(self, rhs: Position) -> Position {
        Position { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl ops::Sub for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y}
    }
}

impl Position {
    pub fn is_adjacent(&self, other: Self) -> bool {
        self.x == other.x || self.y == other.y
    }

    pub fn get_direction(dir: Direction) -> Self {
        let mut to_ret = Self{x: 0, y: 0};
        match dir {
            Up => to_ret.y = -1,
            Down => to_ret.y = 1,
            Left => to_ret.x = -1,
            Right => to_ret.x = 1,
        }

        to_ret
    }
}

impl Tail {
    pub fn new() -> Self {
        Self {
            current: Position{x: 0, y: 0},
            head: Position{x: 0, y: 0},
            visited_position: HashSet::new(),
        }
    }

}

pub fn day_09() {
    let input = get_file_contents("09".to_string());
}
