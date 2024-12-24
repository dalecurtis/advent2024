extern crate itertools;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

type LanParty = HashMap<String, HashSet<String>>;

// This is buggy for finding all chains, but works for finding the max chain.
// Too lazy to fix the bugs so that it works for part 1.
fn chain(pc: &String, current_party: &mut HashSet<String>, lan_party: &LanParty) {
    let links = lan_party.get(pc);
    if links.is_none() {
        return;
    }

    for link in links.unwrap() {
        if current_party.contains(link) {
            continue;
        }

        let target_links = lan_party.get(link).unwrap();

        let mut all_match = true;
        for current_pc in current_party.iter() {
            if !target_links.contains(current_pc) {
                all_match = false;
                break;
            }
        }
        if !all_match {
            continue;
        }

        current_party.insert(link.clone());
        chain(link, current_party, lan_party);
    }
}

fn main() {
    let file = File::open("input23.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut lan_party: LanParty = LanParty::new();
    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let nodes: Vec<_> = line.split('-').map(|s| s.to_string()).collect();

        lan_party
            .entry(nodes[0].clone())
            .or_insert(HashSet::new())
            .insert(nodes[1].clone());
        lan_party
            .entry(nodes[1].clone())
            .or_insert(HashSet::new())
            .insert(nodes[0].clone());
    }

    println!("total pcs: {}", lan_party.keys().len());
    let mut t_count = 0;
    for combo in lan_party.keys().combinations(3) {
        if !combo.iter().any(|s| s.starts_with("t")) {
            continue;
        }

        let direct_connections0 = lan_party.get(combo[0]).unwrap();
        let direct_connections1 = lan_party.get(combo[1]).unwrap();
        let direct_connections2 = lan_party.get(combo[2]).unwrap();

        if direct_connections0.contains(combo[1])
            && direct_connections0.contains(combo[2])
            && direct_connections1.contains(combo[0])
            && direct_connections1.contains(combo[2])
            && direct_connections2.contains(combo[0])
            && direct_connections2.contains(combo[1])
        {
            t_count += 1;
        }
    }

    println!("t_count={}", t_count);

    let mut max_link: (usize, Vec<String>) = (0, Vec::new());
    let mut chains: HashSet<Vec<String>> = HashSet::new();
    for key in lan_party.keys().sorted() {
        let mut key_chain = HashSet::new();
        key_chain.insert(key.clone());
        chain(&key, &mut key_chain, &lan_party);

        if key_chain.len() > 1 {
            let c: Vec<_> = key_chain.iter().cloned().sorted().collect();
            if c.len() > max_link.0 {
                max_link = (c.len(), c.clone());
            }
            chains.insert(c);
        }
    }

    println!("max_link={}", max_link.1.join(","));
}
