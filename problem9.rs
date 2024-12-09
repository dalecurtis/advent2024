const FREESPACE_ID: i64 = 59192512512;
const RESERVED_ID: i64 = 5812851258581;

#[derive(Debug, Default, Clone, Eq, Hash, PartialEq)]
struct Record {
    id: i64,
    len: u32,
    max_len: u32,
}

fn main() {
    let contents =
        std::fs::read_to_string("input9.txt").expect("Should have been able to read the file");
    // let contents = "2333133121414131402";
    // let contents = "90909";

    let mut next_id: i64 = 0;
    let mut input: Vec<Record> = Vec::new();
    let mut next_is_freespace = false;

    let mut next_freespace_index: i32 = -1;
    let mut last_data_index: usize = 0;

    for c in contents.chars() {
        let v = c.to_digit(10).expect("fail");
        if next_is_freespace && v == 0 {
            next_is_freespace = false;
            continue;
        } else if next_is_freespace {
            if next_freespace_index < 0 {
                next_freespace_index = input.len() as i32;
            }
            // LinkedList sucks in rust, so reserve space for packing.
            input.push(Record {
                id: FREESPACE_ID,
                len: v,
                max_len: v,
            });
            for _ in 1..v {
                input.push(Record {
                    id: RESERVED_ID,
                    len: 1,
                    max_len: 1,
                });
            }
        } else {
            last_data_index = input.len();
            input.push(Record {
                id: next_id,
                len: v,
                max_len: v,
            });
            next_id += 1;
        }
        next_is_freespace = !next_is_freespace;
    }

    // println!(
    //     "next_freespace_index={}, last_data_index={}",
    //     next_freespace_index, last_data_index
    // );
    // for rec in &input {
    //     for _ in 0..rec.len {
    //         if rec.id == FREESPACE_ID {
    //             print!(".");
    //         } else if rec.id != RESERVED_ID {
    //             print!("{}", rec.id);
    //         }
    //     }
    // }
    // println!();

    while next_freespace_index < last_data_index as i32 {
        if next_freespace_index < 0 {
            break;
        }   
        let (free_space, data_space) = input.split_at_mut(last_data_index);
        let free_rec = &mut free_space[next_freespace_index as usize];
        let data_rec = &mut data_space[0];
        if free_rec.id != FREESPACE_ID {
            panic!("wtf_should_be_free");
        }
        if data_rec.id == FREESPACE_ID || data_rec.id == RESERVED_ID {
            panic!("wtf_should_be_data");
        }

        free_rec.id = data_rec.id;
        free_rec.len = std::cmp::min(free_rec.len, data_rec.len);
        data_rec.len -= free_rec.len;
        let free_rec_remaining = free_rec.max_len - free_rec.len;
        if data_rec.len == 0 {
            data_rec.id = FREESPACE_ID;
            loop {
                last_data_index -= 1;
                if last_data_index < next_freespace_index as usize {
                    break;
                }
                if free_space[last_data_index].id != FREESPACE_ID && free_space[last_data_index].id != RESERVED_ID {
                    break;
                }
            }
        }
        if last_data_index < next_freespace_index as usize {
            break;
        }
        if free_rec_remaining == 0 {
            loop {
                next_freespace_index += 1;
                if input[next_freespace_index as usize].id == FREESPACE_ID {
                    break;
                }
                if next_freespace_index as usize > last_data_index {
                    break;
                }
            }
        } else if free_rec_remaining > 0 {
            next_freespace_index += 1;
            let reserved_rec = &mut input[next_freespace_index as usize];
            if reserved_rec.id != RESERVED_ID {
                panic!("wtf_reserved")
            }
            reserved_rec.id = FREESPACE_ID;
            reserved_rec.len = free_rec_remaining;
            reserved_rec.max_len = free_rec_remaining;
        }
    }

    // for rec in &input {
    //     for _ in 0..rec.len {
    //         if rec.id == FREESPACE_ID {
    //             print!(".");
    //         } else if rec.id != RESERVED_ID {
    //             print!("{}", rec.id);
    //         }
    //     }
    // }
    // println!();

    let mut pos: i64 = 0;
    let mut checksum: i64 = 0;
    for rec in &input {
        for _ in 0..rec.len {
            if rec.id == FREESPACE_ID || rec.id == RESERVED_ID {
                break;
            }
            checksum += rec.id * pos;
            pos += 1;
        }
    }
    
    println!("checksum={}", checksum);
}
