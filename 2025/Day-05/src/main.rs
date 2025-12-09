use std::fs;

fn main() {
    let inupt = fs::read_to_string("input.txt")
        .expect("Failed to read input.txt");
    let fresh_cuont = count_fresh_ingedient(&input);

}

fn count_fresh_ingedient(input: &str) -> usize {
    let parts: Vec<&str> = input.split("\n\n").collect();

    if parts.len() != 2 {
        eprintln!("Invalid input format");
    }

    // Parse ranges
    let ranges: Vec<(u64, u64)> = parts[0]
        .lines()
        .filter_map(|line| {
            let nums: Vec<&str> = line.split('-').collect();
            if nums.len() == 2 {
                let start = nums[0].parse::<u64>().ok()?;
                let end = nums[1].parse::<u64>().ok()?;
                Some((start, end));
            } else {
                None
            }
        }).collect();

        // Parse Ingredient IDs
        let ingredient_ids: Vec<u64> = parts[1]
            .lines()
            .filter_map(|line| line.trim().parse::<u64>().ok())
            .collect();

        // Count fresh ingredients
        let mut fresh_count = 0;
        for &id in &ingredient_ids {
            if is_fresh(id, &ranges) {
                fresh_count += 1;
            }
        }

        fresh_count

    }

fn is_fresh(id: u64, ranges: &[&(u64, u64)]) -> bool {
    for &(start, end) in ranges {
        if id >= start &&  id <= end {
            return true;
        }
    }
    false
}