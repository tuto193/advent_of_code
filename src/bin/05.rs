advent_of_code::solution!(5);

struct MapRange {
    dst_start: i64,
    src_start: i64,
    range: i64,
}

impl MapRange {
    pub fn new(dst_start: i64, src_start: i64, range: i64) -> Self {
        Self {
            dst_start,
            src_start,
            range: range - 1,
        }
    }

    pub fn is_in_range(&self, x: i64) -> bool {
        x >= self.src_start && x < self.src_start + self.range
    }

    pub fn get_dst(&self, x: i64) -> i64 {
        let offset = self.dst_start - self.src_start;
        x - offset
    }
}

fn get_map_from_section(lines: &Vec<&String>) -> Vec<MapRange> {
    let mut result = vec![];
    for line in lines.iter() {
        let numbers: Vec<i64> = line
            .split_ascii_whitespace()
            .into_iter()
            .map(|ns| ns.parse::<i64>().unwrap())
            .collect();
        result.push(MapRange::new(numbers[0], numbers[1], numbers[2]));
    }
    result
}

fn map_seed_to_location(seed: i64, map_of_maps: &Vec<Vec<MapRange>>) -> i64 {
    let mut start = seed;
    let mut end = -1;
    for map in map_of_maps.into_iter() {
        let containing_ranges = map
            .iter()
            .filter(|&mr| mr.is_in_range(start))
            .collect::<Vec<&MapRange>>();
        if containing_ranges.iter().count() == 1 {
            // There should be exactly one or none containing it
            end = containing_ranges[0].get_dst(start);
        } else {
            end = start;
        }
        println!("Mapping {} to {}", start, end);
        start = end;
    }
    end
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input
        .split("\n")
        .map(|l| l.to_string())
        .collect::<Vec<String>>();
    let mut map_of_maps: Vec<Vec<MapRange>> = vec![];
    let seeds: Vec<i64> = lines[0]
        .split_ascii_whitespace()
        .into_iter()
        .skip(1)
        .map(|ns| ns.parse::<i64>().unwrap())
        .collect();

    for (index, line) in lines.iter().skip(1).enumerate() {
        if line == "" {
            continue;
        }
        if line.ends_with("map:") {
            let lines_with_numbers: Vec<&String> = lines
                .iter()
                .skip(index + 2)
                .take_while(|&l| l != "")
                .collect();
            let map_of_section = get_map_from_section(&lines_with_numbers);
            map_of_maps.push(map_of_section);
        }
    }

    let destinations: Vec<i64> = seeds
        .into_iter()
        .map(|s| map_seed_to_location(s, &map_of_maps))
        .collect();
    Some(destinations.into_iter().min().unwrap() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
