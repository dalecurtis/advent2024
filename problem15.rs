extern crate pathfinding;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use pathfinding::prelude;

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, PartialEq)]
enum XmasObject {
    Empty,
    Wall,
}

const SCALE: usize = 2; // Set to 1 for part 1 and 2 for part 2.

// Implement necessary traits for pathfinding crate
impl Point {
    fn successors(&self, dir: (i32, i32), movable_objects: &HashMap<Point, Point>) -> Vec<Self> {
        let mut successors = Vec::new();

        let next_p = move_point(self, (SCALE as i32 * dir.0, dir.1));
        if movable_objects.contains_key(&next_p) {
            successors.push(next_p);
        } else if dir.1 != 0 {
            for i in 1..SCALE as i32 {
                let before_p = move_point(&next_p, (-i, 0));
                let after_p = move_point(&next_p, (i, 0));
                if movable_objects.contains_key(&before_p) {
                    successors.push(before_p);
                }
                if movable_objects.contains_key(&after_p) {
                    successors.push(after_p);
                }
            }
        }
        return successors;
    }
}

fn print_state(
    warehouse: &Vec<Vec<XmasObject>>,
    robot_state: &Point,
    movable_objects: &HashMap<Point, Point>,
) -> usize {
    let mut gps_coord_sum: usize = 0;
    for y in 0..warehouse.len() {
        let mut x = 0;
        while x < warehouse[y].len() {
            if robot_state.x == x && robot_state.y == y {
                print!("@");
            } else {
                match &warehouse[y][x] {
                    XmasObject::Empty => {
                        if movable_objects.contains_key(&Point { x: x, y: y }) {
                            gps_coord_sum += 100 * y + x;
                            print!("{}", "O".repeat(SCALE));
                            x = (x as i32 + SCALE as i32 - 1) as usize;
                        } else {
                            print!(".")
                        }
                    }
                    XmasObject::Wall => print!("#"),
                }
            }
            x += 1;
        }
        println!();
    }
    println!();
    return gps_coord_sum;
}

fn get_next_dir(c: char) -> (i32, i32) {
    match c {
        '^' => return (0, -1),
        '>' => return (1, 0),
        '<' => return (-1, 0),
        'v' => return (0, 1),
        _ => todo!("bad input"),
    };
}

fn move_point(p: &Point, dir: (i32, i32)) -> Point {
    return Point {
        x: (p.x as i32 + dir.0) as usize,
        y: (p.y as i32 + dir.1) as usize,
    };
}

fn main() {
    let file = File::open("input15.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut cur_pos = Point::default();
    let mut warehouse: Vec<Vec<XmasObject>> = Vec::new();
    let mut moves: Vec<char> = Vec::new();
    let mut movable_objects: HashMap<Point, Point> = HashMap::new();

    let mut handle_move_input = false;
    let mut y_pos = 0;
    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        if line.is_empty() {
            handle_move_input = true;
        }

        if handle_move_input {
            moves.extend(line.chars());
            continue;
        }

        let mut row = Vec::new();
        let mut x_pos: usize = 0;
        for c in line.chars() {
            match c {
                '#' => {
                    for _ in 0..SCALE {
                        row.push(XmasObject::Wall);
                        x_pos += 1;
                    }
                }
                '@' | '.' | 'O' => {
                    if c == '@' {
                        cur_pos.x = x_pos;
                        cur_pos.y = y_pos;
                    } else if c == 'O' {
                        movable_objects.insert(
                            Point { x: x_pos, y: y_pos },
                            Point {
                                x: x_pos + (SCALE - 1),
                                y: y_pos,
                            },
                        );
                    }
                    for _ in 0..SCALE {
                        row.push(XmasObject::Empty);
                        x_pos += 1;
                    }
                }
                _ => todo!("bad input"),
            }
        }
        warehouse.push(row);
        y_pos += 1;
    }

    print_state(&warehouse, &cur_pos, &movable_objects);

    // let mut index: i32 = 0;
    for c in moves {
        // println!("move={}, dir={}", index, c);
        // index += 1;

        let move_dir = get_next_dir(c);
        let next_pos = move_point(&cur_pos, move_dir);
        let next_obj = &warehouse[next_pos.y][next_pos.x];
        match &next_obj {
            XmasObject::Wall => {
                // print_state(&warehouse, &cur_pos, &movable_objects);
                continue;
            }
            XmasObject::Empty => {
                // println!("next_pos={:?}, movable_objects={:?}", next_pos, movable_objects);

                // Either the target is an object or it's back half of another.
                let mut next_obj_pos = next_pos.clone();
                if !movable_objects.contains_key(&next_obj_pos) {
                    let mut found = false;
                    if move_dir.1 != 0 || move_dir.0 == -1 {
                        for _ in 1..SCALE {
                            next_obj_pos.x -= 1;
                            if movable_objects.contains_key(&next_obj_pos) {
                                found = true;
                                break;
                            }
                        }
                    }
                    if !found {
                        // It's actually empty.
                        cur_pos = next_pos;
                        // print_state(&warehouse, &cur_pos, &movable_objects);
                        continue;
                    }
                }

                // Collect all nodes touching the target node.
                let nodes_involved: Vec<Point> = prelude::bfs_reach(next_obj_pos.clone(), |p| {
                    p.successors(move_dir, &movable_objects)
                })
                .collect();
                // println!("nodes_involved={:?}", nodes_involved);

                // Update our movable object set.
                let mut new_nodes: HashMap<Point, Point> = HashMap::new();
                let mut immovable = false;
                for n in &nodes_involved {
                    let new_n = move_point(&n, move_dir);
                    for i in 0..SCALE {
                        if &warehouse[new_n.y][new_n.x + i] == &XmasObject::Wall {
                            immovable = true;
                            break;
                        }
                    }
                    if immovable {
                        break;
                    }
                    let v = movable_objects.get(&n).unwrap();
                    new_nodes.insert(new_n, move_point(&v, move_dir));
                }

                if immovable {
                    // println!("move={}, dir={}, no move", index, c);
                    // print_state(&warehouse, &cur_pos, &movable_objects);
                    continue;
                }

                for n in &nodes_involved {
                    movable_objects.remove(n);
                }

                movable_objects.extend(new_nodes);

                // Move our robot!
                cur_pos = next_pos;
                // print_state(&warehouse, &cur_pos, &movable_objects);
            }
        }
    }

    let gps_coord_sum = print_state(&warehouse, &cur_pos, &movable_objects);
    println!("gps_coord_sum={}", gps_coord_sum);
}
