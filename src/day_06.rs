use std::collections::{HashSet, VecDeque};

use crate::get_file_contents;

pub fn day_06() {
    let input = get_file_contents("06".to_owned());
    let mut counter: usize = 3;
    let input = input.chars().collect::<Vec<char>>();
    let mut different = &input[0..4];
    let iterations = input.len();
    for _ in 3..iterations {
        let set: HashSet<&char> = different.into_iter().collect();
        // println!("Before dedup {:?}", set);
        // set.dedup();
        println!("After dedup {:?}", set);
        if set.len() == 4 {
            break;
        }
        counter += 1;
        // let mut to_append = vec![c];
        // different.append(&mut to_append);
        different = &input[counter - 4..counter];
    }
    println!("The code is: {:?}", different);
    println!("Marker found after {}", counter);
}

pub fn day_06_part2() {
    let input = get_file_contents("06".to_owned());
    let message_length: usize = 14;
    let mut counter: usize = message_length - 1;
    let input = input.chars().collect::<Vec<char>>();
    let mut different = &input[0..message_length];
    let iterations = input.len();
    for _ in message_length - 1..iterations {
        let set: HashSet<&char> = different.into_iter().collect();
        // println!("Before dedup {:?}", set);
        // set.dedup();
        println!("After dedup {:?}", set);
        if set.len() == message_length {
            break;
        }
        counter += 1;
        // let mut to_append = vec![c];
        // different.append(&mut to_append);
        different = &input[counter - message_length..counter];
    }
    println!("The code is: {:?}", different);
    println!("Marker found after {}", counter);
}
