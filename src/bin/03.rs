advent_of_code::solution!(3);
use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct NumberString {
    name: String,
    x: Vec<usize>, // NumberStrings can comprise of many digits
    y: usize,      // But they are all on the same row to be a single string
}

struct Gear {
    x: usize,
    y: usize,
    // product: usize,
}

impl Gear {
    pub fn get_product(x: usize, y: usize, field: &Vec<String>) -> usize {
        let maybe_result = Self { x, y };
        let total_neighbors = maybe_result.get_numstring_neighbors(field);
        if total_neighbors.len() != 2 {
            return 0;
        }
        total_neighbors
            .into_iter()
            .map(|ns| ns.get_value())
            .product::<usize>()
    }

    pub fn get_numstring_neighbors(&self, field: &Vec<String>) -> Vec<NumberString> {
        let mut result = vec![];
        // Check first cross
        let check_down = self.y < field.len() - 1;
        let check_up = self.y > 0;
        // let mut total_neighbors: Vec<bool> = vec![];
        // let current_row = field[self.y].chars().collect::<Vec<>>();
        let current_row = field[self.y]
            .chars()
            .map(|c| c.is_numeric())
            .collect::<Vec<bool>>();
        let check_left = self.x > 0;
        let check_right = self.x < current_row.len() - 2;
        // First check this row
        if check_left {
            // total_neighbors.push(current_row[self.x[0] - 1]);
            if current_row[self.x - 1] {
                result.push(NumberString::from_row(
                    field[self.y].clone(),
                    self.x - 1,
                    self.y,
                ));
            }
        }
        if check_right {
            // total_neighbors.push(current_row[self.x[self.x.len() - 1] + 1]);
            if current_row[self.x + 1] {
                result.push(NumberString::from_row(
                    field[self.y].clone(),
                    self.x + 1,
                    self.y,
                ));
            }
        }
        if check_up {
            let upper_row = field[self.y - 1]
                .chars()
                .map(|c| c.is_numeric())
                .collect::<Vec<bool>>();
            if upper_row[self.x] {
                result.push(NumberString::from_row(
                    field[self.y - 1].clone(),
                    self.x,
                    self.y - 1,
                ));
            }
            if check_left {
                if upper_row[self.x - 1] {
                    result.push(NumberString::from_row(
                        field[self.y - 1].clone(),
                        self.x - 1,
                        self.y - 1,
                    ));
                }
            }
            if check_right {
                if upper_row[self.x + 1] {
                    result.push(NumberString::from_row(
                        field[self.y - 1].clone(),
                        self.x + 1,
                        self.y - 1,
                    ));
                }
            }
        }
        if check_down {
            let lower_row = field[self.y + 1]
                .chars()
                .map(|c| c.is_numeric())
                .collect::<Vec<bool>>();
            if lower_row[self.x] {
                result.push(NumberString::from_row(
                    field[self.y + 1].clone(),
                    self.x,
                    self.y + 1,
                ));
            }
            if check_left {
                if lower_row[self.x - 1] {
                    result.push(NumberString::from_row(
                        field[self.y + 1].clone(),
                        self.x - 1,
                        self.y + 1,
                    ));
                }
            }
            if check_right {
                if lower_row[self.x + 1] {
                    result.push(NumberString::from_row(
                        field[self.y + 1].clone(),
                        self.x + 1,
                        self.y + 1,
                    ));
                }
            }
        }
        // println!("There Neighbors were found so far {:?}", result);
        result.into_iter().unique().collect::<Vec<NumberString>>()
    }
}

impl NumberString {
    pub fn new(nums: String, first_x: usize, y: usize) -> Self {
        Self {
            name: nums.clone(),
            x: (first_x..first_x + nums.len()).collect(),
            y,
        }
    }

    pub fn from_row(row: String, one_x: usize, y: usize) -> Self {
        let left_part = row.split_at(one_x);
        let mut real_x = left_part
            .0
            .to_string()
            .rfind(|c: char| !c.is_numeric())
            .unwrap_or(usize::MAX);
        // It didn't find a Non-numeric character
        if real_x == usize::MAX {
            real_x = left_part
                .0
                .to_string()
                .find(char::is_numeric)
                // If it finds a number there was a number to find
                // Otherwise it was an empty string.
                .unwrap_or(one_x);
        } else {
            // It found a non-numberic character
            real_x += 1;
        }
        let wanted_string = row.split_at(real_x).1;
        let wanted_string = wanted_string
            .split(|c: char| !c.is_numeric())
            .collect::<Vec<&str>>()[0]
            .to_string();
        // let wanted_string
        Self::new(wanted_string, real_x, y)
    }

    pub fn get_value(&self) -> usize {
        // println!(
        //     "Getting value of {} as ({:?},{})",
        //     self.name, self.x, self.y
        // );
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

fn get_gears_products_from_row(row: String, row_index: usize, field: &Vec<String>) -> usize {
    row.chars()
        .into_iter()
        .enumerate()
        .filter(|(_i, c)| *c == '*')
        .map(|(x, _c)| Gear::get_product(x, row_index, field))
        .sum()
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
        let row = row.clone();
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
    let mut sum = 0;
    let lines = input
        .split("\n")
        .filter(|&l| l != "")
        .map(|l| l.to_string())
        .collect::<Vec<String>>();
    for (row_index, row) in lines.clone().into_iter().enumerate() {
        let row = row.clone();
        sum += get_gears_products_from_row(row, row_index, &lines);
    }

    Some(sum as u32)
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
