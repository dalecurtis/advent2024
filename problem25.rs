use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

type KeyLock = [u8; 5];

fn main() {
    let file = File::open("input25.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut keys: Vec<KeyLock> = Vec::new();
    let mut locks: Vec<KeyLock> = Vec::new();

    let mut lock_count = 0;
    let mut key_count = 0;
    let mut is_lock: Option<bool> = None;
    let mut current_key_lock: Option<KeyLock> = None;
    let mut current_depth = 0;
    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        if line.is_empty() {
            if is_lock.unwrap() {
                lock_count += 1;
                locks.push(current_key_lock.unwrap());
            } else {
                key_count += 1;
                keys.push(current_key_lock.unwrap());
            }
            current_key_lock = None;
            is_lock = None;
            current_depth = 0;
            continue;
        }

        if current_key_lock.is_none() {
            is_lock = Some(line.contains('#'));
            if line != "#####" && line != "....." {
                panic!("bad input");
            }
            current_key_lock = Some([0, 0, 0, 0, 0]);
            continue;
        } else if current_depth == 5 {
            continue;
        }

        current_depth += 1;
        for (i, ch) in line.chars().enumerate() {
            if ch == '#' {
                current_key_lock.as_mut().unwrap()[i] += 1;
                if current_key_lock.as_ref().unwrap()[i] > 5 {
                    panic!("unexpected input");
                }
            }
        }
    }
    if is_lock.unwrap() {
        lock_count += 1;
        locks.push(current_key_lock.unwrap());
    } else {
        key_count += 1;
        keys.push(current_key_lock.unwrap());
    }

    println!("unique_locks={}, total={}", locks.len(), lock_count);
    println!("unique_keys={}, total={}", keys.len(), key_count);

    let mut fit_count = 0;
    for key in &keys {
        for lock in &locks {
            let sum: Vec<_> = lock.iter().zip(key.iter()).map(|(&x, &y)| x + y).collect();
            if sum.iter().all(|x| *x <= 5) {
                fit_count += 1;
            }
        }
    }
    println!("fit_count={}", fit_count);
}
