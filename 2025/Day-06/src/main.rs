use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input file");

    let result_part1 = solve_part1(&input);
    println!("Part 1 - Grand total: {}", result_part1);

    let result_part2 = solve_part2(&input);
    println!("Part 2 - Grand total: {}", result_part2);
}

fn solve_part1(input: &str) -> u64 {
    let problems = parse_worksheet_part1(input);

    problems.iter()
        .map(|problem| evaluate_problem(problem))
        .sum()
}

fn solve_part2(input: &str) -> u64 {
    let problems = parse_worksheet_part2(input);

    problems.iter()
        .map(|problem| evaluate_problem(problem))
        .sum()
}

#[derive(Debug)]
struct Problem {
    numbers: Vec<u64>,
    operator: char,
}

fn parse_worksheet_part1(input: &str) -> Vec<Problem> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return vec![];
    }

    // Transpose the grid to work column by column
    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    // Get the operator line (last line)
    let operator_line = lines.last().unwrap();

    let mut problems = Vec::new();
    let mut col = 0;

    while col < max_len {
        // Skip spaces and find the next problem
        // A problem starts when we find a non-space character
        while col < max_len && col < operator_line.len() {
            let op_char = operator_line.chars().nth(col).unwrap_or(' ');
            if op_char == '+' || op_char == '*' {
                break;
            }
            col += 1;
        }

        if col >= max_len {
            break;
        }

        // Found an operator at this column
        let op_char = operator_line.chars().nth(col).unwrap_or(' ');

        if op_char == '+' || op_char == '*' {
            // Find the start of this problem (leftmost column with digits)
            let mut start_col = col;
            while start_col > 0 {
                let prev_col = start_col - 1;
                let mut has_digit = false;

                for line in lines.iter().take(lines.len() - 1) {
                    if prev_col < line.len() {
                        let ch = line.chars().nth(prev_col).unwrap_or(' ');
                        if ch.is_ascii_digit() {
                            has_digit = true;
                            break;
                        }
                    }
                }

                if has_digit {
                    start_col = prev_col;
                } else {
                    break;
                }
            }

            // Find the end of this problem (rightmost column with digits)
            let mut end_col = col;
            loop {
                let next_col = end_col + 1;
                if next_col >= max_len {
                    break;
                }

                let mut has_digit = false;
                for line in lines.iter().take(lines.len() - 1) {
                    if next_col < line.len() {
                        let ch = line.chars().nth(next_col).unwrap_or(' ');
                        if ch.is_ascii_digit() {
                            has_digit = true;
                            break;
                        }
                    }
                }

                if has_digit {
                    end_col = next_col;
                } else {
                    break;
                }
            }

            // Extract all numbers in this column range
            let mut numbers = Vec::new();
            for line in lines.iter().take(lines.len() - 1) {
                // Extract the substring for this problem
                if start_col < line.len() {
                    let end = (end_col + 1).min(line.len());
                    let segment = &line[start_col..end];
                    let trimmed = segment.trim();

                    if !trimmed.is_empty() && trimmed.chars().all(|c| c.is_ascii_digit()) {
                        numbers.push(trimmed.parse::<u64>().unwrap());
                    }
                }
            }

            if !numbers.is_empty() {
                problems.push(Problem {
                    numbers,
                    operator: op_char,
                });
            }

            col = end_col + 1;
        } else {
            col += 1;
        }
    }

    problems
}

