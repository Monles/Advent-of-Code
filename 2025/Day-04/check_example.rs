fn main() {
    let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    let expected = "..xx.xx@x.
x@@.@.@.@@
@@@@@.x.@@
@.@@@@..@.
x@.@@@@.@x
.@@@@@@@.@
.@.@.@.@@@
x.@@@.@@@@
.@@@@@@@@.
x.x.@@@.x.";

    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let exp_grid: Vec<Vec<char>> = expected.lines().map(|l| l.chars().collect()).collect();

    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    let rows = grid.len();
    let cols = grid[0].len();

    // Check all '@' positions
    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == '@' {
                let mut adjacent_count = 0;

                for (dr, dc) in directions.iter() {
                    let nr = row as i32 + dr;
                    let nc = col as i32 + dc;

                    if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                        if grid[nr as usize][nc as usize] == '@' {
                            adjacent_count += 1;
                        }
                    }
                }

                let expected_char = exp_grid[row][col];
                let should_be_accessible = adjacent_count < 4;
                let marked_accessible = expected_char == 'x';

                if should_be_accessible != marked_accessible {
                    println!("MISMATCH at ({}, {}): adjacent={}, logic says {}, but marked as '{}'",
                             row, col, adjacent_count,
                             if should_be_accessible { "accessible" } else { "not accessible" },
                             expected_char);
                }
            }
        }
    }

    println!("Check complete.");
}
