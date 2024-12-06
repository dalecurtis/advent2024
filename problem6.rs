extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

const PAD_LEN: usize = 1;

// Encodes direction as a single value for cycle testing.
fn dir_code(dir_x: i8, dir_y: i8) -> i8 {
    return 10 * dir_x + dir_y;
}

fn rotate_90_degrees(x: i8, y: i8) -> (i8, i8) {
    (y, -x)
}

// Returns the number of moves to walk to exit, or -1 if no exit exists (cycle).
fn walk_maze(
    input_matrix: &Vec<String>,
    visited_matrix: &mut Vec<Vec<i8>>,
    mut start_x: usize,
    mut start_y: usize,
    mut dir_x: i8,
    mut dir_y: i8,
) -> i32 {
    let mut moves: i32 = 0;
    visited_matrix[start_y][start_x] = dir_code(dir_x, dir_y);
    loop {
        let next_x = (start_x as i32 + dir_x as i32) as usize;
        let next_y = (start_y as i32 + -dir_y as i32) as usize;
        let next_char = input_matrix[next_y].chars().nth(next_x).expect("FAIL");
        match next_char {
            ' ' => {
                break;
            }
            '#' => {
                (dir_x, dir_y) = rotate_90_degrees(dir_x, dir_y);
            }
            '.' | '<' | '>' | '^' | 'v' => {
                if visited_matrix[next_y][next_x] == 0 {
                    visited_matrix[next_y][next_x] = dir_code(dir_x, dir_y);
                    moves += 1;
                } else {
                    if visited_matrix[next_y][next_x] == dir_code(dir_x, dir_y) {
                        return -1;
                    }
                }
                start_x = next_x;
                start_y = next_y;
            }
            _ => todo!(),
        }
    }
    return moves;
}

fn main() {
    // Open the file
    let file = File::open("input6.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut input_matrix = Vec::new();

    let mut start_x: usize = 0;
    let mut start_y: usize = 0;
    let mut dir_x: i8 = 0;
    let mut dir_y: i8 = 0;

    let find_re = Regex::new(r"(\^|<|>|v)").unwrap();

    // Create padded input.
    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let padding = " ".repeat(PAD_LEN);

        let padded_line = padding.clone() + &line + &padding;

        if input_matrix.is_empty() {
            for _i in 0..PAD_LEN {
                input_matrix.push(" ".repeat(padded_line.len()));
            }
        }

        for m in find_re.find_iter(&padded_line) {
            start_x = m.start();
            start_y = input_matrix.len();
            match m.as_str() {
                "^" => {
                    dir_x = 0;
                    dir_y = 1;
                }
                ">" => {
                    dir_x = 1;
                    dir_y = 0;
                }
                "<" => {
                    dir_x = -1;
                    dir_y = 0;
                }
                "v" => {
                    dir_x = 0;
                    dir_y = -1;
                }
                _ => {}
            }
        }

        input_matrix.push(padded_line);
    }

    for _i in 0..PAD_LEN {
        input_matrix.push(" ".repeat(input_matrix[input_matrix.len() - 1].len()));
    }

    println!(
        "start_x: {}, start_y: {}, dir_x: {}, dir_y: {}",
        start_x, start_y, dir_x, dir_y
    );

    let init_x = start_x;
    let init_y = start_y;
    let init_dir_x = dir_x;
    let init_dir_y = dir_y;

    let mut visited_matrix: Vec<Vec<i8>> = vec![vec![0; input_matrix[0].len()]; input_matrix.len()];
    let moves = walk_maze(
        &input_matrix,
        &mut visited_matrix,
        start_x,
        start_y,
        dir_x,
        dir_y,
    );
    println!("moves: {}", moves + 1);

    visited_matrix = vec![vec![0; input_matrix[0].len()]; input_matrix.len()];
    visited_matrix[start_y][start_x] = dir_code(dir_x, dir_y);

    let mut barrels: Vec<Vec<bool>> = vec![vec![false; input_matrix[0].len()]; input_matrix.len()];
    let mut cycles: i32 = 0;
    let mut has_los: bool = true;
    loop {
        let next_x = (start_x as i32 + dir_x as i32) as usize;
        let next_y = (start_y as i32 + -dir_y as i32) as usize;
        let next_char = input_matrix[next_y].chars().nth(next_x).expect("FAIL");
        match next_char {
            ' ' => {
                break;
            }
            '#' => {
                has_los = false;
                (dir_x, dir_y) = rotate_90_degrees(dir_x, dir_y);
            }
            '.' | '<' | '>' | '^' | 'v' => {
                if !has_los && next_char == '.' && !barrels[next_y][next_x] {
                    barrels[next_y][next_x] = true;
                    input_matrix[next_y].replace_range(next_x..next_x + 1, "#");

                    // When there are no other intersecting paths we can reuse the existing
                    // visiting tree to avoid rewalking.
                    if visited_matrix[next_y][next_x] == 0 {
                        let mut modified_visits = visited_matrix.clone();
                        if walk_maze(
                            &input_matrix,
                            &mut modified_visits,
                            start_x,
                            start_y,
                            dir_x,
                            dir_y,
                        ) == -1
                        {
                            cycles += 1;
                        }
                    } else {
                        // We need to rewalk from the beginning to see how the placement changes
                        // the graph.
                        let mut modified_visits =
                            vec![vec![0; input_matrix[0].len()]; input_matrix.len()];
                        if walk_maze(
                            &input_matrix,
                            &mut modified_visits,
                            init_x,
                            init_y,
                            init_dir_x,
                            init_dir_y,
                        ) == -1
                        {
                            cycles += 1;
                        }
                    }
                    input_matrix[next_y].replace_range(next_x..next_x + 1, ".");
                }
                visited_matrix[next_y][next_x] = dir_code(dir_x, dir_y);
                start_x = next_x;
                start_y = next_y;
            }
            _ => todo!(),
        }
    }

    println!("Cycles: {}", cycles);
}
