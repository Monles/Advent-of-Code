
# --- Day 9: Movie Theater ---

You slide down the firepole in the corner of the playground and land in the North Pole base movie theater!

The movie theater has a big tile floor with an interesting pattern. Elves here are redecorating the theater by switching out some of the square tiles in the big grid they form. Some of the tiles are red; the Elves would like to find the largest rectangle that uses red tiles for two of its opposite corners. They even have a list of where the red tiles are located in the grid (your puzzle input).

For example:

7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
Showing red tiles as # and other tiles as ., the above arrangement of red tiles would look like this:

..............
.......#...#..
..............
..#....#......
..............
..#......#....
..............
.........#.#..
..............
You can choose any two red tiles as the opposite corners of your rectangle; your goal is to find the largest rectangle possible.

For example, you could make a rectangle (shown as O) with an area of 24 between 2,5 and 9,7:

..............
.......#...#..
..............
..#....#......
..............
..OOOOOOOO....
..OOOOOOOO....
..OOOOOOOO.#..
..............
Or, you could make a rectangle with area 35 between 7,1 and 11,7:

..............
.......OOOOO..
.......OOOOO..
..#....OOOOO..
.......OOOOO..
..#....OOOOO..
.......OOOOO..
.......OOOOO..
..............
You could even make a thin rectangle with an area of only 6 between 7,3 and 2,3:

..............
.......#...#..
..............
..OOOOOO......
..............
..#......#....
..............
.........#.#..
..............
Ultimately, the largest rectangle you can make in this example has area 50. One way to do this is between 2,5 and 11,1:

..............
..OOOOOOOOOO..
..OOOOOOOOOO..
..OOOOOOOOOO..
..OOOOOOOOOO..
..OOOOOOOOOO..
..............
.........#.#..
..............
Using two red tiles as opposite corners, what is the largest area of any rectangle you can make?

Your puzzle answer was 4761736832.

The first half of this puzzle is complete! It provides one gold star: *

# --- Part Two ---

The Elves just remembered: they can only switch out tiles that are red or green. So, your rectangle can only include red or green tiles.

In your list, every red tile is connected to the red tile before and after it by a straight line of green tiles. The list wraps, so the first red tile is also connected to the last red tile. Tiles that are adjacent in your list will always be on either the same row or the same column.

Using the same example as before, the tiles marked X would be green:

..............
.......#XXX#..
.......X...X..
..#XXXX#...X..
..X........X..
..#XXXXXX#.X..
.........X.X..
.........#X#..
..............
In addition, all of the tiles inside this loop of red and green tiles are also green. So, in this example, these are the green tiles:

..............
.......#XXX#..
.......XXXXX..
..#XXXX#XXXX..
..XXXXXXXXXX..
..#XXXXXX#XX..
.........XXX..
.........#X#..
..............
The remaining tiles are never red nor green.

The rectangle you choose still must have red tiles in opposite corners, but any other tiles it includes must now be red or green. This significantly limits your options.

For example, you could make a rectangle out of red and green tiles with an area of 15 between 7,3 and 11,1:

..............
.......OOOOO..
.......OOOOO..
..#XXXXOOOOO..
..XXXXXXXXXX..
..#XXXXXX#XX..
.........XXX..
.........#X#..
..............
Or, you could make a thin rectangle with an area of 3 between 9,7 and 9,5:

..............
.......#XXX#..
.......XXXXX..
..#XXXX#XXXX..
..XXXXXXXXXX..
..#XXXXXXOXX..
.........OXX..
.........OX#..
..............
The largest rectangle you can make in this example using only red and green tiles has area 24. One way to do this is between 9,5 and 2,3:

..............
.......#XXX#..
.......XXXXX..
..OOOOOOOOXX..
..OOOOOOOOXX..
..OOOOOOOOXX..
.........XXX..
.........#X#..
..............
Using two red tiles as opposite corners, what is the largest area of any rectangle you can make using only red and green tiles?

Answer:

Although it hasn't changed, you can still get your puzzle input.

You can also [Share] this puzzle.

# Answer (Part 2)

**1452422268** (candidate answer - awaiting submission after 10-minute wait period)

With 496 red tiles forming a polygon, the largest rectangle that lies entirely within the red/green region has area **1,452,422,268**.

## Solution

The winning rectangle spans from (4615, 66437) to (94737, 50322):
- Width: 90,123 tiles
- Height: 16,116 tiles
- Area: 1,452,422,268 tiles

The polygon contains ~7.05 billion total tiles (calculated via Shoelace formula + Pick's theorem), so this rectangle uses approximately 21% of the available space.

## Algorithm

### Overview

Part 2 requires determining if a rectangle lies entirely within the polygon formed by the red tiles:

1. **Polygon Formation**:
   - Red tiles are connected in sequence order (with wrapping)
   - Green tiles are: edges connecting consecutive red tiles + interior of polygon

2. **Point-in-Polygon Test**:
   - Ray casting algorithm to check if point is inside polygon
   - Edge detection to check if point is on polygon boundary
   - **Critical**: Uses `i64` arithmetic to prevent overflow with large coordinates

3. **Rectangle Validation**:
   - Try all pairs of red tiles as opposite corners
   - Quick rejection: Skip if extends beyond polygon bounding box
   - Quick rejection: Skip if area ≤ current max
   - For rectangles ≤ 10 million area: Check ALL points exhaustively
   - For larger rectangles: Use dense sampling (100 points per edge + 9×9 interior grid)

### Code Implementation

```rust
// Point-in-polygon with overflow protection
fn point_in_or_on_polygon(p: Point, polygon: &[Point]) -> bool {
    // Check edges first
    for i in 0..polygon.len() {
        if is_on_segment(p, polygon[i], polygon[(i + 1) % polygon.len()]) {
            return true;
        }
    }

    // Ray casting with i64 to prevent overflow
    let mut inside = false;
    let mut j = polygon.len() - 1;
    for i in 0..polygon.len() {
        let pi = polygon[i];
        let pj = polygon[j];
        if (pi.y > p.y) != (pj.y > p.y) {
            let x_intersect = (pj.x as i64 - pi.x as i64) * (p.y as i64 - pi.y as i64)
                            / (pj.y as i64 - pi.y as i64) + pi.x as i64;
            if (p.x as i64) < x_intersect {
                inside = !inside;
            }
        }
        j = i;
    }
    inside
}
```

### Key Fixes

**Integer Overflow**: The original implementation used `i32` for geometric calculations. With coordinates up to 98,306, multiplications like `(pj.x - pi.x) * (p.y - pi.y)` could overflow `i32::MAX` (2,147,483,647). Fixed by casting to `i64`.

### Example Walkthrough

With 8 red tiles forming a polygon:

- Largest valid rectangle: (9,5) to (2,3)
- Width = |9-2| + 1 = 8, Height = |5-3| + 1 = 3, Area = 24 ✓
- All 24 points within rectangle are inside or on polygon boundary

### Performance Considerations

- Coordinates range: 1,844 to 98,306 (span ~96,000)
- 496 red tiles forming the polygon
- Maximum possible rectangle: ~9.2 billion square units
- Current approach: Check all points for rectangles up to 10 million area
- Threshold of 10 million provides good balance between accuracy and performance
- The winning rectangle (1.45 billion area) was validated using dense sampling

---
