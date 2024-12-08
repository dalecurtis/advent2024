// After cargo build --release, build with rustc -L target/release/deps problem3.rs

extern crate regex;

use regex::Regex;
use std::fs;

fn main() {
    let contents =
        fs::read_to_string("input3.txt").expect("Should have been able to read the file");
    // let contents = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    // let contents = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    let mul_re = Regex::new(r"(do\(\))|(don't\(\))|(mul\(\d+,\d+\))").unwrap();
    let num_re = Regex::new(r"(\d+)").unwrap();

    let mut enabled: bool = true;
    let mut total: i64 = 0;
    for expression in mul_re.captures_iter(&contents) {
        if expression[0] == *"do()" {
            enabled = true;
            continue;
        } else if expression[0] == *"don't()" {
            enabled = false;
            continue;
        }

        if !enabled {
            continue;
        }

        let mut result: i64 = 1;
        for value in num_re.captures_iter(&expression[0]) {
            let num: i64 = (*value.extract::<1>().0).parse().expect("bahg");
            result *= num;
        }

        total += result;
    }
    println!("Match: {}", total);
}
