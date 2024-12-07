use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Default, Clone)]
struct EqPart {
    total: i64,
    values: Vec<i64>,
}

fn mul_op(a: i64, b: i64) -> i64 {
    return a * b;
}

fn add_op(a: i64, b: i64) -> i64 {
    return a + b;
}

fn cat_op(a: i64, b: i64) -> i64 {
    return a * 10_i64.pow((b.ilog10() + 1) as u32) + b;
}

// Finds lines of the form [total = a, b, c, d] which have solutions
// after trying all combinations of `op_array`. Returns the sum and
// unsolved portions.
fn apply_ops(input_vec: Vec<EqPart>, op_array: Vec<fn(i64, i64) -> i64>) -> (i64, Vec<EqPart>) {
    let mut unsolved: Vec<EqPart> = Vec::new();
    let mut sum: i64 = 0;
    for eq in input_vec {
        let mut solved = false;
        let op_count = op_array.len().pow((eq.values.len() - 1) as u32);
        for i in 0..op_count {
            let mut mask = i;
            let mut result = eq.values[0];
            for j in 1..eq.values.len() {
                result = op_array[mask % op_array.len()](result, eq.values[j]);
                if result > eq.total {
                    break;
                }
                mask /= op_array.len();
            }

            if result == eq.total {
                sum += eq.total;
                solved = true;
                break;
            }
        }

        if !solved {
            unsolved.push(eq);
        }
    }
    return (sum, unsolved);
}

fn main() {
    let file = File::open("input7.txt").expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut input = Vec::new();
    let mut sum: i64 = 0;
    let mut total_count: usize = 0;

    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        total_count += 1;

        let eq_parts: Vec<&str> = line.split(':').collect();
        let val_parts: Vec<&str> = eq_parts[1].split_whitespace().collect();

        let total: i64 = eq_parts[0].parse().expect("fail");
        let mut values: Vec<i64> = Vec::new();
        let mut mul_total: i64 = 1;
        let mut sum_total: i64 = 0;
        for v in val_parts {
            let iv: i64 = v.parse().expect("fail");
            mul_total *= iv;
            sum_total += iv;
            values.push(iv);
        }

        // ignore trivial solves.
        if mul_total == total || sum_total == total {
            sum += total;
            continue;
        }

        let part = EqPart {
            total: total,
            values: values,
        };

        input.push(part);
    }

    println!(
        "Solved {}/{}, Sum: {}",
        total_count - input.len(),
        total_count,
        sum
    );

    let op_array = vec![mul_op, add_op];
    let (sum_partial, unsolved) = apply_ops(input, op_array);
    sum += sum_partial;

    println!(
        "Solved {}/{}, Sum: {}",
        total_count - unsolved.len(),
        total_count,
        sum
    );

    let op_array2 = vec![mul_op, add_op, cat_op];
    let (sum_partial2, unsolved2) = apply_ops(unsolved, op_array2);
    sum += sum_partial2;

    println!(
        "Solved {}/{}, Sum: {}",
        total_count - unsolved2.len(),
        total_count,
        sum
    );
}
