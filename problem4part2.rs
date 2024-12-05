use std::fs::File;
use std::io::{BufRead, BufReader};

const PAD: char = 0xFF as char;
const MATRIX1: [[char; 3]; 3] = [
    ['S', PAD, 'S'], // rustfmt
    [PAD, 'A', PAD],
    ['M', PAD, 'M'],
];
const MATRIX2: [[char; 3]; 3] = [
    ['M', PAD, 'S'], // rustfmt
    [PAD, 'A', PAD],
    ['M', PAD, 'S'],
];
const MATRIX3: [[char; 3]; 3] = [
    ['M', PAD, 'M'], // rustfmt
    [PAD, 'A', PAD],
    ['S', PAD, 'S'],
];
const MATRIX4: [[char; 3]; 3] = [
    ['S', PAD, 'M'], // rustfmt
    [PAD, 'A', PAD],
    ['S', PAD, 'M'],
];

const SOLUTION: [[u8; 3]; 3] = [
    [1, 0, 1], // rustfmt
    [0, 1, 0],
    [1, 0, 1],
];

const PAD_LEN: usize = MATRIX1.len() / 2;

fn xor_matrix(mask: [[char; 3]; 3], input: [[char; 3]; 3]) -> [[u8; 3]; 3] {
    let mut result = [[0; 3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            if mask[i][j] as u8 ^ input[i][j] as u8 == 0 {
                result[i][j] = 1;
            } else {
                result[i][j] = 0;
            }
        }
    }

    return result;
}

fn has_match(matrix1: &[[u8; 3]; 3], matrix2: &[[u8; 3]; 3]) -> bool {
    let mut sum = 0;
    for i in 0..3 {
        for j in 0..3 {
            sum += matrix1[i][j] & matrix2[i][j];
        }
    }
    return sum == 5;
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
            let mut input = [['\0'; 3]; 3];
            for dy in y - PAD_LEN..y + PAD_LEN + 1 {
                for dx in x - PAD_LEN..x + PAD_LEN + 1 {
                    input[(dy + PAD_LEN) - y][(dx + PAD_LEN) - x] =
                        input_matrix[dy].chars().nth(dx).expect("FAIL");
                }
            }

            xmas_count += has_match(&SOLUTION, &xor_matrix(MATRIX1, input)) as i32
                + has_match(&SOLUTION, &xor_matrix(MATRIX2, input)) as i32
                + has_match(&SOLUTION, &xor_matrix(MATRIX3, input)) as i32
                + has_match(&SOLUTION, &xor_matrix(MATRIX4, input)) as i32;
        }
    }

    println!("XMAS Count: {}", xmas_count);
}
