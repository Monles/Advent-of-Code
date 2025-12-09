use std::fs;

#[derive(Debug, Clone, Copy)]
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

// Shoelace formula to calculate polygon area
fn shoelace_area(points: &[Point]) -> i64 {
    let n = points.len();
    let mut sum = 0i64;

    for i in 0..n {
        let j = (i + 1) % n;
        sum += (points[i].x as i64) * (points[j].y as i64);
        sum -= (points[j].x as i64) * (points[i].y as i64);
    }

    sum.abs() / 2
}

// Calculate boundary points (Manhattan distance between consecutive points)
fn boundary_points(points: &[Point]) -> i64 {
    let n = points.len();
    let mut boundary = 0i64;

    for i in 0..n {
        let j = (i + 1) % n;
        let dx = (points[i].x - points[j].x).abs() as i64;
        let dy = (points[i].y - points[j].y).abs() as i64;
        boundary += dx + dy;
    }

    boundary
}

// Pick's theorem: A = I + B/2 - 1, so I = A - B/2 + 1
fn interior_points(area: i64, boundary: i64) -> i64 {
    area - boundary / 2 + 1
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let points = parse_input(&input);

    println!("Number of red tiles (vertices): {}", points.len());

    let area = shoelace_area(&points);
    println!("Polygon area (Shoelace): {}", area);

    let boundary = boundary_points(&points);
    println!("Boundary points: {}", boundary);

    let interior = interior_points(area, boundary);
    println!("Interior points: {}", interior);

    let total_green_red = boundary + interior;
    println!("Total red + green tiles: {}", total_green_red);

    println!("\nBounding box:");
    let min_x = points.iter().map(|p| p.x).min().unwrap();
    let max_x = points.iter().map(|p| p.x).max().unwrap();
    let min_y = points.iter().map(|p| p.y).min().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();
    println!("  X: {} to {} (width: {})", min_x, max_x, max_x - min_x + 1);
    println!("  Y: {} to {} (height: {})", min_y, max_y, max_y - min_y + 1);
    println!("  Bounding box area: {}", (max_x - min_x + 1) as i64 * (max_y - min_y + 1) as i64);
}
