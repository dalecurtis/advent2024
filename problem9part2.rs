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
    let mut free_space: Vec<Record> = Vec::new();
    let mut next_is_freespace = false;
    let mut last_data_index: usize = 0;

    // Turn data stream into `input` and `free_space`. Would probably be better
    // to use some log(n) indexable+reorderable container for free space.
    //
    // Rust LinkedList is apparently only working in nightly... so each free
    // space entry in `input` is represented as one FREESPACE_ID entry with
    // followed by n - 1 RESERVED_ID entries that we can fill in later (to
    // avoid vector resizing).
    for c in contents.chars() {
        let v = c.to_digit(10).expect("fail");
        if next_is_freespace && v == 0 {
            next_is_freespace = false;
            continue;
        } else if next_is_freespace {
            free_space.push(Record {
                id: input.len() as i64,
                len: v,
                max_len: v,
            });
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

    // println!("last_data_index={}", last_data_index);
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

    let mut max_rec_id: i64 = last_data_index as i64 + 1;

    // Find next block of data that is not free or reserved and hasn't already been moved.
    let find_next_data_idx =
        |cur_input: &[Record], mut cur_data_index: usize, max_id: i64| -> usize {
            loop {
                if cur_data_index == 0 {
                    break;
                }
                cur_data_index -= 1;
                let cur_data_id = cur_input[cur_data_index].id;
                if cur_data_id != FREESPACE_ID && cur_data_id != RESERVED_ID && cur_data_id < max_id
                {
                    break;
                }
            }
            return cur_data_index;
        };

    while last_data_index != 0 {
        let (free, data_space) = input.split_at_mut(last_data_index);
        let data_rec = &mut data_space[0];
        if data_rec.id == FREESPACE_ID || data_rec.id == RESERVED_ID {
            panic!("wtf_should_be_data: {}", last_data_index);
        }

        // Rust was super annoying here and refused to allow `free_rec_idx`
        // and `free_rec` to not have an initial value no matter how hard
        // I tried to assert free_space.len() > 0.
        let mut free_rec_idx: usize = 0;
        let mut free_rec: &mut Record = &mut free[0];
        let mut found = false;
        for i in 0..free_space.len() {
            let fr = &mut free_space[i];
            if fr.len >= data_rec.len && fr.id < last_data_index as i64 {
                found = true;
                free_rec_idx = fr.id as usize;
                free_rec = &mut free[free_rec_idx];
                fr.len -= data_rec.len;
                fr.id += 1;
                break;
            }
            if fr.id as usize > last_data_index {
                free_rec_idx = 0;
                max_rec_id = last_data_index as i64;
                break;
            }
        }

        if !found {
            last_data_index = find_next_data_idx(free, last_data_index, max_rec_id);
            continue;
        }

        if free_rec.id != FREESPACE_ID {
            panic!("wtf_should_not_be_free");
        }
        if free_rec.len < data_rec.len {
            panic!("wtf_search_failed");
        }

        free_rec.id = data_rec.id;
        free_rec.len = data_rec.len;
        let free_rec_remaining = free_rec.max_len - free_rec.len;
        if free_rec_remaining > 0 {
            free_rec_idx += 1;
            let reserved_rec = &mut free[free_rec_idx];
            if reserved_rec.id != RESERVED_ID {
                panic!("wtf_reserved")
            }
            reserved_rec.id = FREESPACE_ID;
            reserved_rec.len = free_rec_remaining;
            reserved_rec.max_len = free_rec_remaining;
        }

        max_rec_id = data_rec.id;
        data_rec.id = FREESPACE_ID;
        last_data_index = find_next_data_idx(free, last_data_index, max_rec_id);
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

    // Compute positional checksum for data.s
    let mut pos: i64 = 0;
    let mut checksum: i64 = 0;
    for rec in &input {
        for _ in 0..rec.len {
            if rec.id == FREESPACE_ID {
                pos += 1;
            }
            if rec.id != FREESPACE_ID && rec.id != RESERVED_ID {
                checksum += rec.id * pos;
                pos += 1;
            }
        }
    }

    println!("checksum={}", checksum);
}
