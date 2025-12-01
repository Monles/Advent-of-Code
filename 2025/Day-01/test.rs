use std::fs;

fn main() {
    let input = fs::read_to_string("test_input.txt").expect("Failed to read test_input.txt");
    let password_part1 = solve_part1(&input);
    println!("Part 1 Password (expected 3): {}", password_part1);

    let password_part2 = solve_part2(&input);
    println!("Part 2 Password (expected 6): {}", password_part2);
}

fn solve_part1(input: &str) -> i32 {
    let mut position: i32 = 50;
    let mut zero_count: i32 = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let direction = line.chars().next().unwrap();
        let distance: i32 = line[1..].trim().parse().expect("Invalid distance");

        position = match direction {
            'L' => (position - distance).rem_euclid(100),
            'R' => (position + distance) % 100,
            _ => panic!("Invalid direction: {}", direction),
        };

        if position == 0 {
            zero_count += 1;
        }
    }
    zero_count
}

fn solve_part2(input: &str) -> i32 {
    let mut position: i32 = 50;
    let mut zero_count: i32 = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let direction = line.chars().next().unwrap();
        let distance: i32 = line[1..].trim().parse().expect("Invalid distance");

        // Count how many times we pass through 0 during the rotation
        zero_count += count_zeros_in_rotation(position, direction, distance);

        // Update position
        position = match direction {
            'L' => (position - distance).rem_euclid(100),
            'R' => (position + distance) % 100,
            _ => panic!("Invalid direction: {}", direction),
        };
    }
    zero_count
}

fn count_zeros_in_rotation(start: i32, direction: char, distance: i32) -> i32 {
    let mut count = 0;

    match direction {
        'L' => {
            // Moving left (decreasing numbers, wrapping at 0)
            for i in 1..=distance {
                let pos = (start - i).rem_euclid(100);
                if pos == 0 {
                    count += 1;
                }
            }
        }
        'R' => {
            // Moving right (increasing numbers, wrapping at 99)
            for i in 1..=distance {
                let pos = (start + i) % 100;
                if pos == 0 {
                    count += 1;
                }
            }
        }
        _ => panic!("Invalid direction: {}", direction),
    }

    count
}
