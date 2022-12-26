use std::collections::HashSet;

use crate::get_file_contents;

pub fn day_03() {
    let input = get_file_contents("03".to_owned());
    let mut sum_priorities: usize = 0;
    for elf in input.split("\n").into_iter() {
        if elf == "" {
            continue;
        }
        let bag_size = elf.len() / 2;
        let both_in_bytes = elf.as_bytes();
        let bag_1: HashSet<&u8> = both_in_bytes[.. bag_size]
            .into_iter()
            .collect();
        let bag_2: HashSet<&u8> = both_in_bytes[bag_size .. ]
            .into_iter()
            .collect();
        let common_items = bag_1.intersection(&bag_2);
        // let common_item = common_items
        //     .into_iter()
        //     .collect::<Vec<&&u8>>()[0];
        // let subtractor = if **common_item > 90 { 96 } else { 38 };
        // sum_priorities += (**common_item as usize) - subtractor;
        for item in common_items.into_iter() {
            // println!("Our letter is: {} with value of", item );
            let subtractor = if **item > 90 { 96 } else { 38 };
            sum_priorities += (**item as usize) - subtractor;
        }
    }
    // let a = "a".as_bytes();
    // println!("Letter 'a' to digit: {:?}", a[0] - 96u8)
    println!("The total sum of priorities is: {}", {sum_priorities})
}

pub fn day_03_part2() {
    let input = get_file_contents("03".to_owned());
    let mut sum_priorities: usize = 0;
    let elves = input.split("\n").collect::<Vec<&str>>();
    let iterations = elves.len() / 3;
    let mut sum_priorities: usize = 0;
    for i in 0..iterations {
        let elf1: HashSet<&u8> = elves[i * 3].as_bytes().into_iter().collect();
        let elf2: HashSet<&u8> = elves[(i * 3) + 1].as_bytes().into_iter().collect();
        let elf3: HashSet<&u8> = elves[(i * 3) + 2].as_bytes().into_iter().collect();
        let badge_type1 = elf1.intersection(&elf2).collect::<HashSet<&&u8>>();
        let badge_type2 = elf1.intersection(&elf3).collect::<HashSet<&&u8>>();
        let absolut_type = badge_type1.intersection(&badge_type2);
        for item in absolut_type.into_iter() {
            let subtractor = if ***item > 90 { 96 } else { 38 };
            sum_priorities += (***item as usize) - subtractor;
        }
    }
    println!("The total sum of badge priorities is: {}", sum_priorities);
    // for elf_group in input.split("\n").into_iter() {
    //     print!("This is the first line:\n {}", elf_group);
    //     return
    // }
}
