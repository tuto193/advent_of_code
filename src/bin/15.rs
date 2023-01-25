type Level = Vec<Vec<Tile>>;

type Position = (isize, isize);

#[derive(Clone, Copy, Debug)]
enum Tile {
    Beacon,
    NoBeacon,
    NoIdea,
}

fn get_boundaries_and_centre(sensors_and_beacons: &Vec<(Position, Position)>) -> (usize, usize, Position) {
    let mut min_x: isize = 0;
    let mut min_y: isize = 0;
    let mut max_x: isize = 0;
    let mut max_y: isize = 0;
    for (s, b)in sensors_and_beacons.into_iter() {
        min_x = min_x.min(s.0).min(b.0);
        max_x = max_x.min(s.0).min(b.0);
        min_y = min_y.min(s.1).min(b.1);
        max_y = max_y.min(s.1).min(b.1);
    }
    (
        (min_x.abs() + max_x.abs()) as usize,
        (min_y.abs() + max_y.abs()) as usize,
        (min_x.abs(), min_y.abs())
    )
}

fn init_level(width: usize, height: usize) -> Level {
    let level = vec![vec![Tile::NoIdea; width]; height];
    level
}

fn get_sensors_and_beacons(input: &str) -> Vec<(Position, Position)>{
    let mut sensors_and_beacons: Vec<(Position, Position)> = vec![];
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
        sensors_and_beacons.push(
            (
                (sensor_x, sensor_y),
                (beacon_x, beacon_y)
            )
        );
    }
    sensors_and_beacons
}

fn parse_level_with_offset(level: &mut Level, sensors_and_beacons: &Vec<(Position, Position)>, offset: (isize, isize)) {

}

pub fn part_one(input: &str) -> Option<u32> {
    let sensors_and_beacons = get_sensors_and_beacons(input);
    let boundaries = get_boundaries_and_centre(&sensors_and_beacons);
    let mut level = init_level(boundaries.0, boundaries.1);
    let offset = boundaries.2;
    parse_level_with_offset(&mut level, &sensors_and_beacons, offset);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
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
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), None);
    }
}
