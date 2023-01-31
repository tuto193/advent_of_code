use std::collections::{HashSet, HashMap};
use itertools::Itertools;
use advent_of_code::helpers::sensor::{Point, Sensor, Line, Pair};

fn get_pairs(input: &str) -> Vec<Pair> {
    let mut sensors_and_beacons: Vec<Pair> = vec![];
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
        sensors_and_beacons.push(
           Pair{
                sensor: Point{x: sensor_x, y: sensor_y},
                beacon: Point{x: beacon.0, y: beacon.1}
            }
        );
    }
    sensors_and_beacons
}

fn get_sensors_and_beacons(input: &str) -> Vec<(Sensor, Point)> {
    let mut sensors_and_beacons: Vec<(Sensor, Point)> = vec![];
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
        sensors_and_beacons.push(
            (
                Sensor::new(
                    Point{x: sensor_x, y: sensor_y},
                    Point{x: beacon.0, y: beacon.1}
                ), Point{x: beacon.0, y: beacon.1}
            )
        );
    }
    sensors_and_beacons
}

fn fold_compatible_overlaps(
    parallel: &HashMap<isize, Vec<Line>>,
) -> impl Fn(Vec<Line>, &isize, Line) -> Vec<Line> + '_ {
    |mut overlaps, y_intercept, line| {
        overlaps.extend(
            parallel
                .get(y_intercept)
                .iter()
                .flat_map(|v| v.iter())
                .filter_map(|other| line.overlap(other)),
        );
        overlaps
    }
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
        if b.y == wanted_y {
            total_no_beacon.remove(&b.y);
        }
    }

    Some(total_no_beacon.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let pairs = get_pairs(input);
    let large_lim: isize = 4_000_000;
    // let large_lim: isize = 20;
    let top_right = pairs
        .iter()
        .map(Pair::top_right)
        .into_group_map_by(Line::y_intercept);
    let positive_slope_overlaps = pairs
        .iter()
        .map(Pair::bottom_left)
        .into_grouping_map_by(Line::y_intercept)
        .fold(vec![], fold_compatible_overlaps(&top_right));
    let top_left = pairs
        .iter()
        .map(Pair::top_left)
        .into_group_map_by(Line::y_intercept);
    let negative_slope_overlaps = pairs
        .iter()
        .map(Pair::bottom_right)
        .into_grouping_map_by(Line::y_intercept)
        .fold(vec![], fold_compatible_overlaps(&top_left));
    let Point { x, y } = positive_slope_overlaps
        .values()
        .flatten()
        .cartesian_product(negative_slope_overlaps.values().flatten())
        .find_map(|(positive, negative)| {
            positive.interception(negative).filter(|p| {
                p.x >= 0
                    && p.x <= large_lim
                    && p.y >= 0
                    && p.y <= large_lim
                    && pairs.iter().all(|pair| !pair.covers(p))
            })
        })
        .expect("should be an interception");

    Some((x * 4_000_000 + y) as u32)
    // None
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
