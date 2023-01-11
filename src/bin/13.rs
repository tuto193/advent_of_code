type PacketEntry = (Option<isize>, (isize, isize));

fn parse_packet(packet: &str) -> Vec<PacketEntry> {
    let mut to_return: Vec<(Option<isize>, (isize, isize))> = vec![];
    let mut current_depth: isize = 0;
    let mut last_depth: isize = current_depth;
    let mut packet = packet.split(&['[', ','][..]).into_iter();
    let mut last_entry = packet.next();
    'commas: loop {
        if let Some(this_entry) = last_entry {
            match this_entry {
                "" => current_depth += 1,
                number_and_close => {
                    let mut num_n_cl_iter = number_and_close.split("]").into_iter();
                    let to_add: Option<isize> = match num_n_cl_iter.next().unwrap() {
                        "" => None, // it was a ]
                        x => Some(x.parse().unwrap()),
                    };
                    to_return.push((to_add, (last_depth, current_depth)));
                    while let Some(_close) = num_n_cl_iter.next() {
                        current_depth -= 1;
                    }
                    // Until the comma
                    last_depth = current_depth;
                }
            }
            last_entry = packet.next();
        } else {
            break 'commas;
        }
    }
    to_return
}

fn compare_entries(entry_l: PacketEntry, entry_r: PacketEntry, last_max_depth: isize) -> (Option<bool>, isize) {
    let (try_left_number, (left_prev_depth, left_depth)) = entry_l;
    let (try_right_number, (right_prev_depth, right_depth)) = entry_r;
    // Check if they moved relative to the previous max depth
    let height_diff_right= right_prev_depth - last_max_depth;
    let height_diff_left = left_prev_depth - last_max_depth;
    if height_diff_left >= 0 {
        // Left is going up
        if height_diff_right >= 0 {
            // right is also going up
            // We can then compare normally
            // DO nothing here, and compare them properlly below
        } else {
            // right sis going down
            // Right ran out fore left
            return (Some(false), 0)
        }
    } else {
        // left is going down
        if height_diff_right >= 0 {
            // Right is going up
            // Left ran out before right
            return (Some(true), 0)
        } else {
            // Right is also going down
            if height_diff_left != height_diff_right {
                // One of them ran out of stuff somewhere down the road
                return (Some(height_diff_right > height_diff_left), 0)
            }
            // If they went down the same, they are at the same height, so we compare them normally below
        }
    }
    // If we are here, it's because they both are going up, or went down he same amount
    let current_max_depth = left_depth.max(right_depth);
    if let Some(left_number) = try_left_number {
        // Left is a number and NOT an emty list
        if let Some(right_number) = try_right_number {
            // Right is also a number an not an empty list
            if left_number == right_number {
                return (None, current_max_depth);
            } else {
                return (Some(right_number > left_number), 0);
            }
        } else {
            // Left had a number and right just had an empty list/entry
            return (Some(false), 0);
        }
    } else if try_right_number.is_some() {
        // Right had a number, and left side was empty list/entry
        return (Some(true), 0);
    } else {
        if left_depth == right_depth {
            // Both empty lists were the same depth
            return (None, current_max_depth);
        } else {
            return (Some(right_depth > left_depth), 0);
        }
    }
}

fn are_packets_in_order(left: &str, right: &str) -> bool {
    let left = parse_packet(left);
    let right = parse_packet(right);
    let mut last_max_depth: isize = 0;
    let mut left_iterator = left.into_iter();
    let mut right_iterator = right.into_iter();
    loop {
        let left_entry = left_iterator.next();
        let right_entry = right_iterator.next();
        if let Some(left_remains) = left_entry {
            if let Some(right_remains) = right_entry {
                let (right_is_bigger, new_current_depth) = compare_entries(
                    left_remains,
                    right_remains,
                    last_max_depth,
                );
                if let Some(to_return) = right_is_bigger {
                    return to_return;
                } else {
                    last_max_depth = new_current_depth;
                }
            } else {
                // Right ran out before left
                return false;
            }
        } else if right_entry.is_some() {
            // Left ran out before right
            return true;
        } else {
            // Both ran out of entries. This shouldn't happen!
            panic!("Error: Both sides ran out of entries.");
        }
    }
}

fn get_right_orders_sum(signal: Vec<&str>) -> isize {
    let mut sum = 0;
    for (i, packet) in signal.into_iter().enumerate() {
        let l_r: Vec<&str> = packet.split("\n").collect();
        if are_packets_in_order(l_r[0], l_r[1]) {
            // println!("Right order. Adding!");
            sum += i + 1;
        }
    }
    sum as isize
}

pub fn part_one(input: &str) -> Option<isize> {
    let signal: Vec<&str> = input.split("\n\n").collect();
    let total_packets = signal.len() - 1;
    let signal: Vec<&str> = signal.into_iter().take(total_packets).collect();
    let sum_of_right_orders = get_right_orders_sum(signal);
    return Some(sum_of_right_orders);
}

pub fn part_two(input: &str) -> Option<isize> {
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
