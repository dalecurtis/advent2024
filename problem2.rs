use std::fs::File;
use std::io::{BufRead, BufReader};

fn verify_parts(parts: Vec<&str>) -> bool {
    let mut num_neg: usize = 0;
    let mut num_pos: usize = 0;
    for i in 0..(parts.len() - 1) {
        let num1: i32 = parts[i].parse().expect("Invalid number format");
        let num2: i32 = parts[i + 1].parse().expect("Invalid number format");

        let diff = num1 - num2;
        if diff <= 3 && diff >= 1 {
            num_pos += 1;
        } else if diff >= -3 && diff <= -1 {
            num_neg += 1;
        }
    }

    return (num_pos == 0 && num_neg == parts.len() - 1)
        || (num_neg == 0 && num_pos == parts.len() - 1);
}

fn main() {
    // Open the file
    let file = File::open("input2.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut num_correct: i32 = 0;
    let mut num_correct_with_drops: i32 = 0;

    // Iterate over each line in the file
    for line in reader.lines() {
        let line = line.expect("Unable to read line");

        // Split the line by whitespace
        let parts: Vec<&str> = line.split_whitespace().collect();

        if verify_parts(parts.clone()) {
            num_correct += 1;
            continue;
        }

        for i in 0..parts.len() {
            let mut parts2 = parts.clone();
            parts2.remove(i);
            if verify_parts(parts2) {
                num_correct_with_drops += 1;
                break;
            }
        }
    }

    // Print the numbers
    println!(
        "Correct: {}, Semicorrect: {}, Total: {}",
        num_correct,
        num_correct_with_drops,
        num_correct + num_correct_with_drops
    );
}
