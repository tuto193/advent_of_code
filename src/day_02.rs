pub fn day_02() {
    // A, X = Rock
    // B, Y = Paper
    // C, Z = Scissors
    // let wins = vec!["A Y", "B Z", "C X"];
    // let losses = vec!["A Z", "B X", "C Y"];
    let r_p_s = vec!["A", "B", "C"];
    let contents = crate::get_file_contents("02".into());
    let mut used_tactic_score = 0;
    let mut outcome_score = 0;
    for i in contents.split("\n").into_iter() {
        // if wins.contains(&i) {
        //     // outcome_score += 6;
        // } else if losses.contains(&&i) {
        //     // outcome_score += 0;
        // } else if i == "" {
        if i == "" {
            print!("At the end of file");
            println!("The score is: {}", outcome_score + used_tactic_score);
            return;
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
                    outcome_score += op_index
                }
            }
            // Draw
            "Y" => {
                used_tactic_score += 3;
                outcome_score += op_index + 1;
            }
            // Win
            "Z" => {
                used_tactic_score += 6;
                if op_index == 2 {
                    outcome_score += 1;
                } else {
                    outcome_score += op_index + 2;
                }
            }
            x => print!("Found a weird sign: {}", x),
        }
    }
    println!("The score is: {}", outcome_score + used_tactic_score)
}
