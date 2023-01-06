fn parse_initial_status(init_stat: &str) -> Vec<Vec<String>> {
    let mut stacks = vec![];
    for _ in 0..9 {
        stacks.push(vec![]);
    }
    let rev_init_stat = init_stat
        .split("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .rev()
        .collect::<Vec<&str>>();
    for line in rev_init_stat.into_iter().skip(1) {
        // println!("Checking line: '{}'", line);
        let iterations = line.len() / 4;
        let columns = line.chars().collect::<Vec<char>>();
        for column in 0..iterations + 1 {
            let container = columns[1 + (column * 4)];
            // print!(" '{}' ", container);
            if container.is_ascii_alphabetic() {
                stacks[column].push(container.to_string())
            }
            // println!();
        }
    }
    // let stacks = stacks.to_vec();
    stacks.to_vec()
}

pub fn part_one(input: &str) -> Option<Vec<String>> {
    let input = input.split("\n\n").collect::<Vec<&str>>();
    let initial_status = input[0];
    let input = input[1];
    let mut stacks = parse_initial_status(initial_status);
    // print_stacks(&stacks);
    // println!("Initial status: {:?}", stacks);
    for task in input.split("\n").into_iter() {
        if task == "" {
            continue;
        }
        let move_from_to = task.split(" ").collect::<Vec<&str>>();
        let iterations: usize = move_from_to[1].parse().unwrap();
        let source: usize = move_from_to[3].parse().unwrap();
        let target: usize = move_from_to[5].parse().unwrap();
        for _ in 0..iterations {
            match stacks[source - 1].pop() {
                None => println!("Tried to take from an empty column"),
                Some(container) => stacks[target - 1].push(container),
            }
        }
    }
    let tops: Vec<String> = stacks
        .into_iter()
        .map(|stack| stack.pop().unwrap())
        .collect();
    Some(tops)
}

pub fn part_two(input: &str) -> Option<Vec<String>> {
    let input = input.split("\n\n").collect::<Vec<&str>>();
    let initial_status = input[0];
    let input = input[1];
    let mut stacks = parse_initial_status(initial_status);
    // print_stacks(&stacks);
    // println!("Initial status: {:?}", stacks);
    for task in input.split("\n").into_iter() {
        if task == "" {
            continue;
        }
        let move_from_to = task.split(" ").collect::<Vec<&str>>();
        let iterations: usize = move_from_to[1].parse().unwrap();
        let source: usize = move_from_to[3].parse().unwrap();
        let target: usize = move_from_to[5].parse().unwrap();
        let mut buffer: Vec<String> = vec![];
        for _ in 0..iterations {
            match stacks[source - 1].pop() {
                None => println!("Tried to take from an empty column"),
                Some(container) => buffer.push(container),
            }
        }
        for _ in 0..iterations {
            match buffer.pop() {
                None => println!("Error: buffer was emtpy while moving into target"),
                Some(container) => stacks[target - 1].push(container),
            }
        }
    }
    let tops: Vec<String> = stacks
        .into_iter()
        .map(|stack| stack.pop().unwrap())
        .collect();
    Some(tops)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), None);
    }
}
