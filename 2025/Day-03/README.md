# --- Day 3: Lobby ---

## Part 1

You descend a short staircase, enter the surprisingly vast lobby, and are quickly cleared by the security checkpoint. When you get to the main elevators, however, you discover that each one has a red light above it: they're all offline.

"Sorry about that," an Elf apologizes as she tinkers with a nearby control panel. "Some kind of electrical surge seems to have fried them. I'll try to get them online soon."

You explain your need to get further underground. "Well, you could at least take the escalator down to the printing department, not that you'd get much further than that without the elevators working. That is, you could if the escalator weren't also offline."

"But, don't worry! It's not fried; it just needs power. Maybe you can get it running while I keep working on the elevators."

There are batteries nearby that can supply emergency power to the escalator for just such an occasion. The batteries are each labeled with their joltage rating, a value from 1 to 9. You make a note of their joltage ratings (your puzzle input). For example:

```
987654321111111
811111111111119
234234234234278
818181911112111
```

The batteries are arranged into banks; each line of digits in your input corresponds to a single bank of batteries. Within each bank, you need to turn on exactly two batteries; the joltage that the bank produces is equal to the number formed by the digits on the batteries you've turned on. For example, if you have a bank like 12345 and you turn on batteries 2 and 4, the bank would produce 24 jolts. (You cannot rearrange batteries.)

You'll need to find the largest possible joltage each bank can produce. In the above example:

In 987654321111111, you can make the largest joltage possible, 98, by turning on the first two batteries.
In 811111111111119, you can make the largest joltage possible by turning on the batteries labeled 8 and 9, producing 89 jolts.
In 234234234234278, you can make 78 by turning on the last two batteries (marked 7 and 8).
In 818181911112111, the largest joltage you can produce is 92.
The total output joltage is the sum of the maximum joltage from each bank, so in this example, the total output joltage is 98 + 89 + 78 + 92 = 357.

There are many batteries in front of you. Find the maximum joltage possible from each bank; what is the total output joltage?

---

# Answer (Part 1)

## Problem Summary

Given battery banks (lines of digits 1-9), find the maximum two-digit number you can form from each bank by selecting exactly two batteries in order, then sum all maximums.

**Key Constraint:** You cannot rearrange batteries - the first battery you pick must come before the second in the original sequence.

## The Algorithm

### Core Strategy: Try All Pairs

For each battery bank, we need to:

1. Try every possible pair of batteries (i, j) where i < j
2. Form a two-digit number: `digit[i] * 10 + digit[j]`
3. Track the maximum value found

### Implementation

```rust
fn find_max_joltage(bank: &str) -> u32 {
    let digits: Vec<char> = bank.chars().collect();
    let mut max_joltage = 0;

    // Try all pairs where i < j
    for i in 0..digits.len() {
        for j in (i + 1)..digits.len() {
            let joltage = digits[i].to_digit(10).unwrap() * 10
                        + digits[j].to_digit(10).unwrap();
            max_joltage = max_joltage.max(joltage);
        }
    }

    max_joltage
}
```

## Step-by-Step Walkthrough

### Example 1: `987654321111111`

Let's trace through finding the maximum:

**Bank:** `987654321111111` (15 digits)

#### Some pairs to consider

| Position i | Position j | digit[i] | digit[j] | Joltage | Comment |
|------------|------------|----------|----------|---------|---------|
| 0 | 1 | 9 | 8 | **98** | First two batteries |
| 0 | 2 | 9 | 7 | 97 | |
| 0 | 3 | 9 | 6 | 96 | |
| 1 | 2 | 8 | 7 | 87 | |
| 2 | 3 | 7 | 6 | 76 | |
| ... | ... | ... | ... | ... | Many more pairs... |
| 13 | 14 | 1 | 1 | 11 | Last two batteries |

**Analysis:**

- The first digit is 9 (highest possible)
- The second digit is 8 (second highest in remaining positions)
- No other pair can beat 98 because:
  - Any pair starting with 8 or less: max is 89
  - Any pair starting with 9 but not followed by 8: max is 97

**Maximum: 98** ✓

### Example 2: `811111111111119`

**Bank:** `811111111111119` (15 digits)

#### Key pairs

