# --- Day 8: Playground ---

Equipped with a new understanding of teleporter maintenance, you confidently step onto the repaired teleporter pad.

You rematerialize on an unfamiliar teleporter pad and find yourself in a vast underground space which contains a giant playground!

Across the playground, a group of Elves are working on setting up an ambitious Christmas decoration project. Through careful rigging, they have suspended a large number of small electrical junction boxes.

Their plan is to connect the junction boxes with long strings of lights. Most of the junction boxes don't provide electricity; however, when two junction boxes are connected by a string of lights, electricity can pass between those two junction boxes.

The Elves are trying to figure out which junction boxes to connect so that electricity can reach every junction box. They even have a list of all of the junction boxes' positions in 3D space (your puzzle input).

For example:

162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
This list describes the position of 20 junction boxes, one per line. Each position is given as X,Y,Z coordinates. So, the first junction box in the list is at X=162, Y=817, Z=812.

To save on string lights, the Elves would like to focus on connecting pairs of junction boxes that are as close together as possible according to straight-line distance. In this example, the two junction boxes which are closest together are 162,817,812 and 425,690,689.

By connecting these two junction boxes together, because electricity can flow between them, they become part of the same circuit. After connecting them, there is a single circuit which contains two junction boxes, and the remaining 18 junction boxes remain in their own individual circuits.

Now, the two junction boxes which are closest together but aren't already directly connected are 162,817,812 and 431,825,988. After connecting them, since 162,817,812 is already connected to another junction box, there is now a single circuit which contains three junction boxes and an additional 17 circuits which contain one junction box each.

The next two junction boxes to connect are 906,360,560 and 805,96,715. After connecting them, there is a circuit containing 3 junction boxes, a circuit containing 2 junction boxes, and 15 circuits which contain one junction box each.

The next two junction boxes are 431,825,988 and 425,690,689. Because these two junction boxes were already in the same circuit, nothing happens!

This process continues for a while, and the Elves are concerned that they don't have enough extension cables for all these circuits. They would like to know how big the circuits will be.

After making the ten shortest connections, there are 11 circuits: one circuit which contains 5 junction boxes, one circuit which contains 4 junction boxes, two circuits which contain 2 junction boxes each, and seven circuits which each contain a single junction box. Multiplying together the sizes of the three largest circuits (5, 4, and one of the circuits of size 2) produces 40.

Your list contains many junction boxes; connect together the 1000 pairs of junction boxes which are closest together. Afterward, what do you get if you multiply together the sizes of the three largest circuits?

To begin, get your puzzle input.

---

# Answer (Part 1)

**133574**

With 1000 junction boxes and making 1000 connections between the closest pairs, the product of the three largest circuit sizes is **133574**.

## Algorithm

This problem uses a **Union-Find (Disjoint Set Union)** data structure to efficiently track and merge circuits:

1. **Parse 3D Coordinates**: Extract all junction box positions (x, y, z) from the input
2. **Calculate All Distances**: Compute the squared Euclidean distance between every pair of junction boxes:
   - Distance² = (x₁ - x₂)² + (y₁ - y₂)² + (z₁ - z₂)²
   - We use squared distance to avoid expensive square root calculations
3. **Sort Pairs by Distance**: Sort all pairs in ascending order of distance
4. **Connect Closest Pairs**: Iterate through the sorted pairs and make 1000 connections using Union-Find:
   - Union-Find tracks which junction boxes are in the same circuit
   - When connecting two boxes already in the same circuit, nothing happens
   - Otherwise, merge their circuits
5. **Find Three Largest Circuits**: After all connections, identify the sizes of all distinct circuits and multiply the three largest

## Union-Find Operations

- **Find**: Determine which circuit a junction box belongs to (with path compression)
- **Union**: Merge two circuits into one (with union by size for efficiency)
- **Get Circuit Sizes**: Count how many junction boxes are in each distinct circuit

## Example Walkthrough

With 20 junction boxes and 10 connections:

- After connections, we get: one 5-box circuit, one 4-box circuit, two 2-box circuits, and seven 1-box circuits
- Three largest: 5 × 4 × 2 = **40**

---

# --- Part Two ---

The Elves were right; they definitely don't have enough extension cables. You'll need to keep connecting junction boxes together until they're all in one large circuit.

Continuing the above example, the first connection which causes all of the junction boxes to form a single circuit is between the junction boxes at 216,146,977 and 117,168,530. The Elves need to know how far those junction boxes are from the wall so they can pick the right extension cable; multiplying the X coordinates of those two junction boxes (216 and 117) produces 25272.

Continue connecting the closest unconnected pairs of junction boxes together until they're all in the same circuit. What do you get if you multiply together the X coordinates of the last two junction boxes you need to connect?

---

# Answer (Part 2)

**2435100380**

Continuing to connect junction boxes by distance until all 1000 boxes form a single circuit, the product of the X coordinates of the last connection is **2435100380**.

## Algorithm

Part 2 extends the Union-Find approach from Part 1:

1. **Parse all junction boxes** as before
2. **Generate and sort all pairs** by distance
3. **Keep connecting pairs** in order of increasing distance, tracking successful unions
4. **Monitor the number of circuits**: After each successful union, check if we've reached 1 component
5. **Record the last connection**: Track the indices of the junction boxes in the final connection
6. **Return X coordinate product**: Multiply the X coordinates of the two boxes that completed the circuit

The key difference from Part 1:
- Part 1 stops after exactly 1000 connections and counts circuit sizes
- Part 2 continues until all boxes are in one circuit and returns X coordinates of the last pair

### Implementation Note

The result uses `i64` to handle the product of two large i32 coordinates without overflow.

---
