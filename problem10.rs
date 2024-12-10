extern crate pathfinding;

use pathfinding::prelude::count_paths;
use pathfinding::prelude::dfs;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
    v: u8,
}

// Implement necessary traits for pathfinding crate
impl Point {
    fn successors(&self, graph: &Vec<Self>) -> Vec<Self> {
        let current_index = graph.iter().position(|p| p == self).unwrap();
        let mut successors = Vec::new();
        for (index, point) in graph.iter().enumerate() {
            if index != current_index
                && point.v == self.v + 1
                && ((point.x == self.x && (point.y == self.y + 1 || point.y == self.y - 1))
                    || (point.y == self.y && (point.x == self.x + 1 || point.x == self.x - 1)))
            {
                successors.push(point.clone());
            }
        }
        successors
    }
}

fn main() {
    let file = File::open("input10.txt").expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut input = Vec::new();
    let mut start_points = Vec::new();
    let mut end_points = Vec::new();
    let mut extents = Point { x: 0, y: 0, v: 0 };

    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        extents.x = line.len();
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {}
                c => {
                    input.push(Point {
                        x: x,
                        y: extents.y,
                        v: c.to_digit(10).expect("fail") as u8,
                    });
                    if c == '9' {
                        end_points.push(input.last().expect("fail").clone());
                    } else if c == '0' {
                        start_points.push(input.last().expect("fail").clone());
                    }
                }
            }
        }
        extents.y += 1;
    }

    let mut all_paths = Vec::new();
    let mut total_path_count: usize = 0;
    for start_node in &start_points {
        for end_node in &end_points {
            let paths = dfs(
                start_node.clone(),
                |p| p.successors(&input),
                |p| p == end_node,
            );
            match paths {
                Some(paths) => {
                    all_paths.push(paths);
                    let path_count = count_paths(
                        start_node.clone(),
                        |p| p.successors(&input),
                        |p| p == end_node,
                    );
                    total_path_count += path_count;
                }
                None => {}
            };
        }
    }
    println!(
        "All paths: paths.len = {}, path_count={}",
        all_paths.len(),
        total_path_count
    );
}