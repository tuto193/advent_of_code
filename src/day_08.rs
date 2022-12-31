use crate::get_file_contents;

fn prepare_input(input: String) -> Vec<Vec<usize>> {
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
                .map(|c| c.parse().unwrap())
                .collect()
        })
        .collect();
    input
}

pub fn day_08() {
    let input = get_file_contents("08".to_string());
    let input = prepare_input(input);
    let total_rows = input.len();
    let row_length = input[0].len();
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
    let column: Vec<usize> = grid.iter().map(|r| r[x]).collect();
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

pub fn day_08_part2() {
    let input = get_file_contents("08".to_string());
    let input = prepare_input(input);
    let total_rows = input.len();
    let row_length = input[0].len();
    let mut scenic_scores: Vec<usize> = vec![];
    for y in 1..total_rows - 1 {
        for x in 1..row_length - 1 {
            let tree = input[y][x];
            let scenic_score = get_scenic_score(tree, (x, y), &input);
            scenic_scores.push(scenic_score);
        }
    }
    scenic_scores.sort();
    let max_score = scenic_scores.pop().unwrap();
    println!("Maximum scenic score is '{}'", max_score);
    // println!("First couple elements are '{:?}'", &scenic_scores[..4]);
    // println!(
    //     "Last couple elements are '{:?}'",
    //     &scenic_scores[total_trees - 4..total_trees]
    // );
}

fn get_scenic_score(tree: usize, coords: (usize, usize), grid: &Vec<Vec<usize>>) -> usize {
    let (x, y) = coords;
    let right_limit = grid[0].len();
    let bottom_limit = grid.len();

    // Look left
    let mut total_left: usize = 0;
    for left_tree in (0..x).rev() {
        total_left += 1;
        if grid[y][left_tree] >= tree {
            break;
        }
    }
    // Look right
    let mut total_right: usize = 0;
    for right_tree in x + 1..right_limit {
        total_right += 1;
        if grid[y][right_tree] >= tree {
            break;
        }
    }
    // Look up
    let mut total_top: usize = 0;
    for top_tree in (0..y).rev() {
        total_top += 1;
        if grid[top_tree][x] >= tree {
            break;
        }
    }
    // Look down
    let mut total_bottom: usize = 0;
    for bottom_tree in y + 1..bottom_limit {
        total_bottom += 1;
        if grid[bottom_tree][x] >= tree {
            break;
        }
    }

    total_left * total_right * total_top * total_bottom
}
