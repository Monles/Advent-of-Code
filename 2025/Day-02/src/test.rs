use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");

    let sum = solve(&input);
    println!("Sum of all invalid IDs: {}", sum);
}

fn solve(input: &str) -> u64 {
    let ranges = parse_ranges(input.trim());

    let mut total = 0u64;
    for (start, end) in ranges {
        for id in start..=end {
            if is_invalid_id(id) {
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

fn is_invalid_id(id: u64) -> bool {
    let s = id.to_string();
    let len = s.len();

    // The number must have even length to be made of two identical parts
    if len % 2 != 0 {
        return false;
    }

    // Check if the first half equals the second half
    let mid = len / 2;
    let first_half = &s[..mid];
    let second_half = &s[mid..];

    first_half == second_half
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_invalid_id() {
        assert!(is_invalid_id(11));
        assert!(is_invalid_id(22));
        assert!(is_invalid_id(55));
        assert!(is_invalid_id(99));
        assert!(is_invalid_id(6464));
        assert!(is_invalid_id(123123));
        assert!(is_invalid_id(1010));
        assert!(is_invalid_id(1188511885));
        assert!(is_invalid_id(222222));
        assert!(is_invalid_id(446446));
        assert!(is_invalid_id(38593859));

        assert!(!is_invalid_id(101));
        assert!(!is_invalid_id(1698522));
        assert!(!is_invalid_id(1698528));
    }

    #[test]
    fn test_example() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
                     1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
                     824824821-824824827,2121212118-2121212124";

        let result = solve(input);
        assert_eq!(result, 1227775554);
    }
}
