fn is_right_bigger_than_left(left: &str, right: &str) -> bool {
        let mut left = left.split(&['[', ',', ']'][..]).into_iter();
        let mut right = right.split(&['[', ',', ']'][..]).into_iter();
        let mut last_left = left.next();
        let mut last_right = right.next();
        'numbers: loop {
            if let Some(left) = last_left {
                break 'numbers;
            }
        }
    false
}

fn get_right_orders_sum(signal: Vec<&str>) -> usize {
    let mut sum = 0;
    for (i, packet) in signal.into_iter().enumerate() {
        let l_r: Vec<&str> = packet.split("\n").collect();
        if is_right_bigger_than_left(l_r[0], l_r[1]) {
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
