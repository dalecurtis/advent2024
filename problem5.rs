use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn verify_pages(pages: &Vec<i32>, rules_map: &HashMap<i32, HashSet<i32>>) -> bool {
    for i in (1..pages.len()).rev() {
        let page = pages[i];
        let bad_pages = rules_map.get(&page);
        if bad_pages.is_none() {
            continue;
        }

        for j in (0..i).rev() {
            if bad_pages.unwrap().contains(&pages[j]) {
                return false;
            }
        }
    }
    return true;
}

fn fix_pages(pages: &Vec<i32>, rules_map: &HashMap<i32, HashSet<i32>>) -> Vec<i32> {
    let mut fixed = pages.clone();
    for i in (1..pages.len()).rev() {
        let page = pages[i];
        let bad_pages = rules_map.get(&page);
        if bad_pages.is_none() {
            continue;
        }

        for j in (0..i).rev() {
            if bad_pages.unwrap().contains(&pages[j]) {
                fixed.swap(i, j);
                break;
            }
        }
    }
    return fixed;
}

fn main() {
    // Open the file
    let file = File::open("input5.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut rules_map = HashMap::new();
    let mut middle_sum: i32 = 0;
    let mut middle_sum_fixed: i32 = 0;

    // Iterate over each line in the file
    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        if line.is_empty() {
            continue;
        }

        // Split the line by whitespace
        let rules_str: Vec<&str> = line.split('|').collect();
        if rules_str.len() == 2 {
            let int_rules: Vec<i32> = rules_str
                .into_iter()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            rules_map
                .entry(int_rules[0])
                .or_insert(HashSet::new())
                .insert(int_rules[1]);
            continue;
        }

        let pages_str: Vec<&str> = line.split(',').collect();
        if pages_str.len() > 0 {
            let int_pages: Vec<i32> = pages_str
                .into_iter()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();

            if verify_pages(&int_pages, &rules_map) {
                middle_sum += int_pages[int_pages.len() / 2];
                continue;
            } else {
                let mut new_pages = int_pages;
                loop {
                    new_pages = fix_pages(&new_pages, &rules_map);
                    if verify_pages(&new_pages, &rules_map) {
                        break;
                    }
                }
                middle_sum_fixed += new_pages[new_pages.len() / 2];
            }
        }
    }

    println!(
        "Middle Sum: {}, w/ Fixed Middle Sum: {}",
        middle_sum, middle_sum_fixed
    );
}
