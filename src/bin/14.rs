fn init_level(height: u32, width: u32) -> Vec<Vec<bool>> {
    let mut to_ret = vec![];
    for _ in 0..height {
        let mut to_append = vec![];
        for _ in 0..width {
            to_append.push(false);
        }
        to_ret.push(to_append);
    }
    to_ret
}

fn draw_wall(level: &mut Vec<Vec<bool>>, lines: &str) {
    let coords_list: Vec<(u32, u32)> = lines
        .split(" -> ")
        .into_iter()
        .map(|l| {
            l
                .split(",")


        })
}

fn parse_level(input: &str) -> Vec<Vec<bool>> {
    let mut level = init_level(1000, 1000);
    for wall_lines in input.split("\n") {
        draw_wall(&mut level, wall_lines);
    }
    level
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut level = init_level(1000, 1000);

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), None);
    }
}
