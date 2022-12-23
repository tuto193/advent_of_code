

pub fn day_02() -> usize {
    // A, X = Rock
    // B, Y = Paper
    // C, Z = Scissors
    let wins = vec!["A Y", "B Z", "C X"];
    let losses = vec!["A Z", "B X", "C Y"];
    let contents = crate::get_file_contents("02".into());
    let mut used_tactic_score = 0;
    for i in contents.split("\n").into_iter() {

        match i {
            "A" => used_tactic_score += 1,
            "B" => used_tactic_score += 2,
            "C" => used_tactic_score += 3,

        }
    }
    0
}
