extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Default, Clone)]
struct WalkState {
    start_x: usize,
    start_y: usize,
    dir_x: i8,
    dir_y: i8,
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

    for _i in 0..pad_len {
        input.push(" ".repeat(input[input.len() - 1].len()));
    }

    return input;
}

// Encodes direction as a single value for cycle testing.
fn encode_dir(dir_x: i8, dir_y: i8) -> i8 {
    return 10 * dir_x + dir_y;
}

fn rotate_90_degrees(x: i8, y: i8) -> (i8, i8) {
    (y, -x)
}

// Returns the number of moves to walk to exit or None if no exit exists.
// `walk_fn` is called on moves to '.' out of initial line of sight.
fn walk_maze(
    input_matrix: &Vec<String>,
    mut visited_matrix: Vec<Vec<i8>>,
    mut state: WalkState,
    mut walk_fn: impl FnMut(&WalkState, &WalkState, &Vec<String>, &Vec<Vec<i8>>),
) -> Option<i32> {
    if visited_matrix.is_empty() {
        visited_matrix = vec![vec![0; input_matrix[0].len()]; input_matrix.len()];
    }

    let mut has_los: bool = true;
    let mut moves: i32 = 0;
    visited_matrix[state.start_y][state.start_x] = encode_dir(state.dir_x, state.dir_y);
    loop {
        let next_x = (state.start_x as i32 + state.dir_x as i32) as usize;
        let next_y = (state.start_y as i32 + -state.dir_y as i32) as usize;
        let next_char = input_matrix[next_y].chars().nth(next_x).expect("FAIL");

        match next_char {
            ' ' => {
                break;
            }
            '#' => {
                has_los = false;
                (state.dir_x, state.dir_y) = rotate_90_degrees(state.dir_x, state.dir_y);
            }
            '.' | '<' | '>' | '^' | 'v' => {
                if !has_los && next_char == '.' {
                    let mut next_state = state.clone();
                    next_state.start_x = next_x;
                    next_state.start_y = next_y;
                    walk_fn(&state, &next_state, &input_matrix, &visited_matrix);
                }

                let current_dir = encode_dir(state.dir_x, state.dir_y);
                if visited_matrix[next_y][next_x] == 0 {
                    visited_matrix[next_y][next_x] = current_dir;
                    moves += 1;
                } else {
                    if visited_matrix[next_y][next_x] == current_dir {
                        return None;
                    }
                }
                state.start_x = next_x;
                state.start_y = next_y;
            }
            _ => todo!(),
        }
    }
    return Some(moves);
}

fn main() {
    let input_matrix = create_padded_input("input6.txt", 1);
    let mut init_state = WalkState::default();
    let find_re = Regex::new(r"(\^|<|>|v)").unwrap();

    for (index, line) in input_matrix.iter().enumerate() {
        for m in find_re.find_iter(&line) {
            init_state.start_x = m.start();
            init_state.start_y = index;
            match m.as_str() {
                "^" => {
                    init_state.dir_x = 0;
                    init_state.dir_y = 1;
                }
                ">" => {
                    init_state.dir_x = 1;
                    init_state.dir_y = 0;
                }
                "<" => {
                    init_state.dir_x = -1;
                    init_state.dir_y = 0;
                }
                "v" => {
                    init_state.dir_x = 0;
                    init_state.dir_y = -1;
                }
                _ => todo!(),
            }
        }
    }

    println!("init_state: {:?}", init_state);

    let do_nothing_fn = |_: &WalkState, _: &WalkState, _: &Vec<String>, _: &Vec<Vec<i8>>| {};
    let moves = walk_maze(&input_matrix, Vec::new(), init_state.clone(), do_nothing_fn);
    println!("moves: {}", moves.expect("FAIL!") + 1);

    let mut barrels: Vec<Vec<bool>> = vec![vec![false; input_matrix[0].len()]; input_matrix.len()];
    let mut cycles: i32 = 0;

    let cycle_finder_fn = |cur_state: &WalkState,
                           next_state: &WalkState,
                           cur_input_matrix: &Vec<String>,
                           cur_visited_matrix: &Vec<Vec<i8>>| {
        // Ensure we don't try to place a barrel in the same spots.
        if barrels[next_state.start_y][next_state.start_x] {
            return;
        }
        barrels[next_state.start_y][next_state.start_x] = true;

        let mut modified_input = cur_input_matrix.clone();
        modified_input[next_state.start_y]
            .replace_range(next_state.start_x..next_state.start_x + 1, "#");

        // When there are no other intersecting paths we can reuse
        // `cur_visited_matrix` to avoid rewalking.
        if cur_visited_matrix[next_state.start_y][next_state.start_x] == 0 {
            if walk_maze(
                &modified_input,
                cur_visited_matrix.clone(),
                cur_state.clone(),
                do_nothing_fn,
            )
            .is_none()
            {
                cycles += 1;
            }
            return;
        }
        // We need to rewalk from the beginning to see how the placement changes
        // the graph.
        if walk_maze(
            &modified_input,
            Vec::new(),
            init_state.clone(),
            do_nothing_fn,
        )
        .is_none()
        {
            cycles += 1;
        }
    };

    walk_maze(
        &input_matrix,
        Vec::new(),
        init_state.clone(),
        cycle_finder_fn,
    );
    println!("Cycles: {}", cycles);
}