| Position i | Position j | digit[i] | digit[j] | Joltage | Comment |
|------------|------------|----------|----------|---------|---------|
| 0 | 1 | 8 | 1 | 81 | First and second |
| 0 | 2 | 8 | 1 | 81 | First and third |
| 0 | 14 | 8 | 9 | **89** | First and last |
| 1 | 14 | 1 | 9 | 19 | Second and last |
| 13 | 14 | 1 | 9 | 19 | Second-to-last and last |

**Analysis:**

- We have 8 at position 0 and 9 at position 14
- Best strategy: Pick 8 first, then 9
- This gives us 89, the maximum possible

**Maximum: 89** ✓

### Example 3: `234234234234278`

**Bank:** `234234234234278` (15 digits)

#### Some important pairs

| Position i | Position j | digit[i] | digit[j] | Joltage | Comment |
|------------|------------|----------|----------|---------|---------|
| 0 | 1 | 2 | 3 | 23 | Start |
| 1 | 2 | 3 | 4 | 34 | |
| 12 | 13 | 7 | 8 | **78** | Second-to-last and last |
| 12 | 14 | 7 | 8 | 78 | Third-to-last and last |
| 13 | 14 | 8 | 8 | 88 | Wait... |

**Wait! Let me recount:**

- `234234234234278` has positions 0-14
- Position 12: `2`
- Position 13: `7`
- Position 14: `8`

Actually checking more carefully:

```
2 3 4 2 3 4 2 3 4 2 3 4 2 7 8
0 1 2 3 4 5 6 7 8 9 10 11 12 13 14
```

| Position i | Position j | digit[i] | digit[j] | Joltage | Comment |
|------------|------------|----------|----------|---------|---------|
| 13 | 14 | 7 | 8 | **78** | Last two |
| 12 | 14 | 2 | 8 | 28 | |
| 12 | 13 | 2 | 7 | 27 | |

**Maximum: 78** ✓

### Example 4: `818181911112111`

**Bank:** `818181911112111` (15 digits)

```
8 1 8 1 8 1 9 1 1 1 1 2 1 1 1
0 1 2 3 4 5 6 7 8 9 10 11 12 13 14
```

#### Key pairs

| Position i | Position j | digit[i] | digit[j] | Joltage | Comment |
|------------|------------|----------|----------|---------|---------|
| 0 | 1 | 8 | 1 | 81 | First two |
| 0 | 6 | 8 | 9 | 89 | First 8 and the 9 |
| 2 | 6 | 8 | 9 | 89 | Second 8 and the 9 |
| 4 | 6 | 8 | 9 | 89 | Third 8 and the 9 |
| 6 | 11 | 9 | 2 | **92** | The 9 and the 2 |
| 6 | 7 | 9 | 1 | 91 | The 9 and next 1 |

**Analysis:**

- We have a 9 at position 6
- After the 9, we have several 1s and a 2 at position 11
- Picking 9 then 2 gives us 92
- This beats 89 (8 then 9)

**Maximum: 92** ✓

## Why This Algorithm Works

### Brute Force is Perfect Here

**Time Complexity:** O(n²) where n is the length of each bank

- For a bank with 15 digits: 15 × 14 / 2 = 105 pairs to check
- For a bank with 100 digits: 100 × 99 / 2 = 4,950 pairs

This is very fast for the input size!

### Space Complexity:** O(n)

- We store the digits in a vector
- No other significant memory usage

### Why Not Greedy?

You might think: "Just pick the largest digit, then the largest digit after it."

**This doesn't work!**

Counter-example: `918`

- Greedy: Pick 9 (position 0), then pick 1 (position 1) = 91
- But actually: Pick 1 (position 1), then pick 8 (position 2) = 18... wait no
- Actually: Pick 9 (position 0), then pick 8 (position 2) = 98 ✓

Wait, greedy works here. Let me think of a better counter-example:

Counter-example: `1928`

- Greedy approach 1: Pick largest first (9), then largest after (8) = 98 ✓
- This actually works!

Actually, a **smarter greedy approach** would work:

- For each starting digit, find the maximum digit that comes after it
- Pick the best combination

But the brute force approach is:

1. Simpler to understand and implement
2. Fast enough for the problem
3. Guaranteed to be correct

## The Complete Solution

