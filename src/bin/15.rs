use std::collections::HashSet;

use advent_of_code::helpers::sensor::{Position, Sensor};

fn get_sensors_and_beacons(input: &str) -> Vec<(Sensor, Position)> {
    let mut sensors_and_beacons: Vec<(Sensor, Position)> = vec![];
    let input: Vec<&str> = input.split("\n").collect();
    let wanted_input_len = input.len() - 1;
    for line in input.into_iter().take(wanted_input_len) {
        let l: Vec<&str> = line.split(" ").collect();
        let sensor_x: Vec<&str> = l[2].split(&['=', ','][..]).collect();
        let sensor_x: isize = sensor_x[1].parse().unwrap();
        let sensor_y: Vec<&str> = l[3].split(&['=', ',', ':'][..]).collect();
        let sensor_y: isize = sensor_y[1].parse().unwrap();
        let beacon_x: Vec<&str> = l[8].split(&['=', ','][..]).collect();
        let beacon_x: isize = beacon_x[1].parse().unwrap();
        let beacon_y: Vec<&str> = l[9].split(&['=', ',', ':'][..]).collect();
        let beacon_y: isize = beacon_y[1].parse().unwrap();
        let beacon = (beacon_x, beacon_y);
        sensors_and_beacons.push((Sensor::new((sensor_x, sensor_y), beacon), beacon));
    }
    sensors_and_beacons
}

fn calculate_tuning_frequency(position: Position) -> isize {
    position.0 * 4000000 + position.1
}

pub fn part_one(input: &str) -> Option<u32> {
    let sensors_and_beacons = get_sensors_and_beacons(input);
    // let wanted_y: isize = 10;
    let wanted_y: isize = 2000000;
    let mut found_in_row: Vec<isize> = vec![];
    for (s, _b) in sensors_and_beacons.clone().into_iter() {
        let mut to_append = s.get_all_in_row(wanted_y);
        found_in_row.append(&mut to_append);
    }
    let mut total_no_beacon: HashSet<isize> = found_in_row.into_iter().collect();
    for (_s, b) in sensors_and_beacons.into_iter() {
        if b.1 == wanted_y {
            total_no_beacon.remove(&b.1);
        }
    }

    Some(total_no_beacon.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sensors_and_beacons = get_sensors_and_beacons(input);
    let large_lim: isize = 4000000;
    // let wanted_y: isize = 10;
    // let wanted_y: isize = 2000000;
    let mut impossible_beacon_locs: Vec<Position> = vec![];
    for (s, _b) in sensors_and_beacons.clone().into_iter() {
        let mut to_append = s.get_all_within_radius();
        impossible_beacon_locs.append(&mut to_append);
    }
    let total_no_doubles: HashSet<Position> = impossible_beacon_locs.into_iter().collect();
    let mut range = 0;
    while range <= large_lim {
        for y in 0..=large_lim {
            for x in 0..=large_lim {
                if !total_no_doubles.contains(&(x, y)) {
                    return Some(calculate_tuning_frequency((x, y)) as u32);
                }
            }
        }
        range += 1;
    }
    // Some(total_no_beacon.len() as u32)
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), Some(56000011));
    }
}
