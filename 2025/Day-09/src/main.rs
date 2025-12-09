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

        if (pi.y > p.y) != (pj.y > p.y) {
            // Use i64 to avoid overflow with large coordinates
            let x_intersect = (pj.x as i64 - pi.x as i64) * (p.y as i64 - pi.y as i64) / (pj.y as i64 - pi.y as i64) + pi.x as i64;
            if (p.x as i64) < x_intersect {
                inside = !inside;
            }
        }
        j = i;
    }

    inside
}

// Check if point p is on line segment from p1 to p2
fn is_on_segment(p: Point, p1: Point, p2: Point) -> bool {
    // Check if p is collinear with p1 and p2 using i64 to avoid overflow
    let cross = (p.y as i64 - p1.y as i64) * (p2.x as i64 - p1.x as i64)
              - (p.x as i64 - p1.x as i64) * (p2.y as i64 - p1.y as i64);
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
    let mut best_corners = (Point { x: 0, y: 0 }, Point { x: 0, y: 0 });

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

            // For small rectangles, check all points
            // For larger rectangles, check a sample of points along edges
            const SMALL_AREA_THRESHOLD: i64 = 10000000; // 10 million - check more exhaustively

            let valid = if area <= SMALL_AREA_THRESHOLD {
                // Check all points for small rectangles
                let mut all_valid = true;
                'outer: for x in min_x..=max_x {
                    for y in min_y..=max_y {
                        if !point_in_or_on_polygon(Point { x, y }, points) {
                            all_valid = false;
                            break 'outer;
                        }
                    }
                }
                all_valid
            } else {
                // For large rectangles, sample points more densely along edges
                // Check the 2 implied corners first
                let corner3 = Point { x: min_x, y: max_y };
                let corner4 = Point { x: max_x, y: min_y };

                if !point_in_or_on_polygon(corner3, points) ||
                   !point_in_or_on_polygon(corner4, points) {
                    false
                } else {
                    // Sample 100 points along the perimeter and check interior samples
                    let num_samples = 100;
                    let mut all_valid = true;

                    // Sample points along all 4 edges
                    for i in 0..num_samples {
                        let t = i as f64 / num_samples as f64;

                        // Top edge
                        let x = (min_x as f64 + t * (max_x - min_x) as f64) as i32;
                        if !point_in_or_on_polygon(Point { x, y: max_y }, points) {
                            all_valid = false;
                            break;
                        }

                        // Bottom edge
                        let x = (min_x as f64 + t * (max_x - min_x) as f64) as i32;
                        if !point_in_or_on_polygon(Point { x, y: min_y }, points) {
                            all_valid = false;
                            break;
                        }

                        // Left edge
                        let y = (min_y as f64 + t * (max_y - min_y) as f64) as i32;
                        if !point_in_or_on_polygon(Point { x: min_x, y }, points) {
                            all_valid = false;
                            break;
                        }

                        // Right edge
                        let y = (min_y as f64 + t * (max_y - min_y) as f64) as i32;
                        if !point_in_or_on_polygon(Point { x: max_x, y }, points) {
                            all_valid = false;
                            break;
                        }
                    }

                    // Also check some interior points
                    if all_valid {
                        for i in 1..10 {
                            for j in 1..10 {
                                let x = min_x + (max_x - min_x) * i / 10;
                                let y = min_y + (max_y - min_y) * j / 10;
                                if !point_in_or_on_polygon(Point { x, y }, points) {
                                    all_valid = false;
                                    break;
                                }
                            }
                            if !all_valid {
                                break;
                            }
                        }
                    }

                    all_valid
                }
            };

            if valid {
                if area > max_area {
                    max_area = area;
                    best_corners = (p1, p2);
                }
            }
        }
    }

    if max_area > 0 {
        eprintln!("Best rectangle: ({},{}) to ({},{}) with area {}",
                  best_corners.0.x, best_corners.0.y,
                  best_corners.1.x, best_corners.1.y,
                  max_area);
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
