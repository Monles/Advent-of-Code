use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input.txt");

    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    if grid.is_empty() {
        println!("Error: input.txt is empty. Please add your puzzle input.");
        return;
    }

    // Part 1
    let accessible_count = count_accessible_rolls(&grid);
    println!("Part 1 - Number of accessible rolls: {}", accessible_count);

    // Part 2
    let total_removed = remove_all_accessible_rolls(grid);
    println!("Part 2 - Total rolls removed: {}", total_removed);
}

fn count_accessible_rolls(grid: &[Vec<char>]) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    // Direction vectors for 8 adjacent positions
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),  // top-left, top, top-right
        (0, -1),           (0, 1),    // left, right
        (1, -1),  (1, 0),  (1, 1),    // bottom-left, bottom, bottom-right
    ];

    for row in 0..rows {
        for col in 0..cols {
            // Only check positions with a roll of paper (@)
            if grid[row][col] == '@' {
                let mut adjacent_rolls = 0;

                // Check all 8 adjacent positions
                for (dr, dc) in directions.iter() {
                    let new_row = row as i32 + dr;
                    let new_col = col as i32 + dc;

                    // Check if the position is within bounds
                    if new_row >= 0 && new_row < rows as i32
                        && new_col >= 0 && new_col < cols as i32 {
                        if grid[new_row as usize][new_col as usize] == '@' {
                            adjacent_rolls += 1;
                        }
                    }
                }

                // A roll is accessible if there are fewer than 4 adjacent rolls
                if adjacent_rolls < 4 {
                    count += 1;
                }
            }
        }
    }

    count
}

fn count_adjacent_rolls(grid: &[Vec<char>], row: usize, col: usize) -> usize {
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for (dr, dc) in directions.iter() {
        let new_row = row as i32 + dr;
        let new_col = col as i32 + dc;

        if new_row >= 0 && new_row < rows as i32
            && new_col >= 0 && new_col < cols as i32 {
            if grid[new_row as usize][new_col as usize] == '@' {
                count += 1;
            }
        }
    }

    count
}

fn remove_all_accessible_rolls(mut grid: Vec<Vec<char>>) -> usize {
    let mut total_removed = 0;

    loop {
        // Find all accessible rolls in current state
        let mut to_remove = Vec::new();

        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if grid[row][col] == '@' {
                    let adjacent_count = count_adjacent_rolls(&grid, row, col);
                    if adjacent_count < 4 {
                        to_remove.push((row, col));
                    }
                }
            }
        }

        // If no more accessible rolls, we're done
        if to_remove.is_empty() {
            break;
        }

        // Remove all accessible rolls
        for (row, col) in to_remove.iter() {
            grid[*row][*col] = '.';
        }

        total_removed += to_remove.len();
    }

    total_removed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = vec![
            "..@@.@@@@.".chars().collect(),
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

        assert_eq!(count_accessible_rolls(&input), 13);
    }

    #[test]
    fn test_part2_example() {
        let input = vec![
            "..@@.@@@@.".chars().collect(),
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

        assert_eq!(remove_all_accessible_rolls(input), 43);
    }
}
