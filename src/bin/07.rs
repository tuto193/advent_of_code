use advent_of_code::helpers::directory::*;

pub fn part_one(input: &str) -> Option<usize> {
    let root = get_directories_from_input(input.to_string());
    let max_size: usize = 100000;
    // root.borrow_mut().print_hierarchy(1);
    let total_sum = root.borrow_mut().get_all_under_limit(max_size);
    // println!("The total sum of all under {} is: {}", max_size, total_sum);
    Some(total_sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let root = get_directories_from_input(input.to_string());
    let total_space: i32 = 70000000;
    let current_empty_space = total_space - (root.borrow().total_size() as i32);
    let needed_empty_space = 30000000 - current_empty_space;
    let all_over_size = root.borrow().get_all_over_size(needed_empty_space as usize);
    let smallest = all_over_size.into_iter().min().unwrap();
    // println!("The smallest size found was '{}'", smallest);
    Some(smallest)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
