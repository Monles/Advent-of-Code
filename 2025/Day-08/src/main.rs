use std::fs;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn distance_squared(&self, other: &Point) -> i64 {
        let dx = (self.x - other.x) as i64;
        let dy = (self.y - other.y) as i64;
        let dz = (self.z - other.z) as i64;
        dx * dx + dy * dy + dz * dz
    }
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false; // Already in same set
        }

        // Union by size
        if self.size[root_x] < self.size[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        }
        true
    }

    fn get_circuit_sizes(&mut self) -> Vec<usize> {
        let mut sizes = std::collections::HashMap::new();
        for i in 0..self.parent.len() {
            let root = self.find(i);
            *sizes.entry(root).or_insert(0) += 1;
        }
        sizes.values().copied().collect()
    }

    fn num_components(&mut self) -> usize {
        let mut roots = std::collections::HashSet::new();
        for i in 0..self.parent.len() {
            roots.insert(self.find(i));
        }
        roots.len()
    }
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
            if parts.len() != 3 {
                return None;
            }

            // Try to parse all three parts as integers
            let x = parts[0].trim().parse::<i32>().ok()?;
            let y = parts[1].trim().parse::<i32>().ok()?;
            let z = parts[2].trim().parse::<i32>().ok()?;

            Some(Point { x, y, z })
        })
        .collect()
}

fn solve(input: &str, num_connections: usize) -> usize {
    let points = parse_input(input);
    let n = points.len();

    // Generate all pairs with their distances
    let mut pairs = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let dist = points[i].distance_squared(&points[j]);
            pairs.push((dist, i, j));
        }
    }

    // Sort by distance
    pairs.sort_by_key(|&(dist, _, _)| dist);

    // Connect the closest pairs using Union-Find
    let mut uf = UnionFind::new(n);
    let mut connections_made = 0;

    for &(_, i, j) in &pairs {
        if connections_made >= num_connections {
            break;
        }
        uf.union(i, j);
        connections_made += 1;
    }

    // Get circuit sizes and find three largest
    let mut sizes = uf.get_circuit_sizes();
    sizes.sort_by(|a, b| b.cmp(a)); // Sort descending

    // Multiply three largest
    sizes.iter().take(3).product()
}

fn solve_part2(input: &str) -> i64 {
    let points = parse_input(input);
    let n = points.len();

    // Generate all pairs with their distances
    let mut pairs = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let dist = points[i].distance_squared(&points[j]);
            pairs.push((dist, i, j));
        }
    }

    // Sort by distance
    pairs.sort_by_key(|&(dist, _, _)| dist);

    // Connect pairs until all are in one circuit
    let mut uf = UnionFind::new(n);
    let mut last_connection = (0, 0);

    for &(_, i, j) in &pairs {
        if uf.union(i, j) {
            last_connection = (i, j);
            // Check if all are connected
            if uf.num_components() == 1 {
                break;
            }
        }
    }

    // Return product of X coordinates
    points[last_connection.0].x as i64 * points[last_connection.1].x as i64
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");

    // Solve Part 1 (1000 connections)
    let result = solve(&input, 1000);
    println!("Part 1 (1000 connections): {}", result);

    // Solve Part 2 (connect until all in one circuit)
    let result_part2 = solve_part2(&input);
    println!("Part 2 (X product of last connection): {}", result_part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = r"162,817,812
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
425,690,689";

        let result = solve(input, 10);
        assert_eq!(result, 40);
    }

    #[test]
    fn test_parse() {
        let input = "162,817,812\n57,618,57\n906,360,560";
        let points = parse_input(input);
        assert_eq!(points.len(), 3);
        assert_eq!(points[0].x, 162);
        assert_eq!(points[0].y, 817);
        assert_eq!(points[0].z, 812);
    }

    #[test]
    fn test_part2_example() {
        let input = r"162,817,812
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
425,690,689";

        let result = solve_part2(input);
        assert_eq!(result, 25272); // 216 * 117
    }
}
