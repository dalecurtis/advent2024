extern crate regex;

use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

use std::collections::HashMap;

fn count_possible_constructions(towels: &str, patterns: &[&str]) -> usize {
    let mut dp: HashMap<usize, usize> = HashMap::new();
    dp.insert(0, 1);

    for i in 1..towels.len() + 1 {
        for pattern in patterns {
            if i < pattern.len() {
                continue;
            }

            let pattern_index = i - pattern.len();
            if towels[pattern_index..i] == **pattern {
                let prev_count = *dp.get(&pattern_index).unwrap_or(&0);
                *dp.entry(i).or_insert(0) += prev_count;
            }
        }
    }

    return *dp.get(&towels.len()).unwrap_or(&0);
}

fn main() {
    let file = File::open("input19.txt").expect("Unable to open file");
    let reader = BufReader::new(file);
    let lines: Vec<_> = reader.lines().map(|l| l.unwrap()).collect();

    let mut patterns: Vec<_> = lines[0]
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    // Original part 1
    //
    // // Sort patterns by length (longest first)
    // patterns.sort_by(|a, b| b.len().cmp(&a.len()));
    //
    //
    // let regex_pattern = patterns
    //     .iter()
    //     .map(|p| regex::escape(p))
    //     .collect::<Vec<_>>()
    //     .join("|");
    //     let anchored_pattern = format!("^({})+$", regex_pattern);

    // println!("test_pattern={:?}", anchored_pattern);
    // let re = Regex::new(&anchored_pattern).unwrap();

    let mut possible_matches = 0;
    let mut total_combinations = 0;
    for line in &lines[2..] {
        let count = count_possible_constructions(line, &patterns);
        if count > 0 {
            possible_matches += 1;
            total_combinations += count;
        }
    }

    println!(
        "possible matches: {}, total_comb={}",
        possible_matches, total_combinations
    );
}
