extern crate binary_search;
extern crate pathfinding;

use std::fs::File;
use std::io::{BufRead, BufReader};

use binary_search::{binary_search, Direction};
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
    fn successors(&self, graph: &Vec<Vec<char>>) -> Vec<Self> {
        let mut successors = Vec::new();
        for dir in VALID_DIR {
            let next_p = move_point(&self.end, dir);
            if next_p == self.start || !is_valid(&next_p) || graph[next_p.y][next_p.x] != '.' {
                continue;
            }
            successors.push(Edge {
                start: self.end.clone(),
                end: next_p.clone(),
            });
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

    let mut rocks: Vec<Point> = Vec::new();
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

        rocks.push(rock);
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

        let result = prelude::bfs(
            &start_edge,
            |p: &Edge| p.successors(&maze),
            |p: &Edge| p.end == end,
        );
        if result.is_none() {
            continue;
        }

        let path = result.unwrap();
        println!("len={}", path.len());

        let walked_maze = walk_maze(&maze, &end, &path);
        print_maze(&walked_maze, &start, &end);
        break;
    }

    // Binary search through the solvable mazes until we find one unsolvable.
    let (bad_rock_index, _) =
        binary_search((0, ()), (init_rocks.len(), ()), |rocks_to_drop: usize| {
            let mut test_maze = init_maze.clone();
            for i in 0..rocks_to_drop {
                let r = &init_rocks[i];
                test_maze[r.y][r.x] = '#';
            }

            for d in VALID_DIR {
                let start_edge = Edge {
                    start: start.clone(),
                    end: move_point(&start, d),
                };
                if !is_valid(&start_edge.end)
                    || test_maze[start_edge.end.y][start_edge.end.x] != '.'
                {
                    continue;
                }
                let result = prelude::bfs(
                    &start_edge,
                    |p: &Edge| p.successors(&test_maze),
                    |p: &Edge| p.end == end,
                );

                if result.is_some() {
                    return Direction::Low(());
                }
            }
            return Direction::High(());
        });

    println!("bad rock: {:?}", init_rocks[bad_rock_index.0]);
}
