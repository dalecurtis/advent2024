extern crate pathfinding;

use pathfinding::prelude;
use std::collections::HashMap;

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

// const VALID_DIR: [[i32; 2]; 4] = [[1, 0], [0, -1], [0, 1], [-1, 0]];
const VALID_DIR: [(i32, i32); 4] = [(-1, 0), (0, 1), (0, -1), (1, 0)];

const CODE_GRID: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    ['.', '0', 'A'],
];
const CODE_EXTENTS: Point = Point { x: 3, y: 4 };
const CODE_START_POS: Point = Point { x: 2, y: 3 };

const PAD_GRID: [[char; 3]; 4] = [
    ['.', '^', 'A'], // nowrap
    ['<', 'v', '>'],
    ['.', '.', '.'], // lazy way to share params
    ['.', '.', '.'],
];
const PAD_EXTENTS: Point = Point { x: 3, y: 2 };
const PAD_START_POS: Point = Point { x: 2, y: 0 };

fn is_valid(point: &Point, extents: &Point) -> bool {
    if *extents == CODE_EXTENTS && point.x == 0 && point.y == 3 {
        return false;
    }
    if *extents == PAD_EXTENTS && point.x == 0 && point.y == 0 {
        return false;
    }
    return point.x < extents.x && point.y < extents.y;
}

impl Point {
    fn successors(&self, extents: &Point) -> Vec<(Self, u32)> {
        let mut successors = Vec::new();
        for dir in VALID_DIR {
            let next_x = self.x as i32 + dir.0;
            let next_y = self.y as i32 + dir.1;
            if next_x < 0 || next_y < 0 {
                continue;
            }

            let p = Point {
                x: next_x as usize,
                y: next_y as usize,
            };

            if is_valid(&p, extents) {
                successors.push((p, 1));
            }
        }
        return successors;
    }
}

fn compute_dir(p0: &Point, p1: &Point) -> char {
    match (p1.x as i32 - p0.x as i32, p1.y as i32 - p0.y as i32) {
        (1, 0) => return '>',
        (-1, 0) => return '<',
        (0, 1) => return 'v',
        (0, -1) => return '^',
        _ => panic!("bad input"),
    }
}

fn grid_index(ch: char) -> Point {
    match ch {
        '7' | '8' | '9' => {
            return Point {
                x: ch.to_digit(10).unwrap() as usize - 7,
                y: 0,
            }
        }
        '4' | '5' | '6' => {
            return Point {
                x: ch.to_digit(10).unwrap() as usize - 4,
                y: 1,
            }
        }
        '1' | '2' | '3' => {
            return Point {
                x: ch.to_digit(10).unwrap() as usize - 1,
                y: 2,
            }
        }
        '0' => return Point { x: 1, y: 3 },
        'A' => return Point { x: 2, y: 3 },
        _ => panic!("bad grid index"),
    }
}

fn code_index(ch: char) -> Point {
    match ch {
        '^' => return Point { x: 1, y: 0 },
        '<' => return Point { x: 0, y: 1 },
        'v' => return Point { x: 1, y: 1 },
        '>' => return Point { x: 2, y: 1 },
        'A' => return Point { x: 2, y: 0 },
        _ => panic!("bad code index"),
    }
}

#[allow(dead_code)]
fn sol_string(path: &Vec<Point>) -> String {
    let mut result = String::new();
    for i in 1..path.len() {
        let next_target_ch = compute_dir(&path[i - 1], &path[i]);
        result.push(next_target_ch);
    }
    return result;
}

fn encode(
    target_ch: char,
    start_pos: &Point,
    cache: &mut HashMap<(Point, char, usize), usize>,
    depth: usize,
    max_depth: usize,
) -> usize {
    if depth == max_depth {
        return 1;
    }

    let cache_key = (start_pos.clone(), target_ch, depth);
    {
        let cached = cache.get(&cache_key);
        if cached.is_some() {
            return *cached.unwrap();
        }
    }

    let extents;
    let grid;
    if depth == 0 {
        extents = &CODE_EXTENTS;
        grid = &CODE_GRID;
    } else {
        extents = &PAD_EXTENTS;
        grid = &PAD_GRID;
    }
    let paths: Vec<_> = prelude::astar_bag(
        &start_pos.clone(),
        |p: &Point| p.successors(extents),
        |p: &Point| {
            return (start_pos.x.abs_diff(p.x) + start_pos.y.abs_diff(p.y)) as u32;
        },
        |p: &Point| grid[p.y][p.x] == target_ch,
    )
    .unwrap()
    .0
    .collect();

    let mut min_path_len = usize::MAX;
    for path in paths {
        let mut cur_path_len = 0;
        let mut current_pos = PAD_START_POS.clone();
        for i in 1..path.len() {
            let next_target_ch = compute_dir(&path[i - 1], &path[i]);
            cur_path_len += encode(next_target_ch, &current_pos, cache, depth + 1, max_depth);
            current_pos = code_index(next_target_ch);
        }
        cur_path_len += encode('A', &current_pos, cache, depth + 1, max_depth);
        if cur_path_len < min_path_len {
            min_path_len = cur_path_len;
        }
    }

    if cache.insert(cache_key, min_path_len).is_some() {
        panic!("cache entry shouldn't be here...");
    }
    return min_path_len;
}

fn main() {
    // test
    // let codes = vec!["029A", "980A", "179A", "456A", "379A"];

    // real
    let codes = vec!["279A", "341A", "459A", "540A", "085A"]; // real

    const ROBOT_KEYPADS: usize = 25; // 2 for part 1, 25 for part 2.

    let mut cache: HashMap<(Point, char, usize), usize> = HashMap::new();
    let mut sum = 0;
    for code in codes {
        let code_num = code[0..3].parse::<usize>().unwrap();

        let mut len = 0;
        let mut current_pos = CODE_START_POS.clone();
        for ch in code.chars() {
            len += encode(ch, &current_pos, &mut cache, 0, ROBOT_KEYPADS + 1);
            current_pos = grid_index(ch);
        }
        sum += len * code_num;
        println!("{}, len={}, grade={}", code, len, len * code_num);
    }
    println!("sum={}", sum);
}
