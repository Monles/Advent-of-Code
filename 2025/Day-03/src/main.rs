use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input file");

    let sum_part1 = solve_part1(&input);
    println!("Part 1 - Total output joltage: {}", sum_part1);

    let sum_part2 = solve_part2(&input);
    println!("Part 2 - Total output joltage: {}", sum_part2);
}

fn solve_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| find_max_joltage_part1(line))
        .sum()
}

fn solve_part2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| find_max_joltage_part2(line))
        .sum()
}

fn find_max_joltage_part1(bank: &str) -> u32 {
    let digits: Vec<char> = bank.chars().collect();
    let mut max_joltage = 0;

    // Try all pairs of batteries (positions i and j where i < j)
    for i in 0..digits.len() {
        for j in (i + 1)..digits.len() {
            // Form a two-digit number from batteries at positions i and j
            let joltage = digits[i].to_digit(10).unwrap() * 10 + digits[j].to_digit(10).unwrap();
            max_joltage = max_joltage.max(joltage);
        }
    }

    max_joltage
}

fn find_max_joltage_part2(bank: &str) -> u64 {
    let digits: Vec<char> = bank.chars().collect();
    let n = digits.len();

    // We need to select exactly 12 batteries
    // To maximize the number, we want the largest digits in the leftmost positions
    // Strategy: Create all combinations of 12 positions, form the number, keep the max

    // For efficiency, we can use a greedy approach with backtracking
    // But given the constraints, let's try a smarter greedy approach:
    // For a 12-digit number to be maximum, we want to pick the 12 largest-valued
    // positions while maintaining order

    // Actually, we need to pick indices such that the resulting 12-digit number is maximum
    // This is a combinatorial problem: C(n, 12) combinations

    // Greedy approach: At each position in our result, pick the largest available digit
    // such that we still have enough remaining digits to complete 12 positions

    let mut selected_indices = Vec::new();
    let target_count = 12;

    for result_pos in 0..target_count {
        let remaining_needed = target_count - result_pos - 1;
        let mut best_digit = '0';
        let mut best_idx = 0;

        // Start searching from the position after the last selected index
        let start_idx = if selected_indices.is_empty() {
            0
        } else {
            selected_indices.last().unwrap() + 1
        };

        // We need to leave enough digits after this choice for the remaining positions
        for idx in start_idx..=(n - remaining_needed - 1) {
            if digits[idx] > best_digit {
                best_digit = digits[idx];
                best_idx = idx;
            }
        }

        selected_indices.push(best_idx);
    }

    // Convert selected indices to a number
    let result_str: String = selected_indices.iter()
        .map(|&idx| digits[idx])
        .collect();

    result_str.parse::<u64>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max_joltage_part1() {
        assert_eq!(find_max_joltage_part1("987654321111111"), 98);
        assert_eq!(find_max_joltage_part1("811111111111119"), 89);
        assert_eq!(find_max_joltage_part1("234234234234278"), 78);
        assert_eq!(find_max_joltage_part1("818181911112111"), 92);
    }

    #[test]
    fn test_part1_example() {
        let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
        let result = solve_part1(input);
        assert_eq!(result, 357);
    }

    #[test]
    fn test_part1_small_cases() {
        assert_eq!(find_max_joltage_part1("12"), 12);
        assert_eq!(find_max_joltage_part1("21"), 21);
        assert_eq!(find_max_joltage_part1("123"), 23);
        assert_eq!(find_max_joltage_part1("321"), 32);
        assert_eq!(find_max_joltage_part1("19"), 19);
        assert_eq!(find_max_joltage_part1("91"), 91);
    }

    #[test]
    fn test_find_max_joltage_part2() {
        assert_eq!(find_max_joltage_part2("987654321111111"), 987654321111);
        assert_eq!(find_max_joltage_part2("811111111111119"), 811111111119);
        assert_eq!(find_max_joltage_part2("234234234234278"), 434234234278);
        assert_eq!(find_max_joltage_part2("818181911112111"), 888911112111);
    }

    #[test]
    fn test_part2_example() {
        let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
        let result = solve_part2(input);
        assert_eq!(result, 3121910778619);
    }
}
