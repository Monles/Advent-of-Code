# --- Day 5: Cafeteria ---

As the forklifts break through the wall, the Elves are delighted to discover that there was a cafeteria on the other side after all.

You can hear a commotion coming from the kitchen. "At this rate, we won't have any time left to put the wreaths up in the dining hall!" Resolute in your quest, you investigate.

"If only we hadn't switched to the new inventory management system right before Christmas!" another Elf exclaims. You ask what's going on.

The Elves in the kitchen explain the situation: because of their complicated new inventory management system, they can't figure out which of their ingredients are fresh and which are spoiled. When you ask how it works, they give you a copy of their database (your puzzle input).

The database operates on ingredient IDs. It consists of a list of fresh ingredient ID ranges, a blank line, and a list of available ingredient IDs. For example:

```
3-5
10-14
16-20
12-18

1
5
8
11
17
32
```

The fresh ID ranges are inclusive: the range 3-5 means that ingredient IDs 3, 4, and 5 are all fresh. The ranges can also overlap; an ingredient ID is fresh if it is in any range.

The Elves are trying to determine which of the available ingredient IDs are fresh. In this example, this is done as follows:

- Ingredient ID 1 is spoiled because it does not fall into any range.
- Ingredient ID 5 is fresh because it falls into range 3-5.
- Ingredient ID 8 is spoiled.
- Ingredient ID 11 is fresh because it falls into range 10-14.
- Ingredient ID 17 is fresh because it falls into range 16-20 as well as range 12-18.
- Ingredient ID 32 is spoiled.
So, in this example, 3 of the available ingredient IDs are fresh.

Process the database file from the new inventory management system. How many of the available ingredient IDs are fresh?

To begin, get your puzzle input.

---

# Answer (Part 1)

**513**

## Solution Approach

The problem requires:

1. **Parse fresh ID ranges** - Read lines like "3-5" and store as inclusive ranges
2. **Parse available ingredient IDs** - Read the list of ingredient IDs to check
3. **Check each ID against all ranges** - An ingredient is fresh if it falls within ANY range
4. **Count fresh ingredients** - Count how many IDs are fresh

### Algorithm

```rust
fn is_fresh(id: u64, ranges: &[(u64, u64)]) -> bool {
    for &(start, end) in ranges {
        if id >= start && id <= end {
            return true;
        }
    }
    false
}
```

For each ingredient ID, we check if it falls within any of the fresh ranges. The ranges are inclusive, so an ID is fresh if `start <= id <= end` for any range.

### Key Points

- Ranges can overlap (e.g., 16-20 and 12-18 both include 17)
- An ID only needs to be in ONE range to be considered fresh
- The input is split by a blank line: ranges first, then ingredient IDs

### Running the Solution

```bash
cargo run --release
```

The solution reads from `input.txt` and outputs the count of fresh ingredients.

---

# --- Part Two ---

The Elves start bringing their spoiled inventory to the trash chute at the back of the kitchen.

So that they can stop bugging you when they get new inventory, the Elves would like to know all of the IDs that the fresh ingredient ID ranges consider to be fresh. An ingredient ID is still considered fresh if it is in any range.

Now, the second section of the database (the available ingredient IDs) is irrelevant. Here are the fresh ingredient ID ranges from the above example:

3-5
10-14
16-20
12-18
The ingredient IDs that these ranges consider to be fresh are 3, 4, 5, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, and 20. So, in this example, the fresh ingredient ID ranges consider a total of 14 ingredient IDs to be fresh.

Process the database file again. How many ingredient IDs are considered to be fresh according to the fresh ingredient ID ranges?
---

# Answer (Part 2)

**339668510830757**

## Solution Approach

Part 2 requires counting all unique ingredient IDs that are covered by the fresh ID ranges, ignoring the available ingredient list.

### Key Insight

The ranges can overlap, so we need to:
1. **Merge overlapping ranges** to avoid double-counting IDs
2. **Count all IDs** in the merged ranges

### Algorithm

```rust
fn merge_ranges(ranges: &mut [(u64, u64)]) -> Vec<(u64, u64)> {
    // Sort ranges by start position
    ranges.sort_by_key(|&(start, _)| start);

    let mut merged = Vec::new();
    let mut current = ranges[0];

    for &(start, end) in &ranges[1..] {
        if start <= current.1 + 1 {
            // Overlapping or adjacent - merge them
            current.1 = current.1.max(end);
        } else {
            // Non-overlapping - save current and start new
            merged.push(current);
            current = (start, end);
        }
    }
    merged.push(current);

    merged
}
```

Then sum up the size of each merged range: `(end - start + 1)`

### Example Breakdown

Given ranges:
- `3-5` (3 IDs: 3, 4, 5)
- `10-14` (5 IDs)
- `16-20` (5 IDs)
- `12-18` (7 IDs)

After merging:
- `3-5` (3 IDs)
- `10-20` (11 IDs, merged from overlapping ranges)
- **Total: 14 IDs**

### Running the Solution

```bash
cargo run --release
```

The solution outputs both Part 1 and Part 2 answers.
