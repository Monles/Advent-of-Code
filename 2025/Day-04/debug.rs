fn main() {
    let input = vec![
        "..@@.@@@@.".chars().collect::<Vec<char>>(),
        "@@@.@.@.@@".chars().collect(),
        "@@@@@.@.@@".chars().collect(),
        "@.@@@@..@.".chars().collect(),
        "@@.@@@@.@@".chars().collect(),
        ".@@@@@@@.@".chars().collect(),
        ".@.@.@.@@@".chars().collect(),
        "@.@@@.@@@@".chars().collect(),
        ".@@@@@@@@.".chars().collect(),
        "@.@.@@@.@.".chars().collect(),
    ];

    let expected_accessible = vec![
        "..xx.xx@x.".chars().collect::<Vec<char>>(),
        "x@@.@.@.@@".chars().collect(),
        "@@@@@.x.@@".chars().collect(),
        "@.@@@@..@.".chars().collect(),
        "x@.@@@@.@x".chars().collect(),
        ".@@@@@@@.@".chars().collect(),
        ".@.@.@.@@@".chars().collect(),
        "x.@@@.@@@@".chars().collect(),
        ".@@@@@@@@.".chars().collect(),
        "x.x.@@@.x.".chars().collect(),
    ];

    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    let rows = input.len();
    let cols = input[0].len();

    println!("Checking each @ position:");
    for row in 0..rows {
        for col in 0..cols {
            if input[row][col] == '@' {
                let mut adjacent_rolls = 0;

                for (dr, dc) in directions.iter() {
                    let new_row = row as i32 + dr;
                    let new_col = col as i32 + dc;

                    if new_row >= 0 && new_row < rows as i32
                        && new_col >= 0 && new_col < cols as i32 {
                        if input[new_row as usize][new_col as usize] == '@' {
                            adjacent_rolls += 1;
                        }
                    }
                }

                let is_accessible = adjacent_rolls < 4;
                let expected_char = expected_accessible[row][col];

                if is_accessible != (expected_char == 'x') {
                    println!("MISMATCH at ({}, {}): adjacent={}, accessible={}, expected={}",
                             row, col, adjacent_rolls, is_accessible, expected_char);
                }
            }
        }
    }
}
