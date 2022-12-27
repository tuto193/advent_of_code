use crate::get_file_contents;

pub fn day_05() {
    let input = get_file_contents("05".to_owned());
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
    print_stacks_tops(stacks);
}

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

fn print_stacks(stacks: &Vec<Vec<String>>) {
    for (i, col) in stacks.into_iter().enumerate() {
        let cont = i + 1;
        println!("{}: {:?}", cont, col);
    }
}

fn print_stacks_tops(stacks: Vec<Vec<String>>) {
    for mut stack in stacks.into_iter() {
        let show = stack.pop();
        match show {
            Some(s) => print!("{}", s),
            None => print!(" "),
        }
    }
    println!()

}

pub fn day_05_part2() {
    let input = get_file_contents("05".to_owned());
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
    print_stacks_tops(stacks);

}
