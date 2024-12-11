use std::collections::HashMap;

// Turns 1234 -> 12, 34, etc.
fn split_number(v: u64, num_digits: u32) -> (u64, u64) {
    let divisor = 10_u64.pow(num_digits / 2) as u64;
    let first_part: u64 = v / divisor;
    let second_part: u64 = v % divisor;
    return (first_part, second_part);
}

// Splits stones according to the rules. Sticks result in `cache`
fn split_stone(v: u64, d: u8, max_d: u8, mut cache: &mut HashMap<(u8, u64), u64>) -> u64 {
    if d >= max_d {
        return 1;
    }

    // Can't hold a cache entry while we pass it on below, so a scope block is used.
    {
        let vc = cache.get(&(d, v));
        if !vc.is_none() {
            return *vc.unwrap();
        }
    }

    let result: u64;
    if v == 0 {
        result = split_stone(1, d + 1, max_d, &mut cache);
    } else {
        let num_digits = v.ilog10() + 1;
        if num_digits & 1 == 0 {
            let (first_four, last_four) = split_number(v, num_digits);
            result = split_stone(first_four, d + 1, max_d, &mut cache)
                + split_stone(last_four, d + 1, max_d, &mut cache);
        } else {
            result = split_stone(v * 2024, d + 1, max_d, &mut cache);
        }
    }

    cache.insert((d, v), result);
    return result;
}

fn main() {
    const BLINKS: u8 = 75; // 25 for part 1.

    // let stones: Vec<u64> = vec![125, 17];
    let stones: Vec<u64> = vec![5688, 62084, 2, 3248809, 179, 79, 0, 172169];

    for v in &stones {
        print!("{} ", v)
    }
    println!();

    let mut stone_count = 0;
    let mut cache = HashMap::new();
    for v in stones {
        stone_count += split_stone(v, 0, BLINKS, &mut cache);
        println!("partial_count={}", stone_count);
    }
    println!("stones={}, cache_size={}", stone_count, cache.len());
}