```rust
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input file");

    let sum = solve(&input);
    println!("Part 1 - Total output joltage: {}", sum);
}

fn solve(input: &str) -> u32 {
    input
        .lines()
        .map(|line| find_max_joltage(line))
        .sum()
}

fn find_max_joltage(bank: &str) -> u32 {
    let digits: Vec<char> = bank.chars().collect();
    let mut max_joltage = 0;

    for i in 0..digits.len() {
        for j in (i + 1)..digits.len() {
            let joltage = digits[i].to_digit(10).unwrap() * 10
                        + digits[j].to_digit(10).unwrap();
            max_joltage = max_joltage.max(joltage);
        }
    }

    max_joltage
}
```

## Verification

For the example input:

```
987654321111111 → 98
811111111111119 → 89
234234234234278 → 78
818181911112111 → 92
                  ----
Total:            357 ✓
```

## Running the Solution

```bash
cd 2025/Day-03
cargo test  # Run tests to verify correctness
cargo run   # Get answer for your input
```

## Potential Optimizations (Not Needed)

If the banks were much longer (millions of digits), we could optimize:

1. **Early termination:** If we find a pair of 99, stop searching
2. **Smart search:** For each digit d, only search for digits ≥ (99 - d*10) after it
3. **Index by value:** Pre-compute positions of each digit value

But for this problem, the simple O(n²) solution is perfect!

---

# --- Part Two ---

The escalator doesn't move. The Elf explains that it probably needs more joltage to overcome the static friction of the system and hits the big red "joltage limit safety override" button. You lose count of the number of times she needs to confirm "yes, I'm sure" and decorate the lobby a bit while you wait.

Now, you need to make the largest joltage by turning on exactly twelve batteries within each bank.

The joltage output for the bank is still the number formed by the digits of the batteries you've turned on; the only difference is that now there will be 12 digits in each bank's joltage output instead of two.

Consider again the example from before:

```
987654321111111
811111111111119
234234234234278
818181911112111
```

Now, the joltages are much larger:

In 987654321111111, the largest joltage can be found by turning on everything except some 1s at the end to produce 987654321111.
In the digit sequence 811111111111119, the largest joltage can be found by turning on everything except some 1s, producing 811111111119.
In 234234234234278, the largest joltage can be found by turning on everything except a 2 battery, a 3 battery, and another 2 battery near the start to produce 434234234278.
In 818181911112111, the joltage 888911112111 is produced by turning on everything except some 1s near the front.
The total output joltage is now much larger: 987654321111 + 811111111119 + 434234234278 + 888911112111 = 3121910778619.

What is the new total output joltage?

---

# Answer (Part 2)

## Problem Summary

Now we need to select exactly **12 batteries** from each bank to form the maximum 12-digit number, instead of just 2 batteries.

**Key Insight:** To maximize a 12-digit number, we want the largest possible digits in the leftmost positions.

## The Algorithm: Greedy Selection

### Core Strategy

At each position in our 12-digit result:
1. Pick the **largest available digit** from the remaining batteries
2. Ensure we leave **enough batteries** after this choice to fill the remaining positions

### Why Greedy Works Here

Unlike some problems, greedy is **optimal** for this task because:
- We're building the number left-to-right (most significant to least significant)
- A larger digit in a more significant position always beats a smaller digit there
- We only need to ensure we don't "run out" of batteries for remaining positions

### Implementation

```rust
fn find_max_joltage_part2(bank: &str) -> u64 {
    let digits: Vec<char> = bank.chars().collect();
    let n = digits.len();
    let target_count = 12;
    let mut selected_indices = Vec::new();

    for result_pos in 0..target_count {
        let remaining_needed = target_count - result_pos - 1;
        let mut best_digit = '0';
        let mut best_idx = 0;

        let start_idx = if selected_indices.is_empty() {
            0
        } else {
            selected_indices.last().unwrap() + 1
        };

        // Search range: must leave enough batteries for remaining positions
        for idx in start_idx..=(n - remaining_needed - 1) {
            if digits[idx] > best_digit {
                best_digit = digits[idx];
                best_idx = idx;
            }
        }

        selected_indices.push(best_idx);
    }

    // Convert selected indices to number
    let result_str: String = selected_indices.iter()
        .map(|&idx| digits[idx])
        .collect();

    result_str.parse::<u64>().unwrap()
}
```

## Step-by-Step Walkthrough

### Example 1: `987654321111111`

**Bank:** `987654321111111` (15 digits)
**Target:** 12 digits

