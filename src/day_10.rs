use std::{time::Duration, thread::{sleep, self}};

use crate::get_file_contents;

fn is_cycle_important(cycle: usize) -> bool {
    (i32::try_from(cycle).unwrap() - 20) % 40 == 0
}

fn calculate_signal_strength(x: i32, cycle: usize) -> i32 {
    println!("{} * {}", cycle, x);
    x * (i32::try_from(cycle).unwrap())
}

fn advance_cycle(cycle: &mut usize, times: usize, x: &mut i32, add_x: i32) -> (bool, i32) {
    let mut calculated_important = 0;
    let mut was_important = false;
    for _ in 0..times {
        // println!("Cycle now is {}", *cycle);
        *cycle += 1;
        if is_cycle_important(*cycle) {
            println!("Cycle {} is important!", *cycle);
            was_important = true;
            calculated_important = calculate_signal_strength(*x, *cycle);
        }
    }
    println!("Going to add {}", add_x);
    *x += add_x;
    (was_important, calculated_important)
}

pub fn day_10() {
    // let input = get_file_contents("10".to_string());
    let input = get_file_contents("10".to_string());
    let mut x = 1;
    let mut signal_strengths: Vec<i32> = vec![];
    let last_important_cycle: usize = 220;
    let mut current_cycle: usize = 0;
    // 'cycling: loop {

    for instruction in input.split("\n").into_iter() {
        let instruction: Vec<&str> = instruction.split(" ").collect();
        println!("Instruction is: {:?}", instruction);
        let (times, add_x): (usize, i32) = if instruction.len() == 1 {
            (1, 0)
        } else {
            (2, instruction[1].parse().unwrap())
        };
        let (found_important, important_cycle_strength) =
            advance_cycle(&mut current_cycle, times, &mut x, add_x);
        if found_important {
            signal_strengths.push(important_cycle_strength);
        }
        if current_cycle > last_important_cycle {
            break;
            // break 'cycling;
        }
    }
    // }
    let sum_signal_strengths: i32 = signal_strengths.iter().sum();
    println!("The signal strengths are {:?}", signal_strengths);
    println!("\tand their sum is {}", sum_signal_strengths);
}

fn is_end_of_line(cycle: usize) -> bool {
    cycle != 0 && cycle % 40 == 0
}

fn add1_wrap_around(cycle: usize) -> usize {
    (cycle % 40) + 1
}

fn get_drawn_pixels_from_cycle(cycle: &mut usize, times: usize, x: &mut i32, add_x: i32) {
    // let mut which_cycles: Vec<usize> = vec![];
    let pixel_positions: Vec<i32> = ((*x - 1).max(0).min(39)..(*x + 2).max(0).min(39)).collect();
    let pixel_positions: Vec<usize> = pixel_positions
        .into_iter()
        .map(|int| int.try_into().unwrap())
        .collect();
    for _ in 0..times {
        *cycle = add1_wrap_around(*cycle);
        // print!("On cycle {}\t", *cycle);
        // println!("Pixel positions are {:?}", pixel_positions);
        // let duration = Duration::from_millis(30);
        // sleep(duration);
        let to_print = ["·", "#"][usize::try_from(pixel_positions.contains(&(*cycle - 1))).unwrap()];
        // Register is within drawing range
        // which_cycles.push(*cycle);
        // }
        print!("{}", to_print);
        // println!("{}", to_print);
        if is_end_of_line(*cycle) {
            println!();
        }
    }
    // println!("Going to add {}", add_x);
    *x += add_x;
    // which_cycles
}

// fn draw_found_cycles_pretty(cycle_indices: &Vec<usize>) {
//     let last_index: usize = 240;
//     for index in 0..last_index {
//         if cycle_indices.contains(&index) {
//             print!("#");
//         } else {
//             print!("·");
//         }
//         if index + 1 % 40 == 0 {
//             println!();
//         }
//     }
// }

pub fn day_10_part2() {
    let input = get_file_contents("10".to_string());
    let mut x = 1;
    // let mut cycle_indices_to_draw: Vec<usize> = vec![];
    let mut current_cycle: usize = 0;
    let last_important_cycle = 240;
    // 'cycling: loop {

    for instruction in input.split("\n").into_iter() {
        let instruction: Vec<&str> = instruction.split(" ").collect();
        // println!("Instruction is: {:?}", instruction);
        let (times, add_x): (usize, i32) = if instruction.len() == 1 {
            (1, 0)
        } else {
            (2, instruction[1].parse().unwrap())
        };
        // let mut more_cycles_to_draw =
        get_drawn_pixels_from_cycle(&mut current_cycle, times, &mut x, add_x);
        // signal_strengths.push(important_cycle_strength);
        // cycle_indices_to_draw.append(&mut more_cycles_to_draw);
        if current_cycle > last_important_cycle {
            break;
        }
    }
    println!();
    // draw_found_cycles_pretty(&cycle_indices_to_draw);
}
