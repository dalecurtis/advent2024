extern crate regex;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

#[derive(Clone, Debug, Default)]
struct Robot {
    init_x: i32,
    init_y: i32,
    x_per_sec: i32,
    y_per_sec: i32,
    curr_x: i32,
    curr_y: i32,
}

fn adjacency_score(bots: &HashSet<(i32, i32)>) -> u32 {
    let mut score = 0;
    for &(x, y) in bots {
        // Check neighbors (N, E, S, W)
        if bots.contains(&(x, y + 1)) {
            score += 1; // North
        }
        if bots.contains(&(x + 1, y)) {
            score += 1; // East
        }
        if bots.contains(&(x, y - 1)) {
            score += 1; // South
        }
        if bots.contains(&(x - 1, y)) {
            score += 1; // West
        }
    }

    return score / 2; // Divide by 2 to avoid double counting pairs
}

fn print_state(bots: &Vec<Robot>, width: i32, height: i32) {
    for y in 0..height {
        for x in 0..width {
            let mut cur_bots = 0;
            for b in bots {
                if b.curr_x == x && b.curr_y == y {
                    cur_bots += 1;
                }
            }
            if cur_bots == 0 {
                print!(".");
            } else {
                print!("{}", cur_bots);
            }
        }
        println!();
    }
    println!();
}

fn main() {
    // Open the file
    let file = File::open("input14.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    // Iterate over each line in the file
    let mut bots = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        if line.is_empty() {
            continue;
        }

        // Parses rows of the form:
        //.   p=0,4 v=3,-3
        let re = Regex::new(r"(-?\d+)").unwrap();
        let parsed: Vec<i32> = re
            .captures_iter(&line)
            .map(|cap| cap[0].parse().unwrap())
            .collect();
        let [px, py, vx, vy] = parsed[..] else {
            todo!("wtf");
        };

        bots.push(Robot {
            init_x: px,
            init_y: py,
            x_per_sec: vx,
            y_per_sec: vy,
            curr_x: px,
            curr_y: py,
        });
    }
    const WIDTH: i32 = 101;
    const HEIGHT: i32 = 103;
    // const WIDTH: i32 = 11;  Test data.
    // const HEIGHT: i32 = 7;
    const X_CENTER: i32 = (WIDTH - 1) / 2;
    const Y_CENTER: i32 = (HEIGHT - 1) / 2;

    const PART1_T: i32 = 100;
    for bot in &mut bots {
        bot.curr_x = (bot.curr_x + bot.x_per_sec * PART1_T).rem_euclid(WIDTH);
        bot.curr_y = (bot.curr_y + bot.y_per_sec * PART1_T).rem_euclid(HEIGHT);
    }

    let mut quadrant_counts = [0, 0, 0, 0];
    for bot in &bots {
        // Ignore bots in the center axis.
        if bot.curr_x == X_CENTER || bot.curr_y == Y_CENTER {
            continue;
        }
        if bot.curr_x < X_CENTER {
            if bot.curr_y < Y_CENTER {
                quadrant_counts[0] += 1;
            } else {
                quadrant_counts[1] += 1;
            }
        } else {
            if bot.curr_y < Y_CENTER {
                quadrant_counts[2] += 1;
            } else {
                quadrant_counts[3] += 1;
            }
        }
    }

    println!(
        "part1: quandrant_product={}",
        quadrant_counts.iter().product::<i32>()
    );

    // Part 2
    // -- The semi-smart way after manual find.
    for tn in 1..1000000 {
        let mut points: HashSet<(i32, i32)> = HashSet::new();
        for i in 0..bots.len() {
            let bot = &mut bots[i];
            bot.curr_x = (bot.init_x + tn * bot.x_per_sec).rem_euclid(WIDTH);
            bot.curr_y = (bot.init_y + tn * bot.y_per_sec).rem_euclid(HEIGHT);
            points.insert((bot.curr_x, bot.curr_y));
        }
        let adjacency_score = adjacency_score(&points);
        if adjacency_score > (bots.len() / 2) as u32 {
            println!("t={}, adjacency_score={}", tn, adjacency_score);
            print_state(&bots, WIDTH, HEIGHT);
            break;
        }
    }

    // Original solution:
    //.  1. Print first WIDTH+HEIGHT entries out and look for high entropy patterns.
    //.  2. Notice that 105, 179, 206, 282 are high entropy.
    //.  3. Notice f(t) where t = 4 + WIDTH*n or t = 76 + HEIGHT*n are high entropy.
    //.  4. Visually inspect first 100 patterns with ctrl+f through vim.
    //.  5. Celebrate at 6771. :)

    // for tn in 1..10000 {
    //     if (tn - 4) % WIDTH != 0 && (tn - 76) % HEIGHT != 0 {
    //         continue;
    //     }
    //     println!("t={}", tn);
    //     for i in 0..bots.len() {
    //         let bot = &mut bots[i];
    //         bot.curr_x = (bot.init_x + tn * bot.x_per_sec).rem_euclid(WIDTH);
    //         bot.curr_y = (bot.init_y + tn * bot.y_per_sec).rem_euclid(HEIGHT);
    //     }
    //     print_state(&bots, WIDTH, HEIGHT);
    // }
}
