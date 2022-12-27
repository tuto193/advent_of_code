use crate::get_file_contents;


pub fn day_04() {
    let input = get_file_contents("04".to_owned());
    let mut total_contained: usize = 0;
    for elf_pair in input.split("\n").into_iter() {
        if elf_pair == "" {
            continue
        }
        let elves = elf_pair.split(",").collect::<Vec<&str>>();
        if elves[0] == "" || elves[1] == "" {
            print!("Pair {} wasn't empty but after split it now is.", elf_pair);
            continue
        }
        let elf_1 = elves[0].split("-").collect::<Vec<&str>>();
        let elf_2 = elves[1].split("-").collect::<Vec<&str>>();
        if elf_1.len() < 2 || elf_2.len() < 2 {
            print!("Pair {:?} and {:?} wasn't empty but after split it now is.", elf_1, elf_2);
            continue
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
    println!("Total self-contained pairs: {}", total_contained);
}

pub fn day_04_part2() {
    let input = get_file_contents("04".to_owned());
    let mut total_contained: usize = 0;
    for elf_pair in input.split("\n").into_iter() {
        if elf_pair == "" {
            continue
        }
        let elves = elf_pair.split(",").collect::<Vec<&str>>();
        if elves[0] == "" || elves[1] == "" {
            print!("Pair {} wasn't empty but after split it now is.", elf_pair);
            continue
        }
        let elf_1 = elves[0].split("-").collect::<Vec<&str>>();
        let elf_2 = elves[1].split("-").collect::<Vec<&str>>();
        if elf_1.len() < 2 || elf_2.len() < 2 {
            print!("Pair {:?} and {:?} wasn't empty but after split it now is.", elf_1, elf_2);
            continue
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
    println!("Total self-contained pairs: {}", total_contained);
}
