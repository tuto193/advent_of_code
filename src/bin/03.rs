advent_of_code::solution!(3);

struct NumberString {
    name: String,
    x: Vec<usize>, // NumberStrings can comprise of many digits
    y: usize,      // But they are all on the same row to be a single string
}

impl NumberString {
    pub fn new(nums: String, first_x: usize, y: usize) -> Self {
        Self {
            name: nums.clone(),
            x: (first_x..first_x + nums.len()).collect(),
            y,
        }
    }

    pub fn get_value(&self) -> usize {
        self.name.parse::<usize>().unwrap()
    }

    pub fn has_symbol_neighbor(&self, field: &Vec<String>) -> bool {
        // Check first cross
        let check_down = self.y < field.len() - 1;
        let check_up = self.y > 0;
        let mut total_neighbors: Vec<bool> = vec![];
        // let current_row = field[self.y].chars().collect::<Vec<>>();
        let current_row = field[self.y]
            .chars()
            .map(|c| !c.is_ascii_digit() && c.is_ascii_punctuation() && c != '.')
            .collect::<Vec<bool>>();
        let check_left = self.x[0] > 0;
        let check_right = *self.x.last().unwrap() < current_row.len() - 2;
        // First check this row
        if check_left {
            total_neighbors.push(current_row[self.x[0] - 1]);
        }
        if check_right {
            total_neighbors.push(current_row[self.x[self.x.len() - 1] + 1]);
        }
        if check_up {
            let upper_row = field[self.y - 1]
                .chars()
                .map(|c| !c.is_ascii_digit() && c.is_ascii_punctuation() && c != '.')
                .collect::<Vec<bool>>();
            total_neighbors.push(self.x.iter().any(|&x| upper_row[x]));
            if check_left {
                total_neighbors.push(upper_row[self.x[0] - 1]);
            }
            if check_right {
                total_neighbors.push(upper_row[self.x[self.x.len() - 1] + 1]);
            }
        }
        if check_down {
            let lower_row = field[self.y + 1]
                .chars()
                .map(|c| !c.is_ascii_digit() && c.is_ascii_punctuation() && c != '.')
                .collect::<Vec<bool>>();
            total_neighbors.push(self.x.iter().any(|&x| lower_row[x]));
            if check_left {
                total_neighbors.push(lower_row[self.x[0] - 1]);
            }
            if check_right {
                total_neighbors.push(lower_row[self.x[self.x.len() - 1] + 1]);
            }
        }
        total_neighbors.into_iter().any(|n| n)
    }
}

fn get_number_strings_from_row(row: String, row_index: usize) -> Vec<NumberString> {
    let mut result: Vec<NumberString> = vec![];
    let mut current_number_string: String = "".to_string();
    let mut first_number = 0;
    for (x, c) in row.clone().chars().enumerate() {
        if c.is_numeric() {
            // Adding the first number
            if current_number_string == "" {
                first_number = x;
            }
            current_number_string += &c.to_string();
            continue;
        }
        if current_number_string != "" {
            // The number is now complete but the row keeps going
            result.push(NumberString::new(
                current_number_string,
                first_number,
                row_index,
            ));
        }
        current_number_string = "".to_string();
    }
    // In  case the number went up until the end of the row
    if current_number_string != "" {
        result.push(NumberString::new(
            current_number_string,
            first_number,
            row_index,
        ));
    }
    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input
        .split('\n')
        .filter(|&l| l != "") // Strip the last line
        .map(|l| l.to_string())
        .collect::<Vec<String>>();
    let mut sum = 0;
    for (row_index, row) in lines.clone().into_iter().enumerate() {
        // let mut current_x =;
        let mut row = row.clone();
        let numbers_with_neighbors = get_number_strings_from_row(row, row_index)
            .into_iter()
            .filter(|ns| ns.has_symbol_neighbor(&lines))
            .map(|ns| ns.get_value())
            .collect::<Vec<usize>>();
        sum += numbers_with_neighbors.iter().sum::<usize>();
    }
    Some(sum as u32)
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
