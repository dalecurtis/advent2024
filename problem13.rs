extern crate regex;

use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

// Holds a pair of diophantine equations of the form:
//.    x1 * a + x2 * b = sol_x
//.    y1 * a + y2 * b = sol_y
#[derive(Debug, Default)]
struct DEq {
    x1: i128,
    x2: i128,
    sol_x: i128,

    y1: i128,
    y2: i128,
    sol_y: i128,
}

// Find solutions to a linear diophantine equations. Returns None if
// no integer solutions exist.
//
// Post-solve edit for future reference: After seeing other solutions,
// I should have used https://en.wikipedia.org/wiki/Cramer%27s_rule
fn solve_deq(eq: &DEq, pad: i128) -> Option<(i128, i128)> {
    let sol_x = eq.sol_x + pad;
    let sol_y = eq.sol_y + pad;

    // Translate the equations into an augmented matrix of the form
    //.    [x1 x2 x_pos]
    //.    [y1 y2 y_pos]
    //
    // Note: Technically we don't need the first row or even a matrix since
    // there are only two equations, but it's left for extension later.
    let mut m = [[eq.x1, eq.x2, sol_x], [eq.y1, eq.y2, sol_y]];
    let temp = m[1].clone();

    // Reduce the second row of the matrix to get a 0 in the first column. Below
    // we'll apply the following operations:
    //.    [x1         x2         x_pos.     ]
    //.    [y1 * x1    y2 * x1    y_pos * x1 ]
    for i in 0..m[1].len() {
        m[1][i] *= m[0][0];
    }

    // Now we apply:
    //.    [x1                   x2                   x_pos.                  ]
    //.    [y1 * x1 - y1 * x1    y2 * x1 - y1 * x2    y_pos * x1 - y1 * x_pos ]
    for i in 0..m[1].len() {
        m[1][i] -= temp[0] * m[0][i];
    }

    // Which leaves us with:
    //.    [x1                   x2                   x_pos.                  ]
    //.    [0                    y2 * x1 - y1 * x2    y_pos * x1 - y1 * x_pos ]
    //
    // Which leaves us with the solution:
    //.    b = y_pos * x1 - y1 * x_pos / y2 * x1 - y1 * x2
    //

    // Note: We never update the first row, but would need to if this system had more
    // equations in it.

    // We're only interested in integer solutions.
    if m[1][2] % m[1][1] == 0 {
        let b = m[1][2] / m[1][1];
        if (m[0][2] - m[0][1] * b) % m[0][0] == 0 {
            // Substitute `b` in the original equation to find the a value.
            let a = (m[0][2] - m[0][1] * b) / m[0][0];
            return Some((a, b));
        }
    }

    return None;
}

fn main() {
    // Open the file
    let file = File::open("input13.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    // Iterate over each line in the file
    let mut deqs = Vec::new();
    let mut param_index = 0;
    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        if line.is_empty() {
            continue;
        }

        let idx = param_index % 3;
        if idx == 0 {
            deqs.push(DEq::default());
        };

        // Parses rows of the form:
        //   Button A: X+94, Y+34
        //   Button B: X+22, Y+67
        //   Prize: X=8400, Y=5400
        // Into
        //.  94a + 22b = 8400
        //   34a + 67b = 5400
        let re = Regex::new(r".*X[\+=]([+-]?\d+), Y[\+=]([+-]?\d+)").unwrap();
        let [x, y] = re
            .captures(&line)
            .expect("broken_re")
            .extract::<2>()
            .1
            .map(|s: &str| s.parse::<i128>().expect("bad_input"));
        let last_mut = deqs.last_mut().unwrap();
        match idx {
            0 => {
                last_mut.x1 = x;
                last_mut.y1 = y;
            }
            1 => {
                last_mut.x2 = x;
                last_mut.y2 = y;
            }
            2 => {
                last_mut.sol_x = x;
                last_mut.sol_y = y;
            }
            _ => todo!("wtf"),
        }
        param_index += 1;
    }

    let cost_fn = |a: i128, b: i128| 3 * a + b;

    let mut part1_solved = 0;
    let mut part1_token_cost: i128 = 0;

    let mut part2_solved = 0;
    let mut part2_token_cost: i128 = 0;
    const PART2_PAD: i128 = 10000000000000;

    for eq in deqs {
        match solve_deq(&eq, 0) {
            Some((a, b)) => {
                if a <= 100 && b <= 100 {
                    part1_token_cost += cost_fn(a, b);
                    part1_solved += 1;

                    println!(
                        "{}a + {}a = {}.x, {}b + {}b = {}.y => a={}, b={}",
                        eq.x1, eq.x2, eq.sol_x, eq.y1, eq.y2, eq.sol_y, a, b
                    );
                }
            }
            None => {}
        }

        match solve_deq(&eq, PART2_PAD) {
            Some((a, b)) => {
                part2_token_cost += cost_fn(a, b);
                part2_solved += 1;
                println!(
                    "{}a + {}a = {}.x, {}b + {}b = {}.y => a={}, b={}",
                    eq.x1,
                    eq.x2,
                    eq.sol_x + PART2_PAD,
                    eq.y1,
                    eq.y2,
                    eq.sol_y + PART2_PAD,
                    a,
                    b
                );
            }
            None => {}
        }
    }
    println!();
    println!(
        "part1: token_cost={}, solvable={}",
        part1_token_cost, part1_solved
    );
    println!(
        "part2: token_cost={}, solvable={}",
        part2_token_cost, part2_solved
    );
}
