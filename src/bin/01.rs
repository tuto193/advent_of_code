pub fn part_one(input: &str) -> Option<u32> {
    let elves_vector: Vec<&str> = input.split("\n\n").collect();

    let mut max = 0;
    for elf in elves_vector.into_iter() {
        let elf_string: Vec<&str> = elf.split("\n").collect();
        let mut sum = 0;
        for line in elf_string.into_iter() {
            if line != "" {
                sum += line.parse::<u32>().unwrap();
            }
        }
        if sum > max {
            max = sum;
        }
    }
    Some(max)
}

pub fn part_two(input: &str) -> Option<u32> {
    let elves_vector: Vec<&str> = input.split("\n\n").collect();
    let total_elves = elves_vector.len() - 1;

    // let mut max = 0;
    let mut elves_cals_vector: Vec<u32> = elves_vector
        .into_iter()
        .take(total_elves)
        .map(|elf| {
            elf.split("\n")
                .into_iter()
                .map(|cals| cals.parse::<u32>().unwrap())
                .sum()
        })
        .collect();
    elves_cals_vector.sort();

    Some(elves_cals_vector.into_iter().rev().take(3).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
