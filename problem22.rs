use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
struct Seq {
    delta: i8,
    cost: u8,
}

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
struct Seq4 {
    a: Seq,
    b: Seq,
    c: Seq,
    d: Seq,
}

type Seq4Key = (i8, i8, i8, i8);
impl Seq4 {
    fn rotate(&mut self) {
        std::mem::swap(&mut self.c, &mut self.d);
        std::mem::swap(&mut self.b, &mut self.d);
        std::mem::swap(&mut self.a, &mut self.d);
    }

    fn key(&self) -> Seq4Key {
        (self.a.delta, self.b.delta, self.c.delta, self.d.delta)
    }
}

fn next_secret(mut secret: usize) -> usize {
    secret = (secret ^ (secret << 6)) & 0xFFFFFF;
    secret = (secret ^ (secret >> 5)) & 0xFFFFFF;
    secret = (secret ^ (secret << 11)) & 0xFFFFFF;
    return secret;
}

fn main() {
    let file = File::open("input22.txt").expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut secret_nums = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        secret_nums.push(line.parse::<usize>().expect("fail"));
    }

    let mut overall_costs: HashMap<Seq4Key, usize> = HashMap::new();
    for i in 0..secret_nums.len() {
        let mut current_costs: HashMap<Seq4Key, usize> = HashMap::new();
        let mut current_seq: Seq4 = Seq4::default();
        for j in 0..2000 {
            let old_price = (secret_nums[i] % 10) as u8;
            secret_nums[i] = next_secret(secret_nums[i]);
            let new_price = (secret_nums[i] % 10) as u8;
            let delta = new_price as i8 - old_price as i8;
            if j < 3 {
                match j {
                    0 => {
                        current_seq.a.delta = delta as i8;
                        current_seq.a.cost = new_price;
                    }
                    1 => {
                        current_seq.b.delta = delta as i8;
                        current_seq.b.cost = new_price;
                    }
                    2 => {
                        current_seq.c.delta = delta as i8;
                        current_seq.c.cost = new_price;
                    }
                    _ => panic!("bad index"),
                }
                continue;
            }
            if j > 3 {
                current_seq.rotate();
            }
            current_seq.d.delta = delta as i8;
            current_seq.d.cost = new_price;

            current_costs
                .entry(current_seq.key())
                .or_insert(current_seq.d.cost as usize);
        }
        for (k, cur_value) in current_costs {
            overall_costs
                .entry(k)
                .and_modify(|v| *v += cur_value)
                .or_insert(cur_value);
        }
    }

    let best = overall_costs.iter().max_by_key(|(_, &v)| v);
    println!("best={:?}", best.unwrap());

    let mut sum = 0;
    for secret in secret_nums {
        sum += secret;
    }

    println!("sum={}", sum);
}
