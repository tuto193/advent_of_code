pub fn part_one(input: &str) -> Option<u32> {
    let r_p_s = vec!["X", "Y", "Z"];
    let wins = vec!["C X", "A Y", "B Z"];
    let draws = vec!["A X", "B Y", "C Z"];
    let contents = input;
    let mut used_tactic_score: u32 = 0;
    let mut outcome_score: u32 = 0;
    for i in contents.split("\n").into_iter() {
        // if wins.contains(&i) {
        //     // outcome_score += 6;
        // } else if losses.contains(&&i) {
        //     // outcome_score += 0;
        // } else if i == "" {
        if i == "" {
            print!("At the end of file");
            println!("The score is: {}", outcome_score + used_tactic_score);
            continue;
        }
        // } else {
        //     outcome_score += 3;
        // }
        if wins.contains(&i) {
            outcome_score += 6;
        } else if draws.contains(&i) {
            outcome_score += 3
        }
        let you_vs_me = i.split(" ").collect::<Vec<&str>>();
        // println!("This round is: {:?}", you_vs_me);
        match you_vs_me[1] {
            // Rock
            "X" => used_tactic_score += 1,
            // Paper
            "Y" => used_tactic_score += 2,
            // Scisors
            "Z" => used_tactic_score += 3,
            x => print!("Found a weird sign: {}", x),
        }
    }
    Some(outcome_score + used_tactic_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    // A, X = Rock
    // B, Y = Paper
    // C, Z = Scissors
    // let wins = vec!["A Y", "B Z", "C X"];
    // let losses = vec!["A Z", "B X", "C Y"];
    let r_p_s = vec!["A", "B", "C"];
    let contents = input;
    let mut used_tactic_score: u32 = 0;
    let mut outcome_score: u32 = 0;
    for i in contents.split("\n").into_iter() {
        // if wins.contains(&i) {
        //     // outcome_score += 6;
        // } else if losses.contains(&&i) {
        //     // outcome_score += 0;
        // } else if i == "" {
        if i == "" {
            print!("At the end of file");
            println!("The score is: {}", outcome_score + used_tactic_score);
            continue;
        }
        // } else {
        //     outcome_score += 3;
        // }
        let you_vs_me = i.split(" ").collect::<Vec<&str>>();
        // println!("This round is: {:?}", you_vs_me);
        let op_index = r_p_s.iter().position(|&el| el == you_vs_me[0]).unwrap();
        match you_vs_me[1] {
            // Loss
            "X" => {
                // used_tactic_score += 0;
                if op_index == 0 {
                    outcome_score += 3;
                } else {
                    outcome_score += op_index as u32
                }
            }
            // Draw
            "Y" => {
                used_tactic_score += 3;
                outcome_score += op_index as u32 + 1;
            }
            // Win
            "Z" => {
                used_tactic_score += 6;
                if op_index == 2 {
                    outcome_score += 1;
                } else {
                    outcome_score += op_index as u32 + 2;
                }
            }
            x => print!("Found a weird sign: {}", x),
        }
    }
    Some(outcome_score + used_tactic_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
