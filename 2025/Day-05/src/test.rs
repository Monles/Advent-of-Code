use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");

    let fresh_count = count_fresh_ingredients(&input);
    println!("Part 1 - Number of fresh ingredients: {}", fresh_count);

    let total_fresh_ids = count_total_fresh_ids(&input);
    println!("Part 2 - Total fresh IDs in ranges: {}", total_fresh_ids);
}

fn count_fresh_ingredients(input: &str) -> usize {
    let parts: Vec<&str> = input.split("\n\n").collect();

    if parts.len() != 2 {
        eprintln!("Invalid input format");
        return 0;
    }

    // Parse ranges
    let ranges: Vec<(u64, u64)> = parts[0]
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split('-').collect();
            if parts.len() == 2 {
                let start = parts[0].parse::<u64>().ok()?;
                let end = parts[1].parse::<u64>().ok()?;
                Some((start, end))
            } else {
                None
            }
        })
        .collect();

    // Parse ingredient IDs
    let ingredients: Vec<u64> = parts[1]
        .lines()
        .filter_map(|line| line.trim().parse::<u64>().ok())
        .collect();

    // Count fresh ingredients
    let mut fresh_count = 0;
    for &ingredient_id in &ingredients {
        if is_fresh(ingredient_id, &ranges) {
            fresh_count += 1;
        }
    }

    fresh_count
}

fn is_fresh(id: u64, ranges: &[(u64, u64)]) -> bool {
    for &(start, end) in ranges {
        if id >= start && id <= end {
            return true;
        }
    }
    false
}

fn count_total_fresh_ids(input: &str) -> u64 {
    let parts: Vec<&str> = input.split("\n\n").collect();

    if parts.is_empty() {
        return 0;
    }

    // Parse ranges
    let mut ranges: Vec<(u64, u64)> = parts[0]
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split('-').collect();
            if parts.len() == 2 {
                let start = parts[0].parse::<u64>().ok()?;
                let end = parts[1].parse::<u64>().ok()?;
                Some((start, end))
            } else {
                None
            }
        })
        .collect();

    // Merge overlapping ranges
    let merged_ranges = merge_ranges(&mut ranges);

    // Count total IDs in merged ranges
    merged_ranges
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum()
}

fn merge_ranges(ranges: &mut [(u64, u64)]) -> Vec<(u64, u64)> {
    if ranges.is_empty() {
        return Vec::new();
    }

    // Sort ranges by start position
    ranges.sort_by_key(|&(start, _)| start);

    let mut merged = Vec::new();
    let mut current = ranges[0];

    for &(start, end) in &ranges[1..] {
        if start <= current.1 + 1 {
            // Overlapping or adjacent ranges - merge them
            current.1 = current.1.max(end);
        } else {
            // Non-overlapping range - save current and start new
            merged.push(current);
            current = (start, end);
        }
    }
    merged.push(current);

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

        assert_eq!(count_fresh_ingredients(input), 3);
    }

    #[test]
    fn test_is_fresh() {
        let ranges = vec![(3, 5), (10, 14), (16, 20), (12, 18)];

        assert!(!is_fresh(1, &ranges)); // spoiled
        assert!(is_fresh(5, &ranges)); // fresh (in range 3-5)
        assert!(!is_fresh(8, &ranges)); // spoiled
        assert!(is_fresh(11, &ranges)); // fresh (in range 10-14)
        assert!(is_fresh(17, &ranges)); // fresh (in ranges 16-20 and 12-18)
        assert!(!is_fresh(32, &ranges)); // spoiled
    }

    #[test]
    fn test_part2_example() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

        // Ranges: 3-5 (3 IDs), 10-20 merged (11 IDs) = 14 total
        assert_eq!(count_total_fresh_ids(input), 14);
    }

    #[test]
    fn test_merge_ranges() {
        let mut ranges = vec![(3, 5), (10, 14), (16, 20), (12, 18)];
        let merged = merge_ranges(&mut ranges);

        // Should merge 10-14, 12-18, and 16-20 into 10-20
        // And keep 3-5 separate
        assert_eq!(merged.len(), 2);
        assert_eq!(merged[0], (3, 5));
        assert_eq!(merged[1], (10, 20));
    }
}
