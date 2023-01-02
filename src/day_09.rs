use std::{
    collections::HashSet,
    fmt::Display,
    hash::Hash,
    ops::{self},
    vec,
};

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

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// type Position = (i32, i32);
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
    pub fn is_adjacent(&self, other: Position) -> bool {
        let vector = *self - other;
        // vector.x.abs() + vector.y.abs() == 1 && (this.x == other.x || this.y == other.y)
        let up = Position::get_direction(Direction::Up);
        let down = Position::get_direction(Direction::Down);
        let left = Position::get_direction(Direction::Left);
        let right = Position::get_direction(Direction::Right);
        vector == up || vector == down || vector == left || vector == right
    }

    pub fn get_direction(dir: Direction) -> Position {
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

        Position { x: x, y: y }
    }

    pub fn is_diagonal(&self, other: Position) -> bool {
        let vector = *self - other;
        !self.is_adjacent(other) && vector.x.abs() == 1 && vector.y.abs() == 1
    }

    pub fn clip_length_to_dir(&mut self) -> Self {
        Position {
            x: self.x.max(-1).min(1),
            y: self.y.max(-1).min(1),
        }
    }
}

impl Direction {
    pub fn from_vector(dir: Position) -> Self {
        match dir {
            Position { x: 1, y: 0 } => Direction::Right,
            Position { x: -1, y: 0 } => Direction::Left,
            Position { x: 0, y: -1 } => Direction::Up,
            Position { x: 0, y: 1 } => Direction::Down,
            Position { x: 1, y: 1 } => Direction::DownRight,
            Position { x: -1, y: 1 } => Direction::DownLeft,
            Position { x: 1, y: -1 } => Direction::UpRight,
            Position { x: -1, y: -1 } => Direction::UpLeft,
            x => panic!("Unexpected direction: {}", x),
        }
    }
}

impl Tail {
    pub fn new() -> Self {
        Self {
            position: Position { x: 0, y: 0 },
            head: Position { x: 0, y: 0 },
            visited_position: vec![Position { x: 0, y: 0 }],
        }
    }

    pub fn move_head(&mut self, direction: Direction, steps: usize) {
        if steps == 0 {
            return;
        }
        // Head always moves
        let old_head = self.head;
        let dir = Position::get_direction(direction);
        let new_head = self.head + dir;
        self.head = new_head;
        // let on_top = self.position == self.head;
        // let is_adjacent = self.position.is_adjacent(self.head);
        // let is_diagonal = self.position.is_diagonal(self.head);
        // println!("On top? {} \nAdjacent? {} \nDiagonal? {}",on_top, is_adjacent, is_diagonal);
        // Head either is or was on top
        if self.position == self.head || // Head moved onto tail
            self.position.is_adjacent(self.head) || // head moved somewhere near tail
            self.position.is_diagonal(self.head)
        { // Head moved somewhere diagonal to tail
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

    pub fn follow_head(&mut self, new_head_pos: Position) {
        // let old_head = self.head;
        // let dir_vec = new_head_pos - self.position;
        self.head = new_head_pos;
        if self.position == self.head || // On top
          self.position.is_adjacent(self.head) ||
          self.position.is_diagonal(self.head)
        {
            // Already touching head, so no need to move tail
            // Move normally
            // let dir_vec = Direction::from_vector(dir_vec);
            // self.move_head(dir_vec, 1);
        } else {
            let mut tail_to_head_vec = self.head - self.position;
            let direction_to_move = tail_to_head_vec.clip_length_to_dir();
            self.position = self.position + direction_to_move;
            self.visited_position.push(self.position);
            // println!("Moving towards {}... {} --> {}",direction_to_move, old_pos, self.position);
            // let dir_vec = Direction::from_vector(Position{x: x, y: y});
            // self.move_head(dir_vec, 1)
        }
    }

    pub fn get_visited_positions_set(&self) -> HashSet<Position> {
        let to_return: HashSet<Position> = self.visited_position.clone().into_iter().collect();
        // println!("With repeated positions '{}'. Without repeated '{}'", self.visited_position.len(), to_return.len());
        to_return
    }

    // pub fn show_on_grid(&self, gird: &mut Vec<Vec<String>>, visited_positions_too: bool) {

    // }
}

pub fn day_09() {
    // let input = get_file_contents("009".to_string());
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
        // let to_print = vec![tail.clone()];
        // print_snake(&to_print);
    }
    let visited_positions = tail.get_visited_positions_set().len();
    println!(
        "The tail visited a total of '{}' different positions",
        visited_positions
    );
}

pub fn day_09_part2() {
    // let input = get_file_contents("009".to_string());
    let input = get_file_contents("09".to_string());
    let mut tails: Vec<Tail> = vec![];
    // Initialize all tails
    let last = 9;
    for _ in 0..last {
        tails.push(Tail::new());
    }

    // print_snake(&tails);
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
        move_snake(&mut tails, direction, steps);
    }
    let visited_positions = tails[last - 1].get_visited_positions_set().len();
    println!(
        "The last tail visited a total of '{}' different positions",
        visited_positions
    );
}

fn move_snake(tails: &mut Vec<Tail>, direction: Direction, steps: usize) {
    // println!("Moving {:?} for {} steps", direction, steps);
    for _ in 0..steps {
        // let mut actual_head = tails[0].position;
        tails[0].move_head(direction, 1);
        let mut actual_head = tails[0].position;
        // let mut actual_tail = Position{x: 0, y: 0};

        for t in tails.into_iter().skip(1) {
            let old_head = t.position;
            t.follow_head(actual_head);
            actual_head = old_head;
        }
    }
}
