fn main() {
    let input = vec![
        "..@@.@@@@.".chars().collect::<Vec<char>>(),
    ];

    // Check position (0, 8) which is '@' and should be 'x' (accessible)
    let row = 0;
    let col = 8;

    println!("Checking position ({}, {}) = '{}'", row, col, input[row][col]);

    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    let mut count = 0;
    for (dr, dc) in directions.iter() {
        let nr = row as i32 + dr;
        let nc = col as i32 + dc;

        if nr >= 0 && nr < 1 && nc >= 0 && nc < 10 {
            let ch = if nr < input.len() as i32 && nc < input[0].len() as i32 {
                input[nr as usize][nc as usize]
            } else {
                '.'
            };

            if ch == '@' {
                count += 1;
                println!("  Neighbor at ({}, {}) = '{}'", nr, nc, ch);
            }
        }
    }

    println!("Total adjacent @'s: {}", count);
    println!("Accessible (< 4): {}", count < 4);
}
