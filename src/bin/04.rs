advent_of_code::solution!(4);
use std::collections::HashSet;

fn get_total_winners_from_line(line: String) -> usize {
    let wanted_part = line.split(": ").collect::<Vec<&str>>()[1]
        .split(" | ")
        .collect::<Vec<&str>>();
    let winning_numbers = wanted_part[0]
        .split_ascii_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<HashSet<usize>>();
    let other_numbers = wanted_part[1]
        .split_ascii_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<HashSet<usize>>();
    let total_winners = winning_numbers
        .intersection(&other_numbers)
        .collect::<Vec<&usize>>();
    total_winners.iter().count()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total_points = 0;
    let lines = input
        .split("\n")
        .filter(|&l| l != "")
        .map(|l| l.to_string())
        .collect::<Vec<String>>();
    let base: usize = 2;
    for line in lines.into_iter() {
        let exponent = get_total_winners_from_line(line) as u32;
        if exponent == 0 {
            continue;
        }
        total_points += base.pow(exponent - 1);
    }
    Some(total_points as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input
        .split("\n")
        .filter(|&l| l != "")
        .map(|l| l.to_string())
        .collect::<Vec<String>>();
    // let mut total_cards = lines.len();
    let mut multipliers: Vec<usize> = vec![1; lines.len()];
    // multipliers.resize(lines.len(), 1);
    for (card_index, s_card) in lines.into_iter().enumerate() {
        // let card_multiply = if card_index == 0 {
        //     1
        // } else {
        //     multipliers[card_index - 1]
        // };
        let card_multiply = multipliers[card_index];
        let total_winners = get_total_winners_from_line(s_card);
        multipliers
            .clone()
            .iter()
            .enumerate()
            .skip(card_index + 1)
            .take(total_winners)
            .for_each(|(i, mul)| multipliers[i] = mul + card_multiply);
    }
    Some(multipliers.into_iter().sum::<usize>() as u32)
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
