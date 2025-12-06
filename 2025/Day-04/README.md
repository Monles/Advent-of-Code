# --- Day 4: Printing Department ---

You ride the escalator down to the printing department. They're clearly getting ready for Christmas; they have lots of large rolls of paper everywhere, and there's even a massive printer in the corner (to handle the really big print jobs).

Decorating here will be easy: they can make their own decorations. What you really need is a way to get further into the North Pole base while the elevators are offline.

"Actually, maybe we can help with that," one of the Elves replies when you ask for help. "We're pretty sure there's a cafeteria on the other side of the back wall. If we could break through the wall, you'd be able to keep moving. It's too bad all of our forklifts are so busy moving those big rolls of paper around."

If you can optimize the work the forklifts are doing, maybe they would have time to spare to break through the wall.

The rolls of paper (@) are arranged on a large grid; the Elves even have a helpful diagram (your puzzle input) indicating where everything is located.

For example:

```
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.

```

The forklifts can only access a roll of paper if there are fewer than four rolls of paper in the eight adjacent positions. If you can figure out which rolls of paper the forklifts can access, they'll spend less time looking and more time breaking down the wall to the cafeteria.

In this example, there are 13 rolls of paper that can be accessed by a forklift (marked with x):

```
..xx.xx@x.
x@@.@.@.@@
@@@@@.x.@@
@.@@@@..@.
x@.@@@@.@x
.@@@@@@@.@
.@.@.@.@@@
x.@@@.@@@@
.@@@@@@@@.
x.x.@@@.x.
```

Consider your complete diagram of the paper roll locations. How many rolls of paper can be accessed by a forklift?

# Answer (Part 1)

## Solution Approach

The problem asks us to find how many rolls of paper (@) can be accessed by a forklift. A roll is accessible if it has **fewer than 4** adjacent rolls in the 8 surrounding positions (including diagonals).

### Algorithm

1. **Parse the grid**: Read the input as a 2D grid of characters
2. **Iterate through each position**: For every cell in the grid
3. **Check if it's a roll**: Only process cells containing '@'
4. **Count adjacent rolls**: For each '@', check all 8 adjacent positions:
   - Top-left, top, top-right
   - Left, right
   - Bottom-left, bottom, bottom-right
5. **Determine accessibility**: If the count of adjacent '@' symbols is < 4, increment the accessible counter

### Implementation Details

- Used 8 direction vectors to check all adjacent positions: `[(-1,-1), (-1,0), (-1,1), (0,-1), (0,1), (1,-1), (1,0), (1,1)]`
- Handled boundary conditions by checking if positions are within grid bounds
- Included a test case with the provided example (expected: 13 accessible rolls)

### Running the Solution

```bash
cargo run
```

The solution reads from `input.txt` and outputs the number of accessible rolls.

---

# --- Part Two ---

Now, the Elves just need help accessing as much of the paper as they can.

Once a roll of paper can be accessed by a forklift, it can be removed. Once a roll of paper is removed, the forklifts might be able to access more rolls of paper, which they might also be able to remove. How many total rolls of paper could the Elves remove if they keep repeating this process?

Starting with the same example as above, here is one way you could remove as many rolls of paper as possible, using highlighted @ to indicate that a roll of paper is about to be removed, and using x to indicate that a roll of paper was just removed:

Initial state:

```
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
```

Remove 13 rolls of paper:

```
..xx.xx@x.
x@@.@.@.@@
@@@@@.x.@@
@.@@@@..@.
x@.@@@@.@x
.@@@@@@@.@
.@.@.@.@@@
x.@@@.@@@@
.@@@@@@@@.
x.x.@@@.x.

```

Remove 12 rolls of paper:

```
.......x..
.@@.x.x.@x
x@@@@...@@
x.@@@@..x.
.@.@@@@.x.
.x@@@@@@.x
.x.@.@.@@@
..@@@.@@@@
.x@@@@@@@.
....@@@...
```

Remove 7 rolls of paper:

```
..........
.x@.....x.
.@@@@...xx
..@@@@....
.x.@@@@...
..@@@@@@..
...@.@.@@x
..@@@.@@@@
..x@@@@@@.
....@@@...
```

Remove 5 rolls of paper:

```
..........
..x.......
.x@@@.....
..@@@@....
...@@@@...
..x@@@@@..
...@.@.@@.
..x@@.@@@x
...@@@@@@.
....@@@...
```

Remove 2 rolls of paper:

```
..........
..........
..x@@.....
..@@@@....
...@@@@...
...@@@@@..
...@.@.@@.
...@@.@@@.
...@@@@@x.
....@@@...

```

Remove 1 roll of paper:

```
..........
..........
...@@.....
..x@@@....
...@@@@...
...@@@@@..
...@.@.@@.
...@@.@@@.
...@@@@@..
....@@@...
```

Remove 1 roll of paper:

```
..........
..........
...x@.....
...@@@....
...@@@@...
...@@@@@..
...@.@.@@.
...@@.@@@.
...@@@@@..
....@@@...
```

Remove 1 roll of paper:

```
..........
..........
....x.....
...@@@....
...@@@@...
...@@@@@..
...@.@.@@.
...@@.@@@.
...@@@@@..
....@@@...
```

Remove 1 roll of paper:

```
..........
..........
..........
...x@@....
...@@@@...
...@@@@@..
...@.@.@@.
...@@.@@@.
...@@@@@..
....@@@...
```

Stop once no more rolls of paper are accessible by a forklift. In this example, a total of 43 rolls of paper can be removed.

Start with your original diagram. How many rolls of paper in total can be removed by the Elves and their forklifts?

---

# Answer (Part 2)

**9206**

## Solution Approach

Part 2 requires an iterative removal process:

1. **Find all accessible rolls** in the current grid state (those with < 4 adjacent rolls)
2. **Remove all accessible rolls simultaneously** by replacing them with '.'
3. **Repeat** until no more rolls are accessible
4. **Count total removed** across all iterations

### Key Insight

Unlike Part 1 which just counts accessible rolls in the initial state, Part 2 simulates the actual removal process. As rolls are removed, previously inaccessible rolls may become accessible because they now have fewer neighbors.

### Algorithm

```rust
loop {
    - Find all positions with '@' that have < 4 adjacent '@' symbols
    - If none found, stop
    - Remove all found positions (set to '.')
    - Add count to total
}
```

### Example Breakdown

In the example:
- Round 1: Remove 13 rolls (the initially accessible ones)
- Round 2: Remove 12 rolls (newly exposed)
- Round 3: Remove 7 rolls
- Round 4: Remove 5 rolls
- Round 5: Remove 2 rolls
- Rounds 6-9: Remove 1 roll each
- **Total: 43 rolls removed**

### Running the Solution

```bash
cargo run --release
```

The solution outputs both Part 1 and Part 2 answers.
