# --- Day 1: Secret Entrance --- (Part 01)

The Elves have good news and bad news.

The good news is that they've discovered project management! This has given them the tools they need to prevent their usual Christmas emergency. For example, they now know that the North Pole decorations need to be finished soon so that other critical tasks can start on time.

The bad news is that they've realized they have a different emergency: according to their resource planning, none of them have any time left to decorate the North Pole!

To save Christmas, the Elves need you to finish decorating the North Pole by December 12th.

Collect stars by solving puzzles. Two puzzles will be made available on each day; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

You arrive at the secret entrance to the North Pole base ready to start decorating. Unfortunately, the password seems to have been changed, so you can't get in. A document taped to the wall helpfully explains:

"Due to new security protocols, the password is locked in the safe below. Please see the attached document for the new combination."

The safe has a dial with only an arrow on it; around the dial are the numbers 0 through 99 in order. As you turn the dial, it makes a small click noise as it reaches each number.

The attached document (your puzzle input) contains a sequence of rotations, one per line, which tell you how to open the safe. A rotation starts with an L or R which indicates whether the rotation should be to the left (toward lower numbers) or to the right (toward higher numbers). Then, the rotation has a distance value which indicates how many clicks the dial should be rotated in that direction.

So, if the dial were pointing at 11, a rotation of R8 would cause the dial to point at 19. After that, a rotation of L19 would cause it to point at 0.

Because the dial is a circle, turning the dial left from 0 one click makes it point at 99. Similarly, turning the dial right from 99 one click makes it point at 0.

So, if the dial were pointing at 5, a rotation of L10 would cause it to point at 95. After that, a rotation of R5 could cause it to point at 0.

The dial starts by pointing at 50.

You could follow the instructions, but your recent required official North Pole secret entrance security training seminar taught you that the safe is actually a decoy. The actual password is the number of times the dial is left pointing at 0 after any rotation in the sequence.

For example, suppose the attached document contained the following rotations:

```
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
```

Following these rotations would cause the dial to move as follows:

- The dial starts by pointing at 50.
- The dial is rotated L68 to point at 82.
- The dial is rotated L30 to point at 52.
- The dial is rotated R48 to point at 0.
- The dial is rotated L5 to point at 95.
- The dial is rotated R60 to point at 55.
- The dial is rotated L55 to point at 0.
- The dial is rotated L1 to point at 99.
- The dial is rotated L99 to point at 0.
- The dial is rotated R14 to point at 14.
- The dial is rotated L82 to point at 32.
Because the dial points at 0 a total of three times during this process, the password in this example is 3.

Analyze the rotations in your attached document. What's the actual password to open the door?

To begin, get your puzzle input.

---

# Answer

I'll explain this Rust code line by line:

## Main Function

```rust
use std::fs;
```

Imports the file system module from Rust's standard library for reading files.

```rust
fn main() {
```

Entry point of the program.

```rust
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input.txt");
```

Reads the entire contents of "input.txt" into a `String`. The `expect()` will panic with the error message if the file can't be read.

```rust
    let password = solve(&input);
```

Calls the `solve` function with a reference to the input string and stores the result.

```rust
    println!("The password is: {}", password);
```

Prints the password to stdout.

## Solve Function

```rust
fn solve(input: &str) -> i32 {
```

Defines the solving function that takes a string slice and returns a 32-bit integer.

```rust
    let mut position = 50;
```

Initializes the dial position to 50 (mutable variable).

```rust
    let mut zero_count = 0;
```

Counter for how many times the dial lands on 0.

```rust
    for line in input.lines() {
```

Iterates over each line in the input string.

```rust
        let line = line.trim();
```

Removes leading/trailing whitespace from the line.

```rust
        if line.is_empty() {
            continue;
        }
```

Skips empty lines.

```rust
        let direction = line.chars().next().unwrap();
```

Gets the first character (`'L'` or `'R'`). `chars()` returns an iterator over characters, `next()` gets the first one, `unwrap()` extracts it (panics if None).

```rust
        let distance: i32 = line[1..].parse().expect("Failed to parse distance");
```

Takes the substring from index 1 to the end (`[1..]`), parses it as an `i32`, and panics with an error message if parsing fails.

```rust
        position = match direction {
```

Pattern matching on the direction character to calculate new position.

```rust
            'L' => (position - distance).rem_euclid(100),
```

For left rotation: subtract distance and use `rem_euclid(100)` for proper modular arithmetic (handles negatives correctly by always returning 0-99).

```rust
            'R' => (position + distance) % 100,
```

For right rotation: add distance and use modulo 100 to wrap around.

```rust
            _ => panic!("Invalid direction: {}", direction),
```

Catch-all pattern that panics if direction isn't 'L' or 'R'.

```rust
        if position == 0 {
            zero_count += 1;
        }
```

If the dial lands on 0, increment the counter.

```rust
    zero_count
```

Returns the final count (implicit return without semicolon).

## Tests Section

```rust
#[cfg(test)]
mod tests {
```

Conditional compilation attribute - this module only compiles during testing.

```rust
    use super::*;
```

Imports everything from the parent module (the `solve` function).

```rust
    #[test]
    fn test_example() {
```

Marks this function as a test case.

```rust
        let input = "L68\nL30\n...";
```

Multi-line string literal with the example input.

```rust
        assert_eq!(solve(input), 3);
```

Asserts that solving the example input returns 3 (the expected answer).

The second test (`test_rotation_logic`) similarly validates the rotation math with specific examples to ensure the modular arithmetic works correctly for both directions and wrapping scenarios.

**Key Insight**: `rem_euclid()` is crucial for left rotations because regular `%` in Rust can return negative values, but `rem_euclid()` always returns a positive remainder in the range [0, 100).

---

# Part 2

You're sure that's the right password, but the door won't open. You knock, but nobody answers. You build a snowman while you think.

As you're rolling the snowballs for your snowman, you find another security document that must have fallen into the snow:

"Due to newer security protocols, please use password method 0x434C49434B until further notice."

You remember from the training seminar that "method 0x434C49434B" means you're actually supposed to count the number of times any click causes the dial to point at 0, regardless of whether it happens during a rotation or at the end of one.

Following the same rotations as in the above example, the dial points at zero a few extra times during its rotations:

- The dial starts by pointing at 50.
- The dial is rotated L68 to point at 82; during this rotation, it points at 0 once.
- The dial is rotated L30 to point at 52.
- The dial is rotated R48 to point at 0.
- The dial is rotated L5 to point at 95.
- The dial is rotated R60 to point at 55; during this rotation, it points at 0 once.
- The dial is rotated L55 to point at 0.
- The dial is rotated L1 to point at 99.
- The dial is rotated L99 to point at 0.
- The dial is rotated R14 to point at 14.
- The dial is rotated L82 to point at 32; during this rotation, it points at 0 once.
In this example, the dial points at 0 three times at the end of a rotation, plus three more times during a rotation. So, in this example, the new password would be 6.

Be careful: if the dial were pointing at 50, a single rotation like R1000 would cause the dial to point at 0 ten times before returning back to 50!

Using password method 0x434C49434B, what is the password to open the door?

---

# Answer
