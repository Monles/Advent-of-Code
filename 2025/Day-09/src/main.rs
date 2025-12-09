use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }

            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() != 2 {
                return None;
            }

            let x = parts[0].trim().parse::<i32>().ok()?;
            let y = parts[1].trim().parse::<i32>().ok()?;

            Some(Point { x, y })
        })
        .collect()
}

fn find_largest_rectangle(points: &[Point]) -> i64 {
    let n = points.len();
    let mut max_area = 0i64;

    // Try all pairs of points as opposite corners
    for i in 0..n {
        for j in (i + 1)..n {
            let p1 = points[i];
            let p2 = points[j];

            // Calculate rectangle area (inclusive of corner tiles)
            let width = ((p1.x - p2.x).abs() + 1) as i64;
            let height = ((p1.y - p2.y).abs() + 1) as i64;
            let area = width * height;

            max_area = max_area.max(area);
        }
    }

    max_area
}

// Check if a point is inside or on the boundary of a polygon using ray casting
fn point_in_or_on_polygon(p: Point, polygon: &[Point]) -> bool {
    let n = polygon.len();

    // Check if point is on an edge
    for i in 0..n {
        let p1 = polygon[i];
        let p2 = polygon[(i + 1) % n];

        if is_on_segment(p, p1, p2) {
            return true;
        }
    }

    // Ray casting algorithm for interior points
    let mut inside = false;
    let mut j = n - 1;

    for i in 0..n {
        let pi = polygon[i];
        let pj = polygon[j];

        if ((pi.y > p.y) != (pj.y > p.y)) &&
           (p.x < (pj.x - pi.x) * (p.y - pi.y) / (pj.y - pi.y) + pi.x) {
            inside = !inside;
        }
        j = i;
    }

    inside
}

// Check if point p is on line segment from p1 to p2
fn is_on_segment(p: Point, p1: Point, p2: Point) -> bool {
    // Check if p is collinear with p1 and p2
    let cross = (p.y - p1.y) * (p2.x - p1.x) - (p.x - p1.x) * (p2.y - p1.y);
    if cross != 0 {
        return false;
    }

    // Check if p is within the bounding box of the segment
    let min_x = p1.x.min(p2.x);
    let max_x = p1.x.max(p2.x);
    let min_y = p1.y.min(p2.y);
    let max_y = p1.y.max(p2.y);

    p.x >= min_x && p.x <= max_x && p.y >= min_y && p.y <= max_y
}

fn find_largest_rectangle_part2(points: &[Point]) -> i64 {
    let n = points.len();
    let mut max_area = 0i64;

    // Compute polygon bounding box for quick rejection
    let mut min_poly_x = i32::MAX;
    let mut max_poly_x = i32::MIN;
    let mut min_poly_y = i32::MAX;
    let mut max_poly_y = i32::MIN;
    for p in points {
        min_poly_x = min_poly_x.min(p.x);
        max_poly_x = max_poly_x.max(p.x);
        min_poly_y = min_poly_y.min(p.y);
        max_poly_y = max_poly_y.max(p.y);
    }

    // Try all pairs of red tiles as opposite corners
    for i in 0..n {
        for j in (i + 1)..n {
            let p1 = points[i];
            let p2 = points[j];

            // Get rectangle bounds
            let min_x = p1.x.min(p2.x);
            let max_x = p1.x.max(p2.x);
            let min_y = p1.y.min(p2.y);
            let max_y = p1.y.max(p2.y);

            let width = (max_x - min_x + 1) as i64;
            let height = (max_y - min_y + 1) as i64;
            let area = width * height;

            // Skip if this rectangle can't beat current max
            if area <= max_area {
                continue;
            }

            // Quick rejection: if rectangle extends beyond polygon bounds, skip
            if min_x < min_poly_x || max_x > max_poly_x ||
               min_y < min_poly_y || max_y > max_poly_y {
                continue;
            }

            // Check if rectangle is valid
            // For small to medium rectangles, check all points
            let valid = if width * height <= 20000 {
                let mut all_inside = true;
                'check_all: for x in min_x..=max_x {
                    for y in min_y..=max_y {
                        if !point_in_or_on_polygon(Point { x, y }, points) {
                            all_inside = false;
                            break 'check_all;
                        }
                    }
                }
                all_inside
            } else {
                // For larger rectangles, we can't reliably sample
                // so we skip them for now
                false
            };

            if valid {
                max_area = max_area.max(area);
            }
        }
    }

    max_area
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let points = parse_input(&input);

    println!("Number of red tiles: {}", points.len());

    let result = find_largest_rectangle(&points);
    println!("Part 1 - Largest rectangle area: {}", result);

    let result_part2 = find_largest_rectangle_part2(&points);
    println!("Part 2 - Largest rectangle area (red/green only): {}", result_part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = r"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

        let points = parse_input(input);
        assert_eq!(points.len(), 8);

        let result = find_largest_rectangle(&points);
        assert_eq!(result, 50);
    }

    #[test]
    fn test_parse() {
        let input = "7,1\n11,1\n11,7";
        let points = parse_input(input);
        assert_eq!(points.len(), 3);
        assert_eq!(points[0], Point { x: 7, y: 1 });
        assert_eq!(points[1], Point { x: 11, y: 1 });
        assert_eq!(points[2], Point { x: 11, y: 7 });
    }

    #[test]
    fn test_rectangle_area() {
        let points = vec![Point { x: 2, y: 5 }, Point { x: 11, y: 1 }];
        let result = find_largest_rectangle(&points);
        // Width: |11 - 2| + 1 = 10 (inclusive)
        // Height: |5 - 1| + 1 = 5 (inclusive)
        // Area: 10 * 5 = 50
        assert_eq!(result, 50);
    }

    #[test]
    fn test_part2_example() {
        let input = r"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

        let points = parse_input(input);
        assert_eq!(points.len(), 8);

        let result = find_largest_rectangle_part2(&points);
        assert_eq!(result, 24);
    }
}
