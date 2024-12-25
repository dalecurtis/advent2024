extern crate itertools;
extern crate rand;

use rand::Rng;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

type GateFn = fn(u8, u8) -> u8;
fn or_inst(a: u8, b: u8) -> u8 {
    return a | b;
}
fn and_inst(a: u8, b: u8) -> u8 {
    return a & b;
}
fn xor_inst(a: u8, b: u8) -> u8 {
    return a ^ b;
}
const INSTRUCTIONS: [GateFn; 3] = [or_inst, and_inst, xor_inst];

#[derive(Clone, Debug)]
struct Op {
    gate_fn: u8,
    var1: String,
    var2: String,
    out: String,
}

#[allow(dead_code)]
fn print_op(op: &Op) {
    let fn_str = match op.gate_fn {
        0 => "OR",
        1 => "AND",
        2 => "XOR",
        _ => panic!("bad input"),
    };
    println!("{} {} {} -> {}", op.var1, fn_str, op.var2, op.out);
}

fn gen_sol(ch: char, var_count: usize, vars: &HashMap<String, Option<u8>>) -> u64 {
    let mut val: u64 = 0;
    for z in (0..var_count).rev() {
        let k = format!("{}{:>02}", ch, z);
        val = (val << 1) | vars.get(&k).unwrap().unwrap() as u64;
    }
    return val;
}

fn encode_var(ch: char, mut val: u64, var_count: usize, vars: &mut HashMap<String, Option<u8>>) {
    for n in 0..var_count {
        let k = format!("{}{:>02}", ch, n);
        vars.insert(k, Some((val & 1) as u8));
        val >>= 1;
    }
}

fn run_pc(
    vars: &HashMap<String, Option<u8>>,
    ops: &Vec<Op>,
) -> Option<HashMap<String, Option<u8>>> {
    let mut variables = vars.clone();
    let mut operations = ops.clone();
    let mut solved_z_vars: HashSet<String> = HashSet::new();
    let unsolved_z = vars
        .iter()
        .filter(|(k, v)| k.starts_with("z") && v.is_none())
        .count();

    while solved_z_vars.len() != unsolved_z {
        let mut solved_something = false;
        for i in (0..operations.len()).rev() {
            let op = &operations[i];
            let v0 = variables.get(&op.var1).unwrap();
            let v1 = variables.get(&op.var2).unwrap();
            if v0.is_none() || v1.is_none() {
                continue;
            }

            solved_something = true;
            let out = INSTRUCTIONS[op.gate_fn as usize](v0.unwrap(), v1.unwrap());
            variables.get_mut(&op.out).map(|val| {
                *val = Some(out);
            });
            if op.out.starts_with("z") {
                solved_z_vars.insert(op.out.clone());
            }
            operations.swap_remove(i);
        }
        if !solved_something {
            return None;
        }
    }

    return Some(variables);
}

fn run_full_pc(vars: &HashMap<String, Option<u8>>, ops: &Vec<Op>) -> u64 {
    let sol = run_pc(&vars, &ops).unwrap();
    let z_len = sol.keys().filter(|k| k.starts_with("z")).count();
    return gen_sol('z', z_len, &sol);
}

const MAX_SWAPS: usize = 4;

fn swap_fields(vec: &mut Vec<Op>, index1: usize, index2: usize) {
    if index1 == index2 {
        panic!("shouldn't happen");
    }
    let (left, right) = if index1 < index2 {
        vec.split_at_mut(index2)
    } else {
        vec.split_at_mut(index1)
    };

    let (a, b) = if index1 < index2 {
        (&mut left[index1].out, &mut right[0].out)
    } else {
        (&mut right[0].out, &mut left[index2].out)
    };

    std::mem::swap(a, b);
}

