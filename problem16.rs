extern crate pathfinding;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use pathfinding::prelude;

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
struct Edge {
    start: Point,
    end: Point,
}

const VALID_DIR: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn move_point(p: &Point, dir: (i32, i32)) -> Point {
    return Point {
        x: (p.x as i32 + dir.0) as usize,
        y: (p.y as i32 + dir.1) as usize,
    };
}

fn compute_dir(edge: &Edge) -> (i32, i32) {
    return (
        edge.end.x as i32 - edge.start.x as i32,
        edge.end.y as i32 - edge.start.y as i32,
    );
}

// Implement necessary traits for pathfinding crate
impl Edge {
    fn successors(&self, graph: &Vec<Vec<char>>) -> Vec<(Self, u32)> {
        let mut successors = Vec::new();
        let current_dir = compute_dir(self);
        for dir in VALID_DIR {
            let next_p = move_point(&self.end, dir);
            if next_p == self.start || graph[next_p.y][next_p.x] != '.' {
                continue;
            }

            let cost;
            if dir == current_dir {
                cost = 1;
            } else {
                cost = 1001;
            }

            successors.push((
                Edge {
                    start: self.end.clone(),
                    end: next_p.clone(),
                },
                cost,
            ));
        }
        return successors;
    }
}

fn walk_maze(maze: &Vec<Vec<char>>, end: &Point, path: &Vec<Edge>) -> Vec<Vec<char>> {
    let mut result_maze = maze.clone();
    for edge in path {
        let current_dir = compute_dir(&edge);
        if edge.end == *end {
            continue;
        }
        match current_dir {
            (1, 0) => {
                result_maze[edge.end.y][edge.end.x] = '>';
            }
            (-1, 0) => {
                result_maze[edge.end.y][edge.end.x] = '<';
            }
            (0, 1) => {
                result_maze[edge.end.y][edge.end.x] = 'v';
            }
            (0, -1) => {
                result_maze[edge.end.y][edge.end.x] = '^';
            }
            _ => todo!("bad input: {:?}", current_dir),
        }
    }
    return result_maze;
}

fn print_maze(maze: &Vec<Vec<char>>, start: &Point, end: &Point) {
    for y in 0..maze.len() {
        for x in 0..maze[y].len() {
            match maze[y][x] {
                '#' => print!("#"),
                '.' => {
                    if start.x == x && start.y == y {
                        print!("S")
                    } else if end.x == x && end.y == y {
                        print!("E")
                    } else {
                        print!(".")
                    }
                }
                '>' | '<' | '^' | 'v' => print!("{}", maze[y][x]),
                _ => todo!("bad input"),
            }
        }
        println!();
    }
}

fn main() {
    let file = File::open("input16.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut start = Point::default();
    let mut end = Point::default();

    let mut maze: Vec<Vec<char>> = Vec::new();
    let mut y_pos = 0;
    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    row.push(c);
                }
                'S' | '.' | 'E' => {
                    row.push('.');
                    if c == 'S' {
                        start.x = x;
                        start.y = y_pos;
                    } else if c == 'E' {
                        end.x = x;
                        end.y = y_pos;
                    }
                }
                _ => todo!("bad input"),
            }
        }
        maze.push(row);
        y_pos += 1;
    }

    print_maze(&maze, &start, &end);

    for d in VALID_DIR {
        let start_edge = Edge {
            start: start.clone(),
            end: move_point(&start, d),
        };
        if maze[start_edge.end.y][start_edge.end.x] != '.' {
            continue;
        }

        let cost_increase;
        let dir = compute_dir(&start_edge);
        match dir {
            (1, 0) => {
                // No cost increase.
                cost_increase = 0;
            }
            (-1, 0) => {
                cost_increase = 2001;
            }
            (0, 1) => {
                cost_increase = 1001;
            }
            (0, -1) => {
                cost_increase = 1001;
            }
            _ => panic!("bad data"),
        }

        let result = prelude::astar(
            &start_edge,
            |p: &Edge| p.successors(&maze),
            |p: &Edge| {
                return ((end.x.abs_diff(p.end.x) + end.y.abs_diff(p.end.y)) / 3) as u32;
            },
            |p: &Edge| p.end == end,
        );

        if result.is_none() {
            continue;
        }
        let path = result.unwrap();
        println!("len={}, cost={}", path.0.len(), path.1 + cost_increase);
        println!();

        let walked_maze = walk_maze(&maze, &end, &path.0);
        print_maze(&walked_maze, &start, &end);

        let all_paths = prelude::astar_bag(
            &start_edge,
            |p: &Edge| p.successors(&maze),
            |p: &Edge| {
                return ((end.x.abs_diff(p.end.x) + end.y.abs_diff(p.end.y)) / 3) as u32;
            },
            |p: &Edge| p.end == end,
        )
        .unwrap();

        let mut all_points: HashSet<Point> = HashSet::new();
        for ideal_path in all_paths.0 {
            for edge in ideal_path {
                all_points.insert(edge.end.clone());
            }
        }
        println!("total_points={}", all_points.len() + 1);
    }
}
