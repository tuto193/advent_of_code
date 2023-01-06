use advent_of_code::helpers::snake::*;

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

pub fn part_one(input: &str) -> Option<usize> {
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
    // println!(
    //     "The tail visited a total of '{}' different positions",
    //     visited_positions
    // );
    Some(visited_positions)
}

pub fn part_two(input: &str) -> Option<usize> {
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
    // println!(
    //     "The last tail visited a total of '{}' different positions",
    //     visited_positions
    // );
    Some(visited_positions)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), None);
    }
}
