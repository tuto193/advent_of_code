use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<usize> {
    let mut counter: usize = 3;
    let input = input.chars().collect::<Vec<char>>();
    let mut different = &input[0..4];
    let iterations = input.len();
    for _ in 3..iterations {
        let set: HashSet<&char> = different.into_iter().collect();
        // println!("Before dedup {:?}", set);
        // set.dedup();
        // println!("After dedup {:?}", set);
        if set.len() == 4 {
            break;
        }
        counter += 1;
        // let mut to_append = vec![c];
        // different.append(&mut to_append);
        different = &input[counter - 4..counter];
    }
    // println!("The code is: {:?}", different);
    // println!("Marker found after {}", counter);
    Some(counter)
}

pub fn part_two(input: &str) -> Option<usize> {
    let message_length: usize = 14;
    let mut counter: usize = message_length - 1;
    let input = input.chars().collect::<Vec<char>>();
    let mut different = &input[0..message_length];
    let iterations = input.len();
    for _ in message_length - 1..iterations {
        let set: HashSet<&char> = different.into_iter().collect();
        // println!("Before dedup {:?}", set);
        // set.dedup();
        // println!("After dedup {:?}", set);
        if set.len() == message_length {
            break;
        }
        counter += 1;
        // let mut to_append = vec![c];
        // different.append(&mut to_append);
        different = &input[counter - message_length..counter];
    }
    // println!("The code is: {:?}", different);
    // println!("Marker found after {}", counter);
    Some(counter)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}
