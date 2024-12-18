extern crate pathfinding;

use std::collections::VecDeque;
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

const INIT_ROCKS: usize = 1024;
const GRID_SIZE: usize = 70;
const VALID_DIR: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn is_valid(point: &Point) -> bool {
    return point.x <= GRID_SIZE && point.y <= GRID_SIZE;
}

fn move_point(p: &Point, dir: (i32, i32)) -> Point {
    let npx = p.x as i32 + dir.0;
    let npy = p.y as i32 + dir.1;
    if npx < 0 || npy < 0 {
        // Don't construct invalid movements.
        return Point {
            x: GRID_SIZE + 100,
            y: GRID_SIZE + 100,
        };
    }

    return Point {
        x: npx as usize,
        y: npy as usize,
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
            if next_p == self.start || !is_valid(&next_p) || graph[next_p.y][next_p.x] != '.' {
                continue;
            }

            let cost;
            if dir == current_dir {
                cost = 1;
            } else {
                cost = 1;
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
    println!();
}

fn main() {
    let file = File::open("input18.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let start = Point::default();
    let end = Point {
        x: GRID_SIZE,
        y: GRID_SIZE,
    };

    let mut maze: Vec<Vec<_>> = vec![vec!['.'; GRID_SIZE + 1]; GRID_SIZE + 1];

    let mut rocks: VecDeque<Point> = VecDeque::new();
    for line in reader.lines() {
        let line = line.expect("Unable to read line");

        let point_xy: Vec<usize> = line
            .split(',')
            .into_iter()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        let rock = Point {
            x: point_xy[0],
            y: point_xy[1],
        };

        rocks.push_back(rock);
    }

    print_maze(&maze, &start, &end);

    let init_maze = maze.clone();
    let init_rocks = rocks.clone();
    for r in rocks.drain(0..INIT_ROCKS) {
        maze[r.y][r.x] = '#';
    }

    print_maze(&maze, &start, &end);

    for d in VALID_DIR {
        let start_edge = Edge {
            start: start.clone(),
            end: move_point(&start, d),
        };
        if !is_valid(&start_edge.end) || maze[start_edge.end.y][start_edge.end.x] != '.' {
            continue;
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
        println!("len={}, cost={}", path.0.len(), path.1);

        let walked_maze = walk_maze(&maze, &end, &path.0);
        print_maze(&walked_maze, &start, &end);
        break;
    }

    maze = init_maze;
    rocks = init_rocks;
    loop {
        let r = rocks.pop_front();
        if r.is_none() {
            panic!("wtf");
        }

        let rock = r.unwrap();
        maze[rock.y][rock.x] = '#';

        let mut no_sol = true;
        for d in VALID_DIR {
            let start_edge = Edge {
                start: start.clone(),
                end: move_point(&start, d),
            };
            if !is_valid(&start_edge.end) || maze[start_edge.end.y][start_edge.end.x] != '.' {
                continue;
            }
            let result = prelude::astar(
                &start_edge,
                |p: &Edge| p.successors(&maze),
                |p: &Edge| {
                    return ((end.x.abs_diff(p.end.x) + end.y.abs_diff(p.end.y)) / 3) as u32;
                },
                |p: &Edge| p.end == end,
            );

            if result.is_some() {
                no_sol = false;
                break;
            }
        }
        if no_sol {
            print_maze(&maze, &start, &end);
            println!("bad rock: {:?}", rock);
            return;
        }
    }
}
