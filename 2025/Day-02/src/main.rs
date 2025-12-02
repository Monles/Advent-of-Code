use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input");

    let sum_part1 = solve(&input, false);
    println!("Part 1 - Sum of all invalid IDs: {}", sum_part1);

    let sum_part2 = solve(&input, true);
    println!("Part 2 - Sum of all invalid IDs: {}", sum_part2);
}

fn solve(input: &str, part2: bool) -> u64 {
    let ranges = parse_ranges(input.trim());
    let mut total = 0u64;
    for (start, end) in ranges {
        for id in start..=end {
            let invalid = if part2 {
                is_invalid_id_part2(id)
            } else {
                is_invalid_id_part1(id)
            };
            if invalid {
                total += id;
            }
        }
    }
    total
}

fn parse_ranges(input: &str) -> Vec<(u64, u64)> {
    input
        .split(',')
        .filter_map(|range| {
            let parts: Vec<&str> = range.split('-').collect();
            if parts.len() == 2 {
                let start = parts[0].parse::<u64>().ok()?;
                let end = parts[1].parse::<u64>().ok()?;
                Some((start, end))
            } else {
                None
            }
        })
        .collect()
}

// Part 1: ID is invalid if it's a sequence repeated exactly twice
fn is_invalid_id_part1(id: u64) -> bool {
    let s = id.to_string();
    let len = s.len();

    if len % 2 != 0 {
        return false;
    }

    let mid = len / 2;
    let first_half = &s[..mid];
    let second_half = &s[mid..];

    first_half == second_half
}

// Part 2: ID is invalid if it's a sequence repeated at least twice
fn is_invalid_id_part2(id: u64) -> bool {
    let s = id.to_string();
    let len = s.len();

    // Try all possible pattern lengths from 1 to len/2
    for pattern_len in 1..=(len / 2) {
        // Check if the string length is divisible by pattern length
        if len % pattern_len == 0 {
            let repetitions = len / pattern_len;
            // We need at least 2 repetitions
            if repetitions >= 2 {
                let pattern = &s[..pattern_len];
                // Check if the entire string is made of this pattern repeated
                let is_match = s.chars()
                    .collect::<Vec<_>>()
                    .chunks(pattern_len)
                    .all(|chunk| {
                        let chunk_str: String = chunk.iter().collect();
                        chunk_str == pattern
                    });

                if is_match {
                    return true;
                }
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_invalid_id_part1() {
        // Valid cases for Part 1 (repeated exactly twice)
        assert!(is_invalid_id_part1(11));
        assert!(is_invalid_id_part1(22));
        assert!(is_invalid_id_part1(55));
        assert!(is_invalid_id_part1(99));
        assert!(is_invalid_id_part1(6464));
        assert!(is_invalid_id_part1(123123));
        assert!(is_invalid_id_part1(1010));
        assert!(is_invalid_id_part1(1188511885));
        assert!(is_invalid_id_part1(222222));
        assert!(is_invalid_id_part1(446446));
        assert!(is_invalid_id_part1(38593859));

        // Invalid cases for Part 1
        assert!(!is_invalid_id_part1(101));
        assert!(!is_invalid_id_part1(1698522));
        assert!(!is_invalid_id_part1(1698528));
        assert!(!is_invalid_id_part1(111)); // 3 times, not 2
        assert!(!is_invalid_id_part1(999)); // 3 times, not 2
    }

    #[test]
    fn test_is_invalid_id_part2() {
        // Part 1 cases still work
        assert!(is_invalid_id_part2(11));
        assert!(is_invalid_id_part2(22));
        assert!(is_invalid_id_part2(6464));
        assert!(is_invalid_id_part2(123123));
        assert!(is_invalid_id_part2(1010));
        assert!(is_invalid_id_part2(222222));
        assert!(is_invalid_id_part2(446446));

        // New Part 2 cases (repeated 3+ times)
        assert!(is_invalid_id_part2(111)); // 1 three times
        assert!(is_invalid_id_part2(999)); // 9 three times
        assert!(is_invalid_id_part2(12341234)); // 1234 two times
        assert!(is_invalid_id_part2(123123123)); // 123 three times
        assert!(is_invalid_id_part2(1212121212)); // 12 five times
        assert!(is_invalid_id_part2(1111111)); // 1 seven times
        assert!(is_invalid_id_part2(565656)); // 56 three times
        assert!(is_invalid_id_part2(824824824)); // 824 three times
        assert!(is_invalid_id_part2(2121212121)); // 21 five times

        // Still invalid cases
        assert!(!is_invalid_id_part2(101));
        assert!(!is_invalid_id_part2(1698522));
        assert!(!is_invalid_id_part2(1698528));
    }

    #[test]
    fn test_part1_example() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
                     1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
                     824824821-824824827,2121212118-2121212124";

        let result = solve(input, false);
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn test_part2_example() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
                     1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
                     824824821-824824827,2121212118-2121212124";

        let result = solve(input, true);
        assert_eq!(result, 4174379265);
    }
}
