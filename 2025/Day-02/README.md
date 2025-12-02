# --- Day 2: Gift Shop ---

## Part 1

You get inside and take the elevator to its only other stop: the gift shop. "Thank you for visiting the North Pole!" gleefully exclaims a nearby sign. You aren't sure who is even allowed to visit the North Pole, but you know you can access the lobby through here, and from there you can access the rest of the North Pole base.

As you make your way through the surprisingly extensive selection, one of the clerks recognizes you and asks for your help.

As it turns out, one of the younger Elves was playing on a gift shop computer and managed to add a whole bunch of invalid product IDs to their gift shop database! Surely, it would be no trouble for you to identify the invalid product IDs for them, right?

They've even checked most of the product ID ranges already; they only have a few product ID ranges (your puzzle input) that you'll need to check. For example:

```
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124
```

(The ID ranges are wrapped here for legibility; in your input, they appear on a single long line.)

The ranges are separated by commas (,); each range gives its first ID and last ID separated by a dash (-).

Since the young Elf was just doing silly patterns, you can find the invalid IDs by looking for any ID which is made only of some sequence of digits repeated twice. So, 55 (5 twice), 6464 (64 twice), and 123123 (123 twice) would all be invalid IDs.

None of the numbers have leading zeroes; 0101 isn't an ID at all. (101 is a valid ID that you would ignore.)

Your job is to find all of the invalid IDs that appear in the given ranges. In the above example:

- 11-22 has two invalid IDs, 11 and 22.
- 95-115 has one invalid ID, 99.
- 998-1012 has one invalid ID, 1010.
- 1188511880-1188511890 has one invalid ID, 1188511885.
- 222220-222224 has one invalid ID, 222222.
- 1698522-1698528 contains no invalid IDs.
- 446443-446449 has one invalid ID, 446446.
- 38593856-38593862 has one invalid ID, 38593859.
The rest of the ranges contain no invalid IDs.
Adding up all the invalid IDs in this example produces 1227775554.

What do you get if you add up all of the invalid IDs?

To begin, get your puzzle input.

---

## Answer (Part 1)

### Why `u64` (unsigned 64-bit integer) was chosen?

## **1. Range Requirements**

Looking at the input data:

```rust
// From the input file:
3737324037-3737408513
4955694516-4955781763
9494926669-9494965937
9939271919-9939349036
```

These numbers are **billions** (10-digit numbers). Let's check what fits:

