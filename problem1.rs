use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Open the file
    let file = File::open("input1.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut sum1: i32 = 0;
    let mut sum2: i32 = 0;
    let mut heap1 = BinaryHeap::new();
    let mut heap2 = BinaryHeap::new();
    let mut map2 = HashMap::new();

    // Iterate over each line in the file
    for line in reader.lines() {
        let line = line.expect("Unable to read line");

        // Split the line by whitespace
        let parts: Vec<&str> = line.split_whitespace().collect();

        // Check if there are two numbers
        if parts.len() != 2 {
            continue;
        }

        // Parse the numbers
        let num1: i32 = parts[0].parse().expect("Invalid number format");
        let num2: i32 = parts[1].parse().expect("Invalid number format");

        sum1 += num1;
        sum2 += num2;

        heap1.push(Reverse(num1));
        heap2.push(Reverse(num2));

        *map2.entry(num2).or_insert(0) += 1;
    }

    let mut distance_sum: i32 = 0;
    let mut sim_score: i32 = 0;

    while !heap1.is_empty() {
        let h1 = heap1.pop().unwrap().0;
        let h2 = heap2.pop().unwrap().0;
        let count = *map2.entry(h1).or_insert(0);
        sim_score += h1 * count;
        distance_sum += (h1 - h2).abs();
    }

    // Print the numbers
    println!(
        "Number 1: {}, Number 2: {}, Distance Sum: {}, Sim Score: {}",
        sum1, sum2, distance_sum, sim_score
    );
}
