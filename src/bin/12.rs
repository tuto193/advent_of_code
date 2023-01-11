type Position = (usize, usize);

use advent_of_code::helpers::path_finding::bfs::bfs_with_coords;

pub fn part_one(input: &str) -> Option<usize> {
    let input: Vec<&str> = input.split("\n").collect();
    // let row_length = input[0].len();
    let height = input.len() - 1; // Because of last empty line
                                  // let total_nodes = row_length * height;
    let input: Vec<Vec<char>> = input
        .into_iter()
        .take(height)
        .map(|r| r.chars().into_iter().collect())
        .collect();
    let mut start: Position = (0, 0);
    let mut goal: Position = (0, 0);
    let maze: Vec<Vec<u32>> = input
        .into_iter()
        .enumerate()
        .map(|(y, row)| {
            row.into_iter()
                .enumerate()
                .map(|(x, letter)| match letter {
                    'S' => {
                        start = (x, y);
                        'a' as u32 - 1
                    }
                    'E' => {
                        goal = (x, y);
                        'z' as u32
                    }
                    l => l as u32,
                })
                .collect()
        })
        .collect();
    if let Some(path) = bfs_with_coords(start, goal, &maze, false) {
        return Some(path.len() - 1);
    }
    None
}

pub fn part_two(input: &str) -> Option<usize> {
    let input_: Vec<&str> = input.split("\n").collect();
    // let row_length = input[0].len();
    let height = input_.len() - 1; // Because of last empty line
                                   // let total_nodes = row_length * height;
    let input_: Vec<Vec<char>> = input_
        .into_iter()
        .take(height)
        .map(|r| r.chars().into_iter().collect())
        .collect();
    let mut goal: Position = (0, 0);
    let maze: Vec<Vec<u32>> = input_
        .into_iter()
        .enumerate()
        .map(|(y, row)| {
            row.into_iter()
                .enumerate()
                .map(|(x, letter)| match letter {
                    'S' => 'a' as u32,
                    'E' => {
                        goal = (x, y);
                        'z' as u32
                    }
                    'a' => 'a' as u32,
                    l => l as u32,
                })
                .collect()
        })
        .collect();
    if let Some(path) = bfs_with_coords(goal, (0, 0), &maze, true) {
        return Some(path.len() - 1);
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
