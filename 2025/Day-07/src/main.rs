use std::collections::{HashSet, VecDeque};
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input file");

    let result_part1 = solve_part1(&input);
    println!("Part 1 - Total beam splits: {}", result_part1);

    let result_part2 = solve_part2(&input);
    println!("Part 2 - Total timelines: {}", result_part2);
}

fn solve_part1(input: &str) -> usize {
    let grid = parse_grid(input);
    simulate_beams(&grid)
}

fn solve_part2(input: &str) -> usize {
    let grid = parse_grid(input);
    count_timelines(&grid)
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input.lines()
        .map(|line| line.chars().collect())
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Beam {
    row: usize,
    col: usize,
}

fn simulate_beams(grid: &[Vec<char>]) -> usize {
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };

    // Find the starting position (S)
    let mut start_col = 0;
    for (col_idx, &ch) in grid[0].iter().enumerate() {
        if ch == 'S' {
            start_col = col_idx;
            break;
        }
    }

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut splitters_hit = HashSet::new(); // Track which splitters have been hit

    // Start with a beam at position S
    queue.push_back(Beam { row: 0, col: start_col });

    while let Some(beam) = queue.pop_front() {
        // Check if we've already processed this beam position
        if visited.contains(&beam) {
            continue;
        }
        visited.insert(beam);

        // Move beam downward
        let mut current_row = beam.row;
        let current_col = beam.col;

        // Keep moving down until we hit a splitter or exit the grid
        loop {
            current_row += 1;

            // Check if beam exits the grid
            if current_row >= rows {
                break;
            }

            let cell = grid[current_row][current_col];

            if cell == '^' {
                // Hit a splitter - record it
                let splitter_pos = (current_row, current_col);
                splitters_hit.insert(splitter_pos);

                // Create left beam
                if current_col > 0 {
                    let left_beam = Beam {
                        row: current_row,
                        col: current_col - 1,
                    };
                    if !visited.contains(&left_beam) {
                        queue.push_back(left_beam);
                    }
                }

                // Create right beam
                if current_col + 1 < cols {
                    let right_beam = Beam {
                        row: current_row,
                        col: current_col + 1,
                    };
                    if !visited.contains(&right_beam) {
                        queue.push_back(right_beam);
                    }
                }

                break; // Stop this beam
            }
            // Otherwise continue downward (empty space '.')
        }
    }

    splitters_hit.len()
}

fn count_timelines(grid: &[Vec<char>]) -> usize {
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };

    // Find the starting position (S)
    let mut start_col = 0;
    for (col_idx, &ch) in grid[0].iter().enumerate() {
        if ch == 'S' {
            start_col = col_idx;
            break;
        }
    }

    // Use dynamic programming approach
    // For each position, track how many unique paths lead to it
    // Key: (row, col), Value: number of timelines reaching this point
    let mut paths_at: std::collections::HashMap<(usize, usize), usize> = std::collections::HashMap::new();
    paths_at.insert((0, start_col), 1);

    let mut total_timelines = 0;

    // Process row by row
    for row in 0..rows {
        // Get all positions in current row that have paths
        let current_positions: Vec<((usize, usize), usize)> = paths_at
            .iter()
            .filter(|((r, _), _)| *r == row)
            .map(|(k, v)| (*k, *v))
            .collect();

        for ((_, col), count) in current_positions {
            // Remove from map as we process
            paths_at.remove(&(row, col));

            // Move downward until hitting splitter or exit
            let mut current_row = row;
            loop {
                current_row += 1;

                // Check if beam exits the grid
                if current_row >= rows {
                    total_timelines += count;
                    break;
                }

                let cell = grid[current_row][col];

                if cell == '^' {
                    // Hit a splitter - split paths
                    // Left path
                    if col > 0 {
                        *paths_at.entry((current_row, col - 1)).or_insert(0) += count;
                    }
                    // Right path
                    if col + 1 < cols {
                        *paths_at.entry((current_row, col + 1)).or_insert(0) += count;
                    }
                    break;
                }
            }
        }
    }

    total_timelines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

        let result = solve_part1(input);
        assert_eq!(result, 21);
    }

    #[test]
    fn test_example_part2() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

        let result = solve_part2(input);
        assert_eq!(result, 40);
    }
}
