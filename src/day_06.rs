use std::collections::HashSet;

use crate::get_file_contents;

pub fn day_06() {
    let input = get_file_contents("06".to_owned());
    let mut counter: usize = 4;
    let input = input.chars().collect::<Vec<char>>();
    let mut different: Vec<char> = input[0..4].to_vec();
    for c in input.into_iter().skip(4) {
        counter += 1;
        let set: HashSet<char> = different.clone().into_iter().collect();
        if set.len() == 4 {
            break;
        }
        different.pop();
        let mut to_append = vec![c];
        different.append(&mut to_append);
    }
    println!("Marker found after {}", counter);
}