| Type | Max Value | Fits? |
|------|-----------|-------|
| `u32` | 4,294,967,295 | Too small (can't hold 9,939,271,919) |
| `i32` | 2,147,483,647 | Even smaller |
| `u64` | 18,446,744,073,709,551,615 | ✅ Plenty of room |
| `i64` | 9,223,372,036,854,775,807 | ✅ Would also work |

## **2. Why Unsigned (`u64`) vs Signed (`i64`)?**

**Unsigned was chosen because:**

- Product IDs are never negative (you can't have ID `-5`)
- Gives you twice the positive range compared to signed
- The problem statement says ranges start from `1` (no negatives mentioned)

```rust
// u64 max: 18,446,744,073,709,551,615
// i64 max:  9,223,372,036,854,775,807
```

## **3. Why Not Smaller Types?**

```rust
// If they used u32:
let id: u32 = 9939271919; // Compile error: literal out of range

// With u64:
let id: u64 = 9939271919; // Works fine
```

## **4. The Sum Also Needs u64**

```rust
let mut total = 0u64;
```

The final sum could be very large too. If you're adding millions of billion-sized numbers, you need a large type to hold the result.

From the example:

```
Sum = 1,227,775,554
```

In the actual puzzle, the sum could be much larger.

## **Summary**

`u64` was chosen because:

1. Input ranges go into the billions
2. No negative IDs needed
3. Sum of invalid IDs could be huge
4. Safe choice with plenty of headroom

They *could* have used `i64` (signed), but `u64` is more semantically correct since IDs can't be negative.

## **1. Import and Main Function**

```rust
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input file");

    let sum = solve(&input);
    println!("Sum of all invalid IDs: {}", sum);
}
```

- `use std::fs` - imports file system module
- `fs::read_to_string("input.txt")` - reads entire file into a String
- `.expect()` - crashes program with error message if file reading fails
- `solve(&input)` - passes a reference to the input string to solve function
- `println!` - prints the result

## **2. The Solve Function**

```rust
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
```

- `input: &str` - takes a string reference as parameter
- `-> u64` - returns an unsigned 64-bit integer
- `input.trim()` - removes whitespace from beginning/end
- `let mut total = 0u64` - creates mutable variable initialized to 0
- `for (start, end) in ranges` - loops through each range tuple
- `start..=end` - inclusive range (includes both start and end)
- Checks each ID, adds to total if invalid
- Returns the total sum

## **3. Parse Ranges Function**

```rust
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
```

- `-> Vec<(u64, u64)>` - returns vector of tuples (start, end)
- `.split(',')` - splits input by commas: `"11-22,95-115"` → `["11-22", "95-115"]`
- `.filter_map()` - combines filter and map (transforms and removes None values)
- `range.split('-')` - splits each range by dash: `"11-22"` → `["11", "22"]`
- `.collect()` - collects iterator into Vec
- `parts[0].parse::<u64>()` - converts string to u64 number
- `.ok()?` - converts Result to Option, returns None on error (the `?` early-returns None)
- `Some((start, end))` - wraps tuple in Some if parsing succeeds
- Final `.collect()` - collects all Some values into Vec

## **4. Is Invalid ID Function**

```rust
fn is_invalid_id(id: u64) -> bool {
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
```

- `id.to_string()` - converts number to string (e.g., `123` → `"123"`)
- `s.len()` - gets string length
- `len % 2 != 0` - checks if odd length (can't split evenly)
- `let mid = len / 2` - finds midpoint (e.g., length 6 → mid = 3)
- `&s[..mid]` - string slice from start to mid (e.g., `"123123"[..3]` = `"123"`)
- `&s[mid..]` - string slice from mid to end (e.g., `"123123"[3..]` = `"123"`)
- `first_half == second_half` - returns true if both halves match

**Example:** For `123123`:

- `s = "123123"`, `len = 6`
- `mid = 3`
- `first_half = "123"`, `second_half = "123"`
- Returns `true`

## **5. Tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_invalid_id() {
        assert!(is_invalid_id(11));  // true
        assert!(!is_invalid_id(101)); // false
    }
}
```

- `#[cfg(test)]` - only compiles when running tests
- `use super::*` - imports all items from parent module
- `#[test]` - marks function as a test
- `assert!()` - panics if condition is false
- Run with `cargo test`

# Break down of `collect()`

`.collect()` in Rust is a powerful method that **gathers items from an iterator into a collection**.

## **Basic Concept**

```rust
// Iterator → collect() → Collection
[1, 2, 3].iter()  // Creates iterator
    .map(|x| x * 2)  // Still an iterator
    .collect()  // Materializes into Vec<i32>
```

Think of it like: "I've been lazily processing items one-by-one, now **collect** them all into a concrete data structure."

## **In This Code**

### **Example 1: In `parse_ranges`**

```rust
fn parse_ranges(input: &str) -> Vec<(u64, u64)> {
    input
        .split(',')           // Iterator of &str
        .filter_map(|range| { // Iterator of Option<(u64, u64)>
            // ... parsing logic ...
            Some((start, end))
        })
        .collect()            // Vec<(u64, u64)>
}
```

**Step-by-step:**

```rust
let input = "11-22,95-115,998-1012";

// After split(','):
// Iterator: ["11-22", "95-115", "998-1012"]

// After filter_map:
// Iterator: [Some((11, 22)), Some((95, 115)), Some((998, 1012))]

// After collect():
// Vec: [(11, 22), (95, 115), (998, 1012)]
```

### **Example 2: Earlier in parse_ranges**

```rust
let parts: Vec<&str> = range.split('-').collect();
```

```rust
let range = "11-22";

// split('-') creates iterator: ["11", "22"]
// collect() gathers into: Vec ["11", "22"]
```

## **Why Use collect()?**

### **1. Iterators are Lazy**

```rust
let numbers = vec![1, 2, 3, 4, 5];

// This does NOTHING yet (lazy):
let doubled = numbers.iter().map(|x| x * 2);

// This actually computes:
let result: Vec<i32> = doubled.collect();
// result = [2, 4, 6, 8, 10]
```

### **2. Type Flexibility**

`collect()` can create different types:

```rust
let text = "hello";

// Collect into Vec:
let chars_vec: Vec<char> = text.chars().collect();
// ['h', 'e', 'l', 'l', 'o']

// Collect into String:
let chars_string: String = text.chars().collect();
// "hello"

// Collect into HashSet:
use std::collections::HashSet;
let chars_set: HashSet<char> = text.chars().collect();
// {'h', 'e', 'l', 'o'}
```

Rust knows what to collect into based on the **type annotation** or **context**.

## **How Rust Knows What to Collect Into**

### **Method 1: Type Annotation**

```rust
let result: Vec<i32> = (1..5).collect();
//          ^^^^^^^^ -- Rust sees this and knows to make Vec
```

### **Method 2: Turbofish Syntax**

```rust
let result = (1..5).collect::<Vec<i32>>();
//                          ^^^^^^^^^^^ -- Explicit type
```

### **Method 3: Context Inference**

```rust
fn parse_ranges(input: &str) -> Vec<(u64, u64)> {
    //                          ^^^^^^^^^^^^^^^ Return type tells collect() what to make
    input.split(',').collect()
}
```

## **Common Patterns**

### **Split and Collect**

```rust
let csv = "apple,banana,cherry";
let fruits: Vec<&str> = csv.split(',').collect();
// ["apple", "banana", "cherry"]
```

### **Filter and Collect**

```rust
let numbers = vec![1, 2, 3, 4, 5];
let evens: Vec<i32> = numbers.into_iter()
    .filter(|x| x % 2 == 0)
    .collect();
// [2, 4]
```

### **Map and Collect**

```rust
let numbers = vec![1, 2, 3];
let doubled: Vec<i32> = numbers.iter()
    .map(|x| x * 2)
    .collect();
// [2, 4, 6]
```

## **Key Points**

1. **Consumes the iterator** - Once you collect, the iterator is gone
2. **Creates a new collection** - allocates memory
3. **Type-driven** - Rust figures out what to collect into based on type hints
4. **Versatile** - Works with Vec, String, HashSet, HashMap, and more

```rust
// Without collect - lazy, nothing computed:
let iter = (1..100).map(|x| x * 2);

// With collect - eager, everything computed:
let vec: Vec<i32> = (1..100).map(|x| x * 2).collect();
```

---

# --- Part Two ---

The clerk quickly discovers that there are still invalid IDs in the ranges in your list. Maybe the young Elf was doing other silly patterns as well?

Now, an ID is invalid if it is made only of some sequence of digits repeated at least twice. So, 12341234 (1234 two times), 123123123 (123 three times), 1212121212 (12 five times), and 1111111 (1 seven times) are all invalid IDs.

From the same example as before:

- 11-22 still has two invalid IDs, 11 and 22.
- 95-115 now has two invalid IDs, 99 and 111.
- 998-1012 now has two invalid IDs, 999 and 1010.
- 1188511880-1188511890 still has one invalid ID, 1188511885.
- 222220-222224 still has one invalid ID, 222222.
- 1698522-1698528 still contains no invalid IDs.
- 446443-446449 still has one invalid ID, 446446.
- 38593856-38593862 still has one invalid ID, 38593859.
- 565653-565659 now has one invalid ID, 565656.
- 824824821-824824827 now has one invalid ID, 824824824.
- 2121212118-2121212124 now has one invalid ID, 2121212121.
Adding up all the invalid IDs in this example produces 4174379265.

What do you get if you add up all of the invalid IDs using these new rules?

---

# Answer (part 2)

## The Key Difference from Part 1

**Part 1:** Invalid IDs are sequences repeated **exactly twice**
- `6464` = "64" repeated 2 times ✓
- `111` = "1" repeated 3 times ✗ (not exactly 2)

**Part 2:** Invalid IDs are sequences repeated **at least twice**
- `6464` = "64" repeated 2 times ✓
- `111` = "1" repeated 3 times ✓
- `12341234` = "1234" repeated 2 times ✓
- `123123123` = "123" repeated 3 times ✓
- `1111111` = "1" repeated 7 times ✓

## The Algorithm

### Core Logic: Finding Repeating Patterns

```rust
fn is_invalid_id_part2(id: u64) -> bool {
    let s = id.to_string();
    let len = s.len();

    // Try all possible pattern lengths from 1 to len/2
    for pattern_len in 1..=(len / 2) {
        if len % pattern_len == 0 {
            let repetitions = len / pattern_len;
            if repetitions >= 2 {
                let pattern = &s[..pattern_len];
                // Check if entire string is this pattern repeated
                if is_match {
                    return true;
                }
            }
        }
    }
    false
}
```

### Step-by-Step Walkthrough

Let's trace through example: **`565656`** (which should be invalid)

#### Step 1: Convert to String
```rust
let s = id.to_string();  // "565656"
let len = s.len();       // 6
```

#### Step 2: Try All Pattern Lengths

We try pattern lengths from **1 to 3** (half of 6):

##### Attempt 1: `pattern_len = 1`
```rust
len % pattern_len = 6 % 1 = 0  ✓ (divisible)
repetitions = 6 / 1 = 6        ✓ (at least 2)
pattern = "5"                  (first character)
```

Check if "565656" = "5" repeated 6 times:
```
"565656" = "5" + "6" + "5" + "6" + "5" + "6"
```
- First chunk "5" == "5" ✓
- Second chunk "6" == "5" ✗

**Not a match, continue...**

##### Attempt 2: `pattern_len = 2`
```rust
len % pattern_len = 6 % 2 = 0  ✓ (divisible)
repetitions = 6 / 2 = 3        ✓ (at least 2)
pattern = "56"                 (first 2 characters)
```

Check if "565656" = "56" repeated 3 times:
```
"565656" = "56" + "56" + "56"
```
- First chunk "56" == "56" ✓
- Second chunk "56" == "56" ✓
- Third chunk "56" == "56" ✓

**MATCH FOUND! Return true** ✓

### Another Example: **`123123123`**

```rust
s = "123123123"
len = 9
```

Try pattern lengths 1 to 4:

**pattern_len = 1:**
```
9 % 1 = 0 ✓
repetitions = 9 ✓
pattern = "1"
Check: "1"+"2"+"3"+"1"+"2"+"3"+"1"+"2"+"3"
"2" != "1" ✗
```

**pattern_len = 3:**
```
9 % 3 = 0 ✓
repetitions = 3 ✓
pattern = "123"
Check: "123" + "123" + "123"
All chunks match! ✓ FOUND
```

### Example: **`1698522`** (should be valid)

```rust
s = "1698522"
len = 7
```

Try pattern lengths 1 to 3:

**pattern_len = 1:**
```
7 % 1 = 0 ✓
repetitions = 7 ✓
pattern = "1"
Check: "1"+"6"+"9"+"8"+"5"+"2"+"2"
"6" != "1" ✗
```

**pattern_len = 2:**
```
7 % 2 = 1 ✗ (not divisible)
Skip
```

**pattern_len = 3:**
```
7 % 3 = 1 ✗ (not divisible)
Skip
```

**No patterns found, return false** (valid ID)

## The Pattern Matching Code

```rust
let is_match = s.chars()
    .collect::<Vec<_>>()
    .chunks(pattern_len)
    .all(|chunk| {
        let chunk_str: String = chunk.iter().collect();
        chunk_str == pattern
    });
```

### Breaking This Down

#### Step 1: Convert string to characters
```rust
s.chars().collect::<Vec<_>>()
// "565656" → ['5', '6', '5', '6', '5', '6']
```

#### Step 2: Split into chunks
```rust
.chunks(pattern_len)
// If pattern_len = 2:
// [['5', '6'], ['5', '6'], ['5', '6']]
```

#### Step 3: Check all chunks match pattern
```rust
.all(|chunk| {
    let chunk_str: String = chunk.iter().collect();
    chunk_str == pattern
})
```

For each chunk:
- Convert back to string: `['5', '6']` → `"56"`
- Compare with pattern: `"56" == "56"` ✓

If **all** chunks match, return `true`

## Examples from Part 2

Let's verify the example results:

### Range: `95-115`

**Part 1:** Only `99` is invalid (9 repeated twice)

**Part 2:** `99` AND `111` are invalid
- `99` = "9" × 2 ✓
- `111` = "1" × 3 ✓

### Range: `998-1012`

**Part 1:** Only `1010` is invalid

**Part 2:** `999` AND `1010` are invalid
- `999` = "9" × 3 ✓
- `1010` = "10" × 2 ✓

### Range: `565653-565659`

**Part 1:** No invalid IDs

**Part 2:** `565656` is invalid
- `565656` = "56" × 3 ✓

### Range: `824824821-824824827`

**Part 1:** No invalid IDs

**Part 2:** `824824824` is invalid
- `824824824` = "824" × 3 ✓

### Range: `2121212118-2121212124`

**Part 1:** No invalid IDs

**Part 2:** `2121212121` is invalid
- `2121212121` = "21" × 5 ✓

## Why This Algorithm Works

### Efficient Pattern Detection

Instead of trying every possible substring, we:
1. Only try pattern lengths that **divide evenly** into the total length
2. Stop at the first match (early return)
3. Use Rust's iterator methods for clean, efficient checking

### Mathematical Insight

For a number to be a repeating pattern:
- Length must be divisible by pattern length
- Example: `123123` (length 6) could be patterns of length 1, 2, 3, or 6
- We only need to check 1, 2, 3 (up to half)

### Edge Cases Handled

```rust
// Single digit: pattern_len = 1..=(1/2) = 1..=0 → empty range
// Returns false immediately ✓

// Two digits like 11:
// Try pattern_len = 1: "1" × 2 ✓

// Three digits like 111:
// Try pattern_len = 1: "1" × 3 ✓

// Prime length like 7 digits: 1234567
// Only tries pattern_len = 1, 2, 3
// 7 % 2 ≠ 0, 7 % 3 ≠ 0, only 1 works
// Would need all same digit to match ✓
```

## Complexity Analysis

**Time Complexity:** O(n²) where n is the number of digits
- Outer loop: O(n) pattern lengths to try
- Inner loop: O(n) to check all chunks

**Space Complexity:** O(n)
- Convert number to string
- Create character vector

For the input ranges (numbers up to 10 digits), this is very fast!

## Running the Solution

```bash
cd 2025/Day-02
cargo test   # Should pass all tests
cargo run    # Get answers for both parts
```

Expected output:
```
Part 1 - Sum of all invalid IDs: [your answer]
Part 2 - Sum of all invalid IDs: [your answer]
```
