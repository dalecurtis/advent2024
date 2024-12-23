extern crate pathfinding;

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
const CHEAT_SIZE: usize = 20;
const SAVE: usize = 100;

const ALL_PATHS: usize = 125125125;
const VALID_DIR: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn is_valid(point: &Point, extents: &Point) -> bool {
    return point.x < extents.x && point.y < extents.y;
}

fn move_point(p: &Point, dir: (i32, i32)) -> Point {
    let npx = p.x as i32 + dir.0;
    let npy = p.y as i32 + dir.1;
    if npx < 0 || npy < 0 {
        // Don't construct invalid movements.
        return Point {
            x: 100000,
            y: 100000,
        };
    }

    return Point {
        x: npx as usize,
        y: npy as usize,
    };
}

// Implement necessary traits for pathfinding crate
impl Edge {
    fn successors(
        &self,
        start_p: &Point,
        end_p: &Point,
        extents: &Point,
        graph: &Vec<Vec<char>>,
        target_char: char,
        max_len: usize,
    ) -> Vec<Self> {
        let mut successors = Vec::new();
        for dir in VALID_DIR {
            let next_p = move_point(&self.end, dir);
            let d_x = (next_p.x as i32 - start_p.x as i32).abs() as usize;
            let d_y = (next_p.y as i32 - start_p.y as i32).abs() as usize;
            if d_x + d_y > max_len {
                continue;
            }
            if next_p != self.start
                && is_valid(&next_p, &extents)
                && (graph[next_p.y][next_p.x] == target_char || next_p == *end_p)
            {
                successors.push(Edge {
                    start: self.end.clone(),
                    end: next_p.clone(),
                });
            }
        }
        return successors;
    }
}

fn print_maze(maze: &Vec<Vec<char>>, start: &Point, end: &Point) {
    for y in 0..maze.len() {
        for x in 0..maze[y].len() {
            match maze[y][x] {
                '#' => print!("#"),
                'C' => print!("C"),
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

fn build_distance_grid(end: &Point, extents: &Point, maze: &Vec<Vec<char>>) -> Vec<Vec<usize>> {
    let mut distance_grid = vec![vec![0; extents.x]; extents.y];
    for d in VALID_DIR {
        let start_edge = Edge {
            start: end.clone(),
            end: move_point(end, d),
        };

        if !is_valid(&start_edge.end, extents) || maze[start_edge.end.y][start_edge.end.x] != '.' {
            continue;
        }

        let walk_it = prelude::bfs_reach(start_edge, |p: &Edge| {
            p.successors(end, &Point::default(), extents, maze, '.', ALL_PATHS)
        });
        for e in walk_it {
            distance_grid[e.end.y][e.end.x] = distance_grid[e.start.y][e.start.x] + 1;
        }
        break;
    }

    return distance_grid;
}

fn main() {
    let file = File::open("input20.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut start = Point::default();
    let mut end = Point::default();
    let mut possible_cheats = Vec::new();

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
                    possible_cheats.push(Point { x: x, y: y_pos });
                }
                _ => todo!("bad input"),
            }
        }
        maze.push(row);
        y_pos += 1;
    }
    let extents = Point {
        x: maze[0].len(),
        y: maze.len(),
    };

    println!("start={:?}, end={:?}, extents={:?}", start, end, extents);
    print_maze(&maze, &start, &end);

    let distance_grid = build_distance_grid(&end, &extents, &maze);
    if distance_grid[end.y][end.x] != 0 {
        panic!("bad distance grid");
    }

    println!(
        "d[s]={}, d[e]={}",
        distance_grid[start.y][start.x], distance_grid[end.y][end.x]
    );

    let true_path_len = distance_grid[start.y][start.x];
    let mut good_cheats: usize = 0;
    // let mut cheat_results: HashMap<usize, usize> = HashMap::new();
    for i in 0..possible_cheats.len() {
        for j in i + 1..possible_cheats.len() {
            let mut p1 = &possible_cheats[i];
            let mut p2 = &possible_cheats[j];

            let savings;
            if distance_grid[p1.y][p1.x] < distance_grid[p2.y][p2.x] {
                (p1, p2) = (p2, p1);
                savings = distance_grid[p2.y][p2.x] - distance_grid[p1.y][p1.x];
            } else {
                savings = distance_grid[p1.y][p1.x] - distance_grid[p2.y][p2.x]
            }

            if savings < SAVE {
                continue;
            }

            let d_x = (p1.x as i32 - p2.x as i32).abs() as usize;
            let d_y = (p1.y as i32 - p2.y as i32).abs() as usize;
            if d_x + d_y > CHEAT_SIZE {
                continue;
            }

            // Arggggggh, problem is poorly worded, cheat paths can enter walls more
            // than once... so this part is unnecessary...

            // let cheat_paths = compute_path(p1, p2, &extents, &maze, '#', CHEAT_SIZE + 1);
            // let best = cheat_paths.iter().enumerate().min_by_key(|(_, v)| v.len());
            // if best.is_none() {
            //     continue;
            // }

            // let p = best.unwrap().1;
            // if p[0].start != **p1 || p.last().unwrap().end != **p2 {
            //     panic!("bad search");
            // }

            let p_start = &p1;
            let p_end = &p2;
            let cheated_path_len = (true_path_len - distance_grid[p_start.y][p_start.x])
                + distance_grid[p_end.y][p_end.x]
                + d_x
                + d_y;
            if cheated_path_len < true_path_len {
                if true_path_len - cheated_path_len < SAVE {
                    continue;
                }
                good_cheats += 1;
                // cheat_results
                //     .entry(true_path_len - cheated_path_len)
                //     .and_modify(|e| *e += 1)
                //     .or_insert(1);
            }
        }
    }

    // let mut keys: Vec<&usize> = cheat_results.keys().collect();
    // keys.sort();
    // for k in keys {
    //     println!("{:>3}={:>3}", k, cheat_results[k]);
    // }

    println!("good_cheats={}", good_cheats);
}
