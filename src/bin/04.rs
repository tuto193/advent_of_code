pub fn part_one(input: &str) -> Option<u32> {
    let mut total_contained: u32 = 0;
    for elf_pair in input.split("\n").into_iter() {
        if elf_pair == "" {
            continue;
        }
        let elves = elf_pair.split(",").collect::<Vec<&str>>();
        if elves[0] == "" || elves[1] == "" {
            print!("Pair {} wasn't empty but after split it now is.", elf_pair);
            continue;
        }
        let elf_1 = elves[0].split("-").collect::<Vec<&str>>();
        let elf_2 = elves[1].split("-").collect::<Vec<&str>>();
        if elf_1.len() < 2 || elf_2.len() < 2 {
            print!(
                "Pair {:?} and {:?} wasn't empty but after split it now is.",
                elf_1, elf_2
            );
            continue;
        }
        // parse into numbers
        let elf_1 = elf_1
            .into_iter()
            .map(|n| n.parse().unwrap())
            .collect::<Vec<usize>>();
        let elf_2 = elf_2
            .into_iter()
            .map(|n| n.parse().unwrap())
            .collect::<Vec<usize>>();
        // 2...1...|
        if elf_1[0] >= elf_2[0] {
            // |...1...2
            if elf_1[1] <= elf_2[1] {
                total_contained += 1;
                continue;
            }
        }
        if elf_1[0] <= elf_2[0] {
            if elf_1[1] >= elf_2[1] {
                total_contained += 1;
                continue;
            }
        }
    }
    // println!("Total self-contained pairs: {}", total_contained);
    Some(total_contained)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total_contained: u32 = 0;
    for elf_pair in input.split("\n").into_iter() {
        if elf_pair == "" {
            continue;
        }
        let elves = elf_pair.split(",").collect::<Vec<&str>>();
        if elves[0] == "" || elves[1] == "" {
            print!("Pair {} wasn't empty but after split it now is.", elf_pair);
            continue;
        }
        let elf_1 = elves[0].split("-").collect::<Vec<&str>>();
        let elf_2 = elves[1].split("-").collect::<Vec<&str>>();
        if elf_1.len() < 2 || elf_2.len() < 2 {
            print!(
                "Pair {:?} and {:?} wasn't empty but after split it now is.",
                elf_1, elf_2
            );
            continue;
        }
        // parse into numbers
        let elf_1 = elf_1
            .into_iter()
            .map(|n| n.parse().unwrap())
            .collect::<Vec<usize>>();
        let elf_2 = elf_2
            .into_iter()
            .map(|n| n.parse().unwrap())
            .collect::<Vec<usize>>();

        // first side 1
        if elf_1[0] >= elf_2[0] && elf_1[0] <= elf_2[1] {
            total_contained += 1;
            continue;
        }
        // first side 2
        if elf_1[1] >= elf_2[0] && elf_1[1] <= elf_2[1] {
            total_contained += 1;
            continue;
        }
        // second side 1
        if elf_2[0] >= elf_1[0] && elf_2[0] <= elf_1[1] {
            total_contained += 1;
            continue;
        }
        // second side 2
        if elf_2[1] >= elf_1[0] && elf_2[1] <= elf_1[1] {
            total_contained += 1;
            continue;
        }
    }
    // println!("Total self-contained pairs: {}", total_contained);
    Some(total_contained)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), None);
    }
}
