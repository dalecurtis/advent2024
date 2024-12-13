extern crate pathfinding;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use pathfinding::prelude;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
struct Point {
    x: usize,
    y: usize,
    v: char,
}

const VALID_DIR: [[i32; 2]; 4] = [[1, 0], [0, 1], [-1, 0], [0, -1]];

fn is_valid(point: &Point, extents: &Point) -> bool {
    return point.x < extents.x && point.y < extents.y;
}

fn is_neighbor(p1: &Point, p2: &Point) -> bool {
    return (p1.x == p2.x && (p1.y == p2.y + 1 || p1.y == p2.y - 1))
        || (p1.y == p2.y && (p1.x == p2.x + 1 || p1.x == p2.x - 1));
}

// Implement necessary traits for pathfinding crate
impl Point {
    fn successors(&self, graph: &Vec<Vec<char>>, extents: &Point) -> Vec<Self> {
        let mut successors = Vec::new();
        for dir in VALID_DIR {
            let next_x = self.x as i32 + dir[0];
            let next_y = self.y as i32 + dir[1];
            if next_x < 0 || next_y < 0 {
                continue;
            }

            let p = Point {
                x: next_x as usize,
                y: next_y as usize,
                v: self.v,
            };

            if is_valid(&p, extents) && graph[p.y][p.x] == self.v {
                successors.push(p);
            }
        }
        return successors;
    }
}

// Reads lines of a file and then pads the data with `pad_len` spaces
// above and below.
fn create_padded_input(file_name: &str, pad_len: usize) -> Vec<String> {
    let file = File::open(file_name).expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut input = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let padding = " ".repeat(pad_len);
        let padded_line = padding.clone() + &line + &padding;
        if input.is_empty() {
            for _i in 0..pad_len {
                input.push(" ".repeat(padded_line.len()));
            }
        }
        input.push(padded_line);
    }
    return input;
}

fn main() {
    // Didn't end up needing padding...
    let padded_input = create_padded_input("input12.txt", 0);

    let mut input = Vec::new();
    let mut input_graph: Vec<Vec<char>> = Vec::new();
    let mut extents = Point {
        x: 0,
        y: 0,
        v: '\0',
    };

    for line in padded_input {
        extents.x = line.len();
        input_graph.push(line.chars().collect());
        for (x, c) in line.chars().enumerate() {
            match c {
                ' ' => {}
                c => {
                    input.push(Point {
                        x: x,
                        y: extents.y,
                        v: c,
                    });
                }
            }
        }
        extents.y += 1;
    }

    let components = prelude::strongly_connected_components(&input, |p: &Point| {
        p.successors(&input_graph, &extents)
    });

    let mut total_perimeter_cost: usize = 0;
    let mut total_side_cost: usize = 0;
    for keys in &components {
        let mut total_perimeter = 0;
        let total_area = keys.len();

        let mut all_sides: HashMap<Point, HashSet<char>> = HashMap::new();
        for i in 0..keys.len() {
            let mut sides = HashSet::from(['U', 'D', 'L', 'R']);
            for j in 0..keys.len() {
                if i == j {
                    continue;
                }

                let a = &keys[i];
                let b = &keys[j];
                if a.x == b.x {
                    if a.y == b.y + 1 {
                        sides.remove(&'D');
                    } else if a.y == b.y - 1 {
                        sides.remove(&'U');
                    }
                } else if a.y == b.y {
                    if a.x == b.x + 1 {
                        sides.remove(&'R');
                    } else if a.x == b.x - 1 {
                        sides.remove(&'L');
                    }
                }
            }
            total_perimeter += sides.len();
            all_sides.insert(keys[i], sides);
        }

        // Compute sides that are shared between points.
        let mut shared_sides = 0;
        let points: Vec<&Point> = all_sides.keys().collect();
        for i in 0..points.len() {
            for j in i + 1..points.len() {
                let p1 = points[i];
                let p2 = points[j];

                if is_neighbor(p1, p2) {
                    let sides1 = all_sides.get(p1).unwrap();
                    let sides2 = all_sides.get(p2).unwrap();
                    shared_sides += sides1.intersection(sides2).count();
                }
            }
        }

        // Subtract that from the perimeter to get the total unique sides.
        let total_sides = total_perimeter - shared_sides;

        println!(
            "ch={}, perimeter={}, area={}, cost={}, sides={}, side_cost={}",
            keys[0].v,
            total_perimeter,
            total_area,
            total_perimeter * total_area,
            total_sides,
            total_area * total_sides
        );
        total_perimeter_cost += total_perimeter * total_area;
        total_side_cost += total_area * total_sides;
    }

    println!(
        "components={}, perimeter_cost={}, side_cost={}",
        components.len(),
        total_perimeter_cost,
        total_side_cost
    );
}