fn descend_pc(
    vars: &mut HashMap<String, Option<u8>>,
    ops: &mut Vec<Op>,
    pos: usize,
    swap_count: usize,
    okay_swaps: &mut HashSet<usize>,
    var_len: usize,
) -> Option<Vec<String>> {
    let mut rng = rand::thread_rng();
    const MAX_BITS: usize = 49;
    if pos == 0 {
        encode_var('y', 1, var_len, vars);
    }
    if pos > var_len {
        println!("here!!");
        return Some(Vec::new());
    };

    let test_x: u64 = 2_u64.pow(pos as u32) - 1;
    encode_var('x', test_x, var_len, vars);

    let init_vars = run_pc(&vars, &ops).unwrap();
    let init_sol = gen_sol('z', var_len + 1, &init_vars);
    let target = test_x + 1;
    let bad_z = init_sol ^ target;

    println!(
        "\nx ={:#0width$b}, y={:#0width$b}, i = {}",
        test_x,
        1,
        pos,
        width = MAX_BITS
    );
    println!("z0={:#0width$b}", init_sol, width = MAX_BITS);
    println!("zt={:#0width$b}", target, width = MAX_BITS);
    println!("z^={:#0width$b}", bad_z, width = MAX_BITS);

    if bad_z == 0 {
        return descend_pc(vars, ops, pos + 1, swap_count, okay_swaps, var_len);
    }

    if swap_count + 1 > MAX_SWAPS {
        return None;
    }

    for swap in okay_swaps.iter().combinations(2) {
        let (a, b) = (*swap[0], *swap[1]);

        swap_fields(ops, a, b);

        let solved_vars = run_pc(&vars, &ops);
        if solved_vars.is_none() {
            swap_fields(ops, a, b);
            continue;
        }

        let zs_sol = gen_sol('z', var_len + 1, &solved_vars.unwrap());
        if zs_sol != target {
            swap_fields(ops, a, b);
            continue;
        }

        let mut new_swaps = okay_swaps
            .iter()
            .filter(|i| **i != a && **i != b)
            .cloned()
            .collect();
        let v = descend_pc(vars, ops, pos + 1, swap_count + 1, &mut new_swaps, var_len);
        encode_var('x', test_x, var_len, vars);
        if v.is_none() {
            swap_fields(ops, a, b);
            continue;
        }

        let tx = rng.gen::<u64>() & test_x;
        let ty = rng.gen::<u64>() & test_x;
        encode_var('x', tx, var_len, vars);
        encode_var('y', ty, var_len, vars);
        let z_target = tx + ty;
        let verify_sol = run_pc(vars, ops).unwrap();
        if z_target != gen_sol('z', var_len + 1, &verify_sol) {
            encode_var('x', test_x, var_len, vars);
            encode_var('y', 1, var_len, vars);
            swap_fields(ops, a, b);
            continue;
        }

        let mut sol_vec = vec![ops[a].out.clone(), ops[b].out.clone()];
        sol_vec.extend(v.unwrap());
        return Some(sol_vec);
    }

    return None;
}

fn main() {
    let file = File::open("input24.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut variables: HashMap<String, Option<u8>> = HashMap::new();
    let mut operations: Vec<Op> = Vec::new();

    let mut handle_op_input = false;
    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        if line.is_empty() {
            handle_op_input = true;
            continue;
        }

        if handle_op_input {
            // Form is x00 AND y00 -> z00
            let op_str: Vec<_> = line.split_whitespace().collect();

            let gate_fn;
            match op_str[1] {
                "OR" => {
                    gate_fn = 0;
                }
                "AND" => {
                    gate_fn = 1;
                }
                "XOR" => {
                    gate_fn = 2;
                }
                _ => panic!("bad input"),
            }
            let op = Op {
                gate_fn: gate_fn,
                var1: op_str[0].to_string(),
                var2: op_str[2].to_string(),
                out: op_str[4].to_string(),
            };

            variables.entry(op.var1.clone()).or_insert(None);
            variables.entry(op.var2.clone()).or_insert(None);
            variables.entry(op.out.clone()).or_insert(None);
            operations.push(op);
            continue;
        }

        // Form: x00: 1
        let var_str: Vec<_> = line.split(":").collect();
        variables
            .entry(var_str[0].to_string())
            .or_insert(Some(var_str[1].trim().parse::<u8>().unwrap()));
    }

    println!("variables={}", variables.len());
    println!("operations={}", operations.len());

    let mut x_count: usize = 0;
    let mut y_count: usize = 0;
    let mut z_count: usize = 0;
    for v in variables.keys() {
        if v.starts_with("z") {
            z_count += 1;
        } else if v.starts_with("x") {
            x_count += 1;
        } else if v.starts_with("y") {
            y_count += 1;
        }
    }

    println!(
        "x_count={}, y_count={}, z_count={}",
        x_count, y_count, z_count
    );

    let z0_sol = run_full_pc(&variables, &operations);
    println!("initial solve={}", z0_sol);

    let mut okay_swaps: HashSet<usize> = (0..operations.len()).collect();
    let mut mod_var = variables.clone();

    let mut mod_op = operations.clone();
    let sol = descend_pc(&mut mod_var, &mut mod_op, 0, 0, &mut okay_swaps, x_count);
    if sol.is_some() {
        let mut sol = sol.unwrap();
        println!("sol={}", sol.join(","));
        sol.sort();
        println!("sol_sorted={}", sol.join(","));
    }
}
