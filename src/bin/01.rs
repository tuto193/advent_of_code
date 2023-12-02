advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<String> = input.split("\n").map(|e| e.to_string()).collect();
    let mut sum: u32 = 0;
    for line in lines {
        if line == "" {
            continue;
        }
        let numbers: Vec<u32> = line
            .chars()
            .filter(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        let to_add = (numbers[0] * 10) + (numbers[numbers.len() - 1]);
        sum += to_add;
    }
    Some(sum)
}

fn parse_number_from_line(line: String) -> u32 {
    let numbers = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let line = line.to_lowercase();
    let first_digit_index = line.find(char::is_numeric).unwrap_or(9999);

    let last_digit_index = line.rfind(char::is_numeric).unwrap_or(0);
    let mut first_written_index = 9999;
    let mut first_written_digit: u32 = 9;
    let mut last_written_index = 0;
    let mut last_written_digit = 9;
    for (i, &n) in numbers.iter().enumerate() {
        let possible_first = line.find(n).unwrap_or(9999);
        let possible_last = line.rfind(n).unwrap_or(0);
        if possible_first < first_written_index {
            first_written_index = possible_first;
            first_written_digit = i as u32 + 1;
            // numbers.iter().enumerate().find(|nu| *nu.1 == n).unwrap().0 as u32 + 1;
        }
        if possible_last > last_written_index {
            last_written_index = possible_last;
            last_written_digit = 1 as u32 + 1;
            // numbers.iter().enumerate().rfind(|nu| *nu.1 == n).unwrap().0 as u32 + 1;
        }
    }
    let first_actual_digit = if first_digit_index < first_written_index {
        *line
            .chars()
            .filter(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>()
            .first()
            .unwrap()
    } else {
        first_written_digit
    };
    let last_actual_digit = if last_digit_index > last_written_index {
        *line
            .chars()
            .filter(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>()
            .last()
            .unwrap()
    } else {
        last_written_digit
    };
    first_actual_digit * 10 + last_actual_digit
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<String> = input.split("\n").map(|e| e.to_string()).collect();
    let mut sum: u32 = 0;
    for line in lines {
        if line == "" {
            continue;
        }
        let number = parse_number_from_line(line);
        sum += number;
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
