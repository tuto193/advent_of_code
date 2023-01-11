fn is_right_bigger_than_left(left: &str, right: &str) -> bool {
        let mut left = left.split(&['[', ','][..]).into_iter();
        let mut right = right.split(&['[', ','][..]).into_iter();
        let mut last_left = left.next();
        let mut last_right = right.next();
        'numbers: loop {
            // println!("Comparing L:{:?} and R:{:?}", last_left, last_right);
            if let Some(this_left) = last_left {
                // Left still has stuff, so compare
                match this_left {
                    "" => {
                        // Left is "[" so advance it
                        last_left = left.next();
                        if let Some(this_right) = last_right {
                            // right is standing on something
                            match this_right {
                                "" => last_right = right.next(),
                                right_number_par=> {
                                    let actual_right: &str = right_number_par
                                        .split("]")
                                        .into_iter().next().unwrap();
                                    match actual_right {
                                        "" => {
                                            // It was an empty list, so left is bigger
                                            return false;
                                        }
                                        num => {
                                            // It's standing on a number, so we can carry on passing it on
                                            last_right = Some(num);
                                        }
                                    }
                                }
                            }
                            continue 'numbers;
                        } else {
                            // Right ran out before left. Out of order!
                            return false;
                        }
                    }
                    // Left: on number or number+"]+"
                    left_num_par => {
                        let actual_left = left_num_par
                            .split("]")
                            .into_iter()
                            .next()
                            .unwrap();
                        match actual_left {
                            "" => {
                                // It's empty, so it was just an empty list in the end
                                if let Some(this_right) = last_right {
                                    let actual_right = this_right.split("]").into_iter().next().unwrap();
                                    match actual_right {
                                        "" => {
                                            // Right also doesn't go deeper
                                            // con
                                        }
                                    }
                                }
                            }
                        }
                        if let Some(this_right) = last_right {
                            match this_right{
                                "" => {
                                    last_right = right.next();
                                    continue 'numbers;
                                }
                                right_num => {
                                    let l_num: isize = left_num.parse().unwrap();
                                    let r_num: isize = right_num.parse().unwrap();
                                    if r_num == l_num {
                                        // they're equal
                                        last_left = left.next();
                                        last_right = right.next();
                                        continue 'numbers;
                                    } else {
                                        return r_num > l_num;
                                    }
                                }
                            }
                        } else {
                            // left had a number, but right had run out.
                            return false;
                        }
                    }
                }
            } else if let Some(_this_right) = last_right {
                // right has more, and left doesn't, so it's the right oder
                return true;
            } else {
                // Both ran out while still being "equal", so it's not the right oder
                return false;
            }
        }
}

fn get_right_orders_sum(signal: Vec<&str>) -> usize {
    let mut sum = 0;
    for (i, packet) in signal.into_iter().enumerate() {
        let l_r: Vec<&str> = packet.split("\n").collect();
        if is_right_bigger_than_left(l_r[0], l_r[1]) {
            // println!("Right order. Adding!");
            sum += i + 1;
        }
    }
    sum
}

pub fn part_one(input: &str) -> Option<usize> {
    let signal: Vec<&str> = input.split("\n\n").collect();
    let total_packets = signal.len() - 1;
    let signal: Vec<&str> = signal.into_iter().take(total_packets).collect();
    let sum_of_right_orders = get_right_orders_sum(signal);
    return Some(sum_of_right_orders);
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), None);
    }
}