fn parse_worksheet_part2(input: &str) -> Vec<Problem> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return vec![];
    }

    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let operator_line = lines.last().unwrap();

    let mut problems = Vec::new();
    let mut col = 0;

    while col < max_len {
        // Find the next operator
        while col < max_len && col < operator_line.len() {
            let op_char = operator_line.chars().nth(col).unwrap_or(' ');
            if op_char == '+' || op_char == '*' {
                break;
            }
            col += 1;
        }

        if col >= max_len {
            break;
        }

        let op_char = operator_line.chars().nth(col).unwrap_or(' ');

        if op_char == '+' || op_char == '*' {
            // Find problem boundaries (same as Part 1)
            let mut start_col = col;
            while start_col > 0 {
                let prev_col = start_col - 1;
                let mut has_digit = false;

                for line in lines.iter().take(lines.len() - 1) {
                    if prev_col < line.len() {
                        let ch = line.chars().nth(prev_col).unwrap_or(' ');
                        if ch.is_ascii_digit() {
                            has_digit = true;
                            break;
                        }
                    }
                }

                if has_digit {
                    start_col = prev_col;
                } else {
                    break;
                }
            }

            let mut end_col = col;
            loop {
                let next_col = end_col + 1;
                if next_col >= max_len {
                    break;
                }

                let mut has_digit = false;
                for line in lines.iter().take(lines.len() - 1) {
                    if next_col < line.len() {
                        let ch = line.chars().nth(next_col).unwrap_or(' ');
                        if ch.is_ascii_digit() {
                            has_digit = true;
                            break;
                        }
                    }
                }

                if has_digit {
                    end_col = next_col;
                } else {
                    break;
                }
            }

            // Part 2: Read columns right-to-left, each column is a digit position
            let mut numbers = Vec::new();

            // Process columns from right to left
            for problem_col in (start_col..=end_col).rev() {
                // Read this column top-to-bottom to form a number
                let mut digits_str = String::new();

                for line in lines.iter().take(lines.len() - 1) {
                    if problem_col < line.len() {
                        let ch = line.chars().nth(problem_col).unwrap_or(' ');
                        if ch.is_ascii_digit() {
                            digits_str.push(ch);
                        } else if ch == ' ' && !digits_str.is_empty() {
                            // Space in the middle of a number, skip
                        }
                    }
                }

                if !digits_str.is_empty() {
                    if let Ok(num) = digits_str.parse::<u64>() {
                        numbers.push(num);
                    }
                }
            }

            if !numbers.is_empty() {
                problems.push(Problem {
                    numbers,
                    operator: op_char,
                });
            }

            col = end_col + 1;
        } else {
            col += 1;
        }
    }

    problems
}

fn evaluate_problem(problem: &Problem) -> u64 {
    if problem.numbers.is_empty() {
        return 0;
    }

    let mut result = problem.numbers[0];

    for &num in &problem.numbers[1..] {
        match problem.operator {
            '+' => result += num,
            '*' => result *= num,
            _ => {}
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

        let result = solve_part1(input);
        assert_eq!(result, 4277556);
    }

    #[test]
    fn test_part1_individual_problems() {
        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

        let problems = parse_worksheet_part1(input);

        assert_eq!(problems.len(), 4);

        // Problem 1: 123 * 45 * 6 = 33210
        assert_eq!(evaluate_problem(&problems[0]), 33210);

        // Problem 2: 328 + 64 + 98 = 490
        assert_eq!(evaluate_problem(&problems[1]), 490);

        // Problem 3: 51 * 387 * 215 = 4243455
        assert_eq!(evaluate_problem(&problems[2]), 4243455);

        // Problem 4: 64 + 23 + 314 = 401
        assert_eq!(evaluate_problem(&problems[3]), 401);
    }

    #[test]
    fn test_part2_example() {
        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

        let result = solve_part2(input);
        assert_eq!(result, 3263827);
    }

    #[test]
    fn test_part2_individual_problems() {
        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

        let problems = parse_worksheet_part2(input);

        assert_eq!(problems.len(), 4);

        // Problem parsing is left-to-right, but reading is right-to-left per problem
        // Leftmost problem (index 0): 356 * 24 * 1 = 8544
        assert_eq!(evaluate_problem(&problems[0]), 8544);

        // Second problem (index 1): 8 + 248 + 369 = 625
        assert_eq!(evaluate_problem(&problems[1]), 625);

        // Third problem (index 2): 175 * 581 * 32 = 3253600
        assert_eq!(evaluate_problem(&problems[2]), 3253600);

        // Rightmost problem (index 3): 4 + 431 + 623 = 1058
        assert_eq!(evaluate_problem(&problems[3]), 1058);
    }
}
