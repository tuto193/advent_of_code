use crate::get_file_contents;

pub fn day_08() {
    let input = get_file_contents("08".to_string());
    let input: Vec<&str> = input.split("\n").collect();
    let row_length = input[0].len();
    let total_rows = input.len() - 1; // we don't count the last empty line
    let input: Vec<Vec<usize>> = input
        .into_iter()
        .take(total_rows)
        .map(|s| {
            s.split("")
                .into_iter()
                // Splitting at "" always puts empty string at front
                .skip(1)
                // and at the back as well; better skip them
                .take(row_length)
                .map(|c| {
                    c.parse().unwrap()
                }).collect()
        }).collect();
    // All trees on the edge are visible:
    let mut total_visible: usize = total_rows * 2 + (row_length * 2 - 4);
    for y in 1..input.len() - 1 {
        for x in 1..row_length - 1 {
            let tree = input[y][x];
            if is_tree_visible(tree, (x, y), &input) {
                total_visible += 1;
            }
        }
    }
    println!("Visible trees: '{}'", total_visible);
}

fn is_tree_visible(tree_size: usize, coords: (usize, usize), grid: &Vec<Vec<usize>>) -> bool {
    let (x, y) = coords;
    let row = &grid[y];
    let column: Vec<usize> = grid.iter().map(|r| {
        r[x]
        }).collect();
    // Left
    if row[0..x].iter().all(|&other_t| other_t < tree_size) {
        return true;
    }
    // right
    if row[x + 1..].iter().all(|&other_t| other_t < tree_size) {
        return true;
    }
    // Top
    if column[0..y].iter().all(|&other_t| other_t < tree_size) {
        return true;
    }
    // Bottom
    if column[y + 1..].iter().all(|&other_t| other_t < tree_size) {
        return true;
    }
    false
}