```
Digits:  9 8 7 6 5 4 3 2 1 1 1 1 1 1 1
Indices: 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14
```

Building the 12-digit number:

| Result Pos | Remaining Needed | Search Range | Best Digit | Best Index |
|------------|------------------|--------------|------------|------------|
| 0 | 11 | 0..3 | 9 | 0 |
| 1 | 10 | 1..4 | 8 | 1 |
| 2 | 9 | 2..5 | 7 | 2 |
| 3 | 8 | 3..6 | 6 | 3 |
| 4 | 7 | 4..7 | 5 | 4 |
| 5 | 6 | 5..8 | 4 | 5 |
| 6 | 5 | 6..9 | 3 | 6 |
| 7 | 4 | 7..10 | 2 | 7 |
| 8 | 3 | 8..11 | 1 | 8 |
| 9 | 2 | 9..12 | 1 | 9 |
| 10 | 1 | 10..13 | 1 | 10 |
| 11 | 0 | 11..14 | 1 | 11 |

**Result:** `987654321111` ✓

**Analysis:** We skip the last three 1s (indices 12, 13, 14) to get exactly 12 digits.

### Example 2: `811111111111119`

**Bank:** `811111111111119` (15 digits)

```
Digits:  8 1 1 1 1 1 1 1 1 1 1 1 1 1 9
Indices: 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14
```

| Result Pos | Search Range | Best Digit | Best Index |
|------------|--------------|------------|------------|
| 0 | 0..3 | 8 | 0 |
| 1 | 1..4 | 1 | 1 |
| 2 | 2..5 | 1 | 2 |
| ... | ... | 1 | ... |
| 10 | 10..13 | 1 | 10 |
| 11 | 11..14 | **9** | 14 |

**Result:** `811111111119` ✓

**Analysis:** We skip indices 11, 12, 13 (three 1s) to make room for the 9 at position 14.

### Example 3: `234234234234278`

**Bank:** `234234234234278` (15 digits)

```
Digits:  2 3 4 2 3 4 2 3 4 2 3 4 2 7 8
Indices: 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14
```

Greedy selection:
- Position 0: Range [0, 3], best is '4' at index 2
- Position 1: Range [3, 4], best is '3' at index 4
- Position 2: Range [5, 5], best is '4' at index 5
- Position 3: Range [6, 6], best is '2' at index 6
- Continue pattern...
- Position 11: Range [14, 14], best is '8' at index 14

**Result:** `434234234278` ✓

### Example 4: `818181911112111`

**Bank:** `818181911112111` (15 digits)

```
Digits:  8 1 8 1 8 1 9 1 1 1 1 2 1 1 1
Indices: 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14
```

Greedy selection:
- Pos 0: range [0, 3], best = '8' at 0
- Pos 1: range [1, 4], best = '8' at 2
- Pos 2: range [3, 5], best = '8' at 4
- Pos 3: range [5, 6], best = '9' at 6
- Pos 4-7: pick 1s at indices 7, 8, 9, 10
- Pos 8: range [11, 11], best = '2' at 11
- Pos 9-11: pick 1s at indices 12, 13, 14

**Result:** `888911112111` ✓

## Why This Algorithm Works

### Greedy is Optimal

**Theorem:** For maximizing a k-digit number by selecting k positions from n positions (k ≤ n), the greedy approach is optimal.

**Proof sketch:**
- Suppose greedy picks digit d₁ at position p₁ for the first position
- Any alternative picking d₂ < d₁ will produce a smaller number
- Even if the alternative makes better choices later, it can't overcome the difference in the most significant digit
- By induction, this applies to each subsequent position

### Time Complexity

**O(n × k)** where n = bank length, k = target count (12)
- For each of the 12 positions: O(k)
- We search through at most n positions: O(n)
- Total: O(12 × n) = O(n) for our purposes

### Space Complexity

**O(n + k)**
- Store digits: O(n)
- Store selected indices: O(k) = O(12)

## Verification

For the example input:
```
987654321111111 → 987654321111
811111111111119 → 811111111119
234234234234278 → 434234234278
818181911112111 → 888911112111
                   ----------------
Total:             3121910778619 ✓
```

## Running the Solution

```bash
cd 2025/Day-03
cargo test  # All tests should pass
cargo run   # Get both Part 1 and Part 2 answers
```
