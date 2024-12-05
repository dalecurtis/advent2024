use std::fs::File;
use std::io::{BufRead, BufReader};

const PAD: char = 0xFF as char;
const MATRIX: [[char; 7]; 7] = [
    ['S', PAD, PAD, 'S', PAD, PAD, 'S'],
    [PAD, 'A', PAD, 'A', PAD, 'A', PAD],
    [PAD, PAD, 'M', 'M', 'M', PAD, PAD],
    ['S', 'A', 'M', 'X', 'M', 'A', 'S'],
    [PAD, PAD, 'M', 'M', 'M', PAD, PAD],
    [PAD, 'A', PAD, 'A', PAD, 'A', PAD],
    ['S', PAD, PAD, 'S', PAD, PAD, 'S'],
];
const PAD_LEN: usize = MATRIX.len() / 2;

fn xor_matrix(mask: [[char; 7]; 7], input: [[char; 7]; 7]) -> [[u8; 7]; 7] {
    let mut result = [[0; 7]; 7];

    for i in 0..7 {
        for j in 0..7 {
            if mask[i][j] as u8 ^ input[i][j] as u8 == 0 {
                result[i][j] = 1;
            } else {
                result[i][j] = 0;
            }
        }
    }

    return result;
}

fn has_match(matrix1: &[[u8; 4]; 4], matrix2: &[[u8; 4]; 4]) -> bool {
    let mut sum = 0;
    for i in 0..4 {
        for j in 0..4 {
            sum += matrix1[i][j] & matrix2[i][j];
        }
    }
    return sum == 4;
}

fn matrix_extract(matrix: &[[u8; 7]; 7], x: usize, y: usize) -> [[u8; 4]; 4] {
    let mut result = [[0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            result[i][j] = matrix[y + i][x + j];
        }
    }
    return result;
}

fn vertical_xmas(input: [[u8; 7]; 7]) -> i32 {
    const VERT_SOLUTION: [[u8; 4]; 4] = [
        [0, 0, 0, 1], // rustfmt
        [0, 0, 0, 1],
        [0, 0, 0, 1],
        [0, 0, 0, 1],
    ];

    let top = matrix_extract(&input, 0, 0);
    let bottom = matrix_extract(&input, 0, 3);
    return has_match(&VERT_SOLUTION, &top) as i32 + has_match(&VERT_SOLUTION, &bottom) as i32;
}

fn horizontal_xmas(input: [[u8; 7]; 7]) -> i32 {
    const HORZ_SOLUTION: [[u8; 4]; 4] = [
        [0, 0, 0, 0], // rustfmt
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [1, 1, 1, 1],
    ];

    let left = matrix_extract(&input, 0, 0);
    let right = matrix_extract(&input, 3, 0);
    return has_match(&HORZ_SOLUTION, &left) as i32 + has_match(&HORZ_SOLUTION, &right) as i32;
}

fn diagnol_xmas(input: [[u8; 7]; 7]) -> i32 {
    const DIAG_SOLUTION1: [[u8; 4]; 4] = [
        [1, 0, 0, 0], // rustfmt
        [0, 1, 0, 0],
        [0, 0, 1, 0],
        [0, 0, 0, 1],
    ];
    const DIAG_SOLUTION2: [[u8; 4]; 4] = [
        [0, 0, 0, 1], // rustfmt
        [0, 0, 1, 0],
        [0, 1, 0, 0],
        [1, 0, 0, 0],
    ];

    let top_left = matrix_extract(&input, 0, 0);
    let bottom_right = matrix_extract(&input, 3, 3);
    let top_right = matrix_extract(&input, 3, 0);
    let bottom_left = matrix_extract(&input, 0, 3);
    return has_match(&DIAG_SOLUTION1, &top_left) as i32
        + has_match(&DIAG_SOLUTION1, &bottom_right) as i32
        + has_match(&DIAG_SOLUTION2, &top_right) as i32
        + has_match(&DIAG_SOLUTION2, &bottom_left) as i32;
}

fn main() {
    // Open the file
    let file = File::open("input4.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut input_matrix = Vec::new();

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

        input_matrix.push(padded_line);
    }

    for _i in 0..PAD_LEN {
        input_matrix.push(" ".repeat(input_matrix[input_matrix.len() - 1].len()));
    }

    let mut xmas_count: i32 = 0;

    for y in PAD_LEN..input_matrix.len() - PAD_LEN {
        for x in PAD_LEN..input_matrix[y].len() - PAD_LEN {
            // Generate source matrix.
            let mut input = [['\0'; 7]; 7];
            for dy in y - PAD_LEN..y + PAD_LEN + 1 {
                for dx in x - PAD_LEN..x + PAD_LEN + 1 {
                    input[(dy + PAD_LEN) - y][(dx + PAD_LEN) - x] =
                        input_matrix[dy].chars().nth(dx).expect("FAIL");
                }
            }

            let result = xor_matrix(MATRIX, input);

            xmas_count += vertical_xmas(result) + horizontal_xmas(result) + diagnol_xmas(result);
        }
    }

    println!("XMAS Count: {}", xmas_count);
}
