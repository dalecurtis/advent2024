// After cargo build --release, build with rustc -L target/release/deps problem3.rs

extern crate itertools;

use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Default, Clone, Eq, Hash, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

fn is_in_bounds(point: &Point, extents: &Point) -> bool {
    return point.x >= 0 && point.y >= 0 && point.x < extents.x && point.y < extents.y;
}

fn main() {
    let file = File::open("input8.txt").expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut input = HashMap::new();
    let mut extents = Point { x: 0, y: 0 };

    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        extents.x = line.len() as i64;
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {}
                _ => input.entry(c).or_insert(Vec::new()).push(Point {
                    x: x as i64,
                    y: extents.y,
                }),
            }
        }
        extents.y += 1;
    }

    println!("[0, 0, {:?}]", extents);
    let mut unique_antinodes: HashSet<Point> = HashSet::new();
    let mut extended_antinodes: HashSet<Point> = HashSet::new();

    for (_, points) in input {
        for combination in points.iter().combinations(2) {
            let p1 = &combination[0];
            let p2 = &combination[1];
            extended_antinodes.insert((*p1).clone());
            extended_antinodes.insert((*p2).clone());

            let distance = Point {
                x: p1.x - p2.x,
                y: p1.y - p2.y,
            };

            let pos_antinode = Point {
                x: p1.x + distance.x,
                y: p1.y + distance.y,
            };
            if is_in_bounds(&pos_antinode, &extents) {
                let mut next_antinode = pos_antinode.clone();
                unique_antinodes.insert(pos_antinode);
                extended_antinodes.insert(next_antinode.clone());
                loop {
                    next_antinode.x += distance.x;
                    next_antinode.y += distance.y;
                    if is_in_bounds(&next_antinode, &extents) {
                        extended_antinodes.insert(next_antinode.clone());
                    } else {
                        break;
                    }
                }
            }

            let neg_antinode = Point {
                x: p2.x - distance.x,
                y: p2.y - distance.y,
            };
            if is_in_bounds(&neg_antinode, &extents) {
                let mut next_antinode = neg_antinode.clone();
                unique_antinodes.insert(neg_antinode);
                extended_antinodes.insert(next_antinode.clone());
                loop {
                    next_antinode.x -= distance.x;
                    next_antinode.y -= distance.y;
                    if is_in_bounds(&next_antinode, &extents) {
                        extended_antinodes.insert(next_antinode.clone());
                    } else {
                        break;
                    }
                }
            }
        }
    }

    println!(
        "unique_len={}, extended_len={}",
        unique_antinodes.len(),
        extended_antinodes.len()
    );
}
