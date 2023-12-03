advent_of_code::solution!(2);
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut sum = 0;
    let max_cubes: Vec<usize> = vec![12, 13, 14];
    let mut cube_colors: HashMap<String, usize> = HashMap::new();
    cube_colors.insert("red".to_string(), 0);
    cube_colors.insert("green".to_string(), 1);
    cube_colors.insert("blue".to_string(), 2);
    'game: for (i, game) in lines.into_iter().enumerate() {
        if game == "" {
            continue;
        }
        // print!("On game {index}: ", index = i + 1);
        // let mut available_cubes = max_cubes.clone();
        let spl_line = game.split(": ").collect::<Vec<&str>>();
        // let index = spl_line[0]
        //     .split_ascii_whitespace()
        //     .last()
        //     .unwrap()
        //     .chars()
        //     .last()
        //     .unwrap()
        //     .to_digit(10)
        //     .unwrap();
        let sets = spl_line[1].split("; ").collect::<Vec<&str>>();
        // println!("Sets are {s:?}", s = sets);
        // println!("\tCubes before (rgb): {c:?}", c = available_cubes);
        for (set_i, set) in sets.into_iter().enumerate() {
            let mut available_cubes = max_cubes.clone();
            let cubes = set.split(", ").collect::<Vec<&str>>();
            for cube in cubes {
                let split_cube = cube.split_ascii_whitespace().collect::<Vec<&str>>();
                let how_many = split_cube[0].to_string().parse::<usize>().unwrap();
                let color_index = *cube_colors.get(split_cube[1]).unwrap();
                if available_cubes[color_index] < how_many {
                    // Game wasn't possible
                    continue 'game;
                }
                available_cubes[color_index] -= how_many;
            }
            // println!(
            //     "\tCubes after set {set_index} (rgb): {c:?}",
            //     c = available_cubes,
            //     set_index = set_i
            // );
        }
        // println!("  Set {ind} was possible!\n", ind = i + 1);
        // print!("Sum : {s} -> ", s = sum);
        sum += (i as u32) + 1;
        // println!("{s}", s = sum);
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut sum = 0;
    let mut cube_colors: HashMap<String, usize> = HashMap::new();
    cube_colors.insert("red".to_string(), 0);
    cube_colors.insert("green".to_string(), 1);
    cube_colors.insert("blue".to_string(), 2);
    for (i, game) in lines.into_iter().enumerate() {
        if game == "" {
            continue;
        }
        let spl_line = game.split(": ").collect::<Vec<&str>>();
        let sets = spl_line[1].split("; ").collect::<Vec<&str>>();
        let mut minimum_number_of_cubes = vec![0, 0, 0];
        for (set_i, set) in sets.into_iter().enumerate() {
            let cubes = set.split(", ").collect::<Vec<&str>>();
            for cube in cubes {
                let split_cube = cube.split_ascii_whitespace().collect::<Vec<&str>>();
                let how_many = split_cube[0].to_string().parse::<usize>().unwrap();
                let color_index = *cube_colors.get(split_cube[1]).unwrap();
                if minimum_number_of_cubes[color_index] < how_many {
                    minimum_number_of_cubes[color_index] = how_many;
                }
            }
            // minimum_number_of_cubes = available_cubes.clone();
        }
        sum += minimum_number_of_cubes.iter().product::<usize>() as u32;
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
